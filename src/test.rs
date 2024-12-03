// use cosmwasm_std::testing::mock_dependencies;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{attr, coins, from_json, Addr, Uint128};
use crate::contract::{instantiate, execute};
use crate::msg::{InstantiateMsg, ExecuteMsg};
use crate::state::{Config, RewardPerSecond, TotalStaked, UserStaked, UserReward, ExchangeRate, CONFIG, REWARD_PER_SECOND, TOTAL_STAKED, USER_STAKED, USER_REWARD, EXCHANGE_RATE};

pub const ADDR1: &str = "orai1etpatp0s4899uxda844q8074q29f7w664924h3";
    pub const ADDR2: &str = "orai1tyl0wdg7shhv9m44kq34za5gqanu7uwfz2xvrc";
    pub const OCH_TOKEN: &str = "orai1hn8w33cqvysun2aujk5sv33tku4pgcxhhnsxmvnkfvdxagcx0p8qa4l98q";
    pub const USDC_TOKEN: &str = "orai15un8msx3n5zf9ahlxmfeqd2kwa5wm0nrpxer304m9nd5q6qq0g6sku5pdd";
    

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let time = env.block.time.to_string();
        let info = mock_info(ADDR2, &vec![]);

        let reward_per_second = Uint128::new(1);
        let msg = InstantiateMsg { admin: Some(String::from(ADDR1)), och_token: OCH_TOKEN.to_string(), usdc_token: USDC_TOKEN.to_string(), reward_per_second: reward_per_second };  

          
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), 
                attr("admin", ADDR1),
                attr("och_token", OCH_TOKEN),
                attr("usdc_token", USDC_TOKEN),
                attr("exchange_rate", "0"),
                attr("last_update_time", time),
                attr("reward per second", "1"),
                attr("total staked", "0")
                ]
        )
    }

    #[test]
    fn test_admin_set_reward_per_second() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        let msg = InstantiateMsg { admin: Some(String::from(ADDR1)), och_token: OCH_TOKEN.to_string(), usdc_token: USDC_TOKEN.to_string(), reward_per_second: Uint128::from(1u128) };    
        let _ = instantiate(deps.as_mut(), env, info, msg).unwrap();

        TOTAL_STAKED.save(deps.as_mut().storage, &TotalStaked { total_staked: Uint128::from(100u64) }).unwrap();
        
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        let new_reward_per_second = Uint128::from(100u64);
        let msg = ExecuteMsg::AdminSetRewardPerSecond { new_reward_per_second };
        let res = execute(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("method", "execute_admin_set_reward_per_second"), 
                attr("new_reward_per_second", "100"),
                attr("update exchange rate", "0"),
                ]
        );

        println!("last_update_time_ex_rate: {:?}", EXCHANGE_RATE.load(deps.as_ref().storage).unwrap().last_update_time.seconds());
    }

    #[test]
    fn test_admin_set_reward_per_second_with_other_total_staked() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let reward_per_second = Uint128::from(1u64);
        let msg = InstantiateMsg { admin: Some(String::from(ADDR1)), och_token: OCH_TOKEN.to_string(), usdc_token: USDC_TOKEN.to_string(), reward_per_second: reward_per_second };      
        let _ = instantiate(deps.as_mut(), env, info, msg, ).unwrap();

        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        let new_reward_per_second = Uint128::from(2u64);

        let msg = ExecuteMsg::AdminSetRewardPerSecond { new_reward_per_second };
        let res = execute(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("method", "execute_admin_set_reward_per_second"), 
                attr("new_reward_per_second", "2"),
                attr("update exchange rate", "0"),
                ]
        )
    }    