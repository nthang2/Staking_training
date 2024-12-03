use std::ops::Add;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Addr, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Uint64, WasmMsg};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, Cw20ReceiveMsg};
use cw20_base::allowances::{execute_decrease_allowance, execute_increase_allowance};



use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, RewardPerSecondResponse, TotalStakedResponse, UserRewardsResponse, UserStakedResponse};
use crate::state::{Config, ExchangeRate, RewardPerSecond, TotalStaked, UserReward, UserStaked, CONFIG, EXCHANGE_RATE, REWARD_PER_SECOND, TOTAL_STAKED, USER_REWARD, USER_STAKED};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const DECIMAL: Uint128 = Uint128::new(1_000_000);


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
    
) -> Result<Response, ContractError> {
    
    let admin = msg.admin.unwrap_or(info.sender.to_string());
    
    let validated_admin = deps.api.addr_validate(&admin.to_string())?;
    
    let och_token = deps.api.addr_validate(&msg.och_token)?;
    println!("Och Token: {}", och_token);
    let usdc_token = deps.api.addr_validate(&msg.usdc_token.to_string())?;
    let config = Config {
        admin: validated_admin.clone(),
        och_token: och_token.clone(),
        usdc_token: usdc_token.clone(),
        contract_addr: env.contract.address.to_string(),
    };

    let reward_per_second = msg.reward_per_second;

    let start_exchange_rate = ExchangeRate {
        exchange_rate: Decimal::new(Uint128::from(0u128)),
        last_update_time: env.block.time,
    };

    let initial_reward_per_second = RewardPerSecond {
        reward_per_second: reward_per_second,
    };

    let total_staked = TotalStaked {
        total_staked: Uint128::from(0u128),
    };

    CONFIG.save(deps.storage, &config)?;
    EXCHANGE_RATE.save(deps.storage, &start_exchange_rate)?;
    REWARD_PER_SECOND.save(deps.storage, &initial_reward_per_second)?;
    TOTAL_STAKED.save(deps.storage, &total_staked)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string())
        .add_attribute("och_token", och_token.to_string())
        .add_attribute("usdc_token", usdc_token.to_string())
        .add_attribute("exchange_rate", start_exchange_rate.exchange_rate.to_string())
        .add_attribute("last_update_time", start_exchange_rate.last_update_time.to_string())
        .add_attribute("reward per second", initial_reward_per_second.reward_per_second.to_string())
        .add_attribute("total staked", total_staked.total_staked.to_string())
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let och_token = config.och_token.to_string();
    match msg {
        ExecuteMsg::AdminSetRewardPerSecond { new_reward_per_second } => execute_admin_set_reward_per_second(deps, env, info, new_reward_per_second),
        ExecuteMsg::UserSendStake { amount_stake} => execute_user_send_stake(deps, env, info, amount_stake),
        ExecuteMsg::UserWithdrawStake { amount_withdraw } => execute_user_withdraw_stake(deps, env, info, amount_withdraw),
        ExecuteMsg::UserClaimReward {} => execute_user_claim_reward(deps, env, info),
    }
}

fn execute_admin_set_reward_per_second(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_reward_per_second: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let reward_per_second = RewardPerSecond {
        reward_per_second: new_reward_per_second,
    };

    REWARD_PER_SECOND.save(deps.storage, &reward_per_second)?;

    let old_exchange_rate = EXCHANGE_RATE.load(deps.storage)?.exchange_rate;
    let last_update_time = EXCHANGE_RATE.load(deps.storage)?.last_update_time;
    let delta_time = Uint128::from(env.block.time.seconds() - last_update_time.seconds());
    let total_staked = TOTAL_STAKED.load(deps.storage)?.total_staked;

    if total_staked == Uint128::from(0u128) {
        EXCHANGE_RATE.save(deps.storage, &ExchangeRate {
            exchange_rate: old_exchange_rate,
            last_update_time: env.block.time,
        })?;
        return Ok(Response::new()
            .add_attribute("method", "execute_admin_set_reward_per_second")
            .add_attribute("new_reward_per_second", reward_per_second.reward_per_second.to_string())
            .add_attribute("update exchange rate", old_exchange_rate.to_string())
        )
    }
    let exchange_rate_update = ExchangeRate {
        exchange_rate: old_exchange_rate + Decimal::from_ratio((new_reward_per_second * delta_time * DECIMAL), total_staked),
        last_update_time: env.block.time,
    };

    EXCHANGE_RATE.save(deps.storage, &exchange_rate_update)?;
    
    Ok(Response::new()
        .add_attribute("method", "execute_admin_set_reward_per_second")
        .add_attribute("new_reward_per_second", reward_per_second.reward_per_second.to_string())
        .add_attribute("update exchange rate", exchange_rate_update.exchange_rate.to_string())
    )
}

fn execute_user_send_stake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount_stake: Uint128,
) -> Result<Response, ContractError> {
    let user_address = info.sender.clone();
    let user_address_clone_1 = info.sender.clone();
    let user_address_clone_2 = info.sender.clone();
    let user_address_clone_3 = info.sender.clone();
    let user_address_clone_4 = info.sender.clone();
    
    let last_exchange_rate = EXCHANGE_RATE.load(deps.storage)?.exchange_rate;
    let last_update_time = EXCHANGE_RATE.load(deps.storage)?.last_update_time;

    let delta_time = Uint128::from(env.block.time.seconds() - last_update_time.seconds());

    let total_staked = TOTAL_STAKED.load(deps.storage)?.total_staked;
    let new_total_staked = total_staked + amount_stake;

    let reward_per_second = REWARD_PER_SECOND.load(deps.storage)?.reward_per_second;

    let user_staked = USER_STAKED.load(deps.storage, user_address).unwrap_or(UserStaked {
        number_staked: Uint128::from(0u128),
        last_time_update: env.block.time,
        last_exr_when_interact: last_exchange_rate,
    });
    let number_user_staked = user_staked.number_staked;


    let new_exchange_rate = last_exchange_rate + Decimal::from_ratio((reward_per_second * delta_time*DECIMAL),new_total_staked);
    // khong can tinh current exr khi stake, chi can tinh khi withdraw hoac claim reward

    let update_user_staked = UserStaked {
        number_staked: number_user_staked + amount_stake,
        last_time_update: env.block.time,
        last_exr_when_interact: new_exchange_rate,
    };

    
    let update_total_staked = TotalStaked {
        total_staked: new_total_staked,
    };

    let update_exchange_rate = ExchangeRate {
        exchange_rate: new_exchange_rate,
        last_update_time: env.block.time,
    };

    let user_reward = USER_REWARD.load(deps.storage, user_address_clone_4)
                                            .unwrap_or(UserReward {
                                                number_reward: Uint128::from(0u128),
                                                last_time_update: env.block.time,
                                                exr_last_update: new_exchange_rate,
                                            });
    
    let new_number_reward = user_reward.number_reward + (number_user_staked * (new_exchange_rate - user_reward.exr_last_update))/DECIMAL;

    USER_STAKED.save(deps.storage, user_address_clone_1, &update_user_staked)?;
    TOTAL_STAKED.save(deps.storage, &update_total_staked)?;
    EXCHANGE_RATE.save(deps.storage, &update_exchange_rate)?;
    
    USER_REWARD.save(deps.storage, user_address_clone_2, &UserReward {
        number_reward: new_number_reward,
        last_time_update: env.block.time,
        exr_last_update: new_exchange_rate,
    })?;

    let config = CONFIG.load(deps.storage)?;
    let contract_addr = config.contract_addr;
    let contract_addr_clone = env.contract.address.to_string();

    Ok(Response::new()
            .add_attribute("action", "user_send_stake")
            .add_attribute("user staked", update_user_staked.number_staked.to_string())
            .add_attribute("last exchange rate when interact", update_user_staked.last_exr_when_interact.to_string())
            .add_attribute("total staked", update_total_staked.total_staked.to_string())    
            .add_attribute("new exchange rate", update_exchange_rate.exchange_rate.to_string())
            .add_message(WasmMsg::Execute { 
                contract_addr: config.och_token.clone().to_string(),
                msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
                    owner: user_address_clone_3.to_string(),
                    recipient: contract_addr_clone,
                    amount: amount_stake,
                })?,
                funds: vec![], })
            )       

        
}


fn execute_user_withdraw_stake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount_withdraw: Uint128,
) -> Result<Response, ContractError> {
    let user_address = info.sender.clone();
    let user_address_clone_1 = info.sender.clone();
    let user_address_clone_2 = info.sender.clone();
    let user_address_clone_3 = info.sender.clone();
    let user_address_clone_4 = info.sender.clone();

    let user_collateral = USER_STAKED.load(deps.storage, user_address_clone_1)?.number_staked;
    if amount_withdraw > user_collateral {
        return Err(ContractError::InsufficientStakedAmount {});
    }

    let new_user_collateral = user_collateral - amount_withdraw;
    let current_total_staked = TOTAL_STAKED.load(deps.storage)?.total_staked;
    let new_total_staked = current_total_staked - amount_withdraw;

    let exchange_rate = EXCHANGE_RATE.load(deps.storage)?;
    let last_exchange_rate = exchange_rate.exchange_rate;
    let last_update_time = exchange_rate.last_update_time;
    let delta_time = Uint128::from(env.block.time.seconds() - last_update_time.seconds());
    let reward_per_second = REWARD_PER_SECOND.load(deps.storage)?.reward_per_second;
    //update exchange rate
    let current_exchange_rate = last_exchange_rate + Decimal::from_ratio(reward_per_second * delta_time * DECIMAL, current_total_staked);
    let new_exchange_rate = last_exchange_rate + Decimal::from_ratio((reward_per_second * delta_time * DECIMAL), new_total_staked);

    let user_reward = USER_REWARD.load(deps.storage, user_address_clone_2)?;
    let new_number_reward = user_reward.number_reward + (user_collateral * (current_exchange_rate - user_reward.exr_last_update))/DECIMAL;

    let new_user_reward = UserReward {
        number_reward: new_number_reward,
        last_time_update: env.block.time,
        exr_last_update: new_exchange_rate,
    };


    USER_STAKED.save(deps.storage, user_address_clone_3, &UserStaked {
        number_staked: new_user_collateral,
        last_time_update: env.block.time,
        last_exr_when_interact: new_exchange_rate,
    })?;

    TOTAL_STAKED.save(deps.storage, &TotalStaked {
        total_staked: new_total_staked,
    })?;

    EXCHANGE_RATE.save(deps.storage, &ExchangeRate {
        exchange_rate: new_exchange_rate,
        last_update_time: env.block.time,
    })?;

    USER_REWARD.save(deps.storage, user_address_clone_4, &new_user_reward)?;

    let config = CONFIG.load(deps.storage)?;
    Ok(Response::new()
        .add_attribute("amount withdraw", amount_withdraw.to_string())
        .add_attribute("user staked", new_user_collateral.to_string())
        .add_attribute("last exchange rate when interact", new_user_reward.exr_last_update.to_string())
        .add_attribute("total staked", new_total_staked.to_string())
        .add_attribute("new exchange rate", new_exchange_rate.to_string())
        .add_message(WasmMsg::Execute {
            contract_addr: config.och_token.to_string(),
            msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                recipient: user_address.into_string(),
                amount: amount_withdraw,
            })?,
            funds: vec![],
        })
    )
}

fn execute_user_claim_reward(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let user_address = info.sender.clone();
    let user_address_clone_1 = info.sender.clone();
    let user_address_clone_2 = info.sender.clone();
    let user_address_clone_3 = info.sender.clone();

    let total_staked = TOTAL_STAKED.load(deps.storage)?.total_staked;

    let user_collateral = USER_STAKED.load(deps.storage, user_address_clone_1)?.number_staked;

    let exchange_rate = EXCHANGE_RATE.load(deps.storage)?;
    let last_exchange_rate = exchange_rate.exchange_rate;
    let last_update_time = exchange_rate.last_update_time;
    let delta_time = Uint128::from(env.block.time.seconds() - last_update_time.seconds());
    let reward_per_second = REWARD_PER_SECOND.load(deps.storage)?.reward_per_second;
    let current_exchange_rate = last_exchange_rate + Decimal::from_ratio(reward_per_second * delta_time * DECIMAL, total_staked);

    let user_reward = USER_REWARD.load(deps.storage, user_address)?;

    let update_current_user_reward = user_reward.number_reward + (user_collateral * (current_exchange_rate - user_reward.exr_last_update))/DECIMAL;
    //then, send update_current_user_reward to user_address

    let new_user_reward = UserReward {
        number_reward: Uint128::from(0u128),
        last_time_update: env.block.time,
        exr_last_update: current_exchange_rate,
    };

    USER_REWARD.save(deps.storage, user_address_clone_2, &new_user_reward)?;
    EXCHANGE_RATE.save(deps.storage, &ExchangeRate {
        exchange_rate: current_exchange_rate,
        last_update_time: env.block.time,
    })?;

    let config = CONFIG.load(deps.storage)?;
    Ok(Response::new().add_attribute("action", "user_claim_reward")
        .add_attribute("user reward", update_current_user_reward.to_string())
        .add_message(WasmMsg::Execute {
            contract_addr: config.usdc_token.to_string(),
            msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                recipient: user_address_clone_3.into_string(),
                amount: update_current_user_reward,
            })?,
            funds: vec![],
        })
    )
}




#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::RewardPerSecond {} => query_reward_per_second(deps, _env),
        QueryMsg::TotalStaked {} => query_total_staked(deps, _env),
        QueryMsg::UserStaked { user_address } => query_user_staked(deps, user_address),
        QueryMsg::UserRewards { user_address } => query_user_rewards(deps, _env, user_address)
    }
}

fn query_reward_per_second(deps: Deps, _env: Env) -> StdResult<Binary> {
    let reward_per_second = REWARD_PER_SECOND.may_load(deps.storage)?;
    to_json_binary(&RewardPerSecondResponse {
        reward_per_second: reward_per_second.unwrap().reward_per_second,
    })
}

fn query_total_staked(deps: Deps, _env: Env) -> StdResult<Binary> {
    let total_staked = TOTAL_STAKED.may_load(deps.storage)?;
    to_json_binary(&TotalStakedResponse {
        total_staked: total_staked.unwrap().total_staked,
    })
}

fn query_user_staked(deps: Deps, user_address: Addr) -> StdResult<Binary> {
    let user_staked = USER_STAKED.may_load(deps.storage, user_address)?;
    match user_staked {
        Some(user) => to_json_binary(&UserStakedResponse {
            user_staked: user.number_staked,
        }),
        None => to_json_binary(&UserStakedResponse {
            user_staked: Uint128::from(0u128),
        })
    }
}

fn query_user_rewards(deps: Deps, env: Env, user_address: Addr) -> StdResult<Binary> {
    let user_reward = USER_REWARD.load(deps.storage, user_address.clone())?;
    let user_staked = USER_STAKED.load(deps.storage, user_address.clone())?;
    let last_exchange_rate = EXCHANGE_RATE.load(deps.storage)?.exchange_rate;

    let total_staked = TOTAL_STAKED.load(deps.storage)?.total_staked;

    let last_update_time = EXCHANGE_RATE.load(deps.storage)?.last_update_time;
    let delta_time = Uint128::from(env.block.time.seconds() - last_update_time.seconds());

    let reward_per_second = REWARD_PER_SECOND.load(deps.storage)?.reward_per_second;

    let current_exchange_rate = last_exchange_rate + Decimal::from_ratio(reward_per_second * delta_time * DECIMAL, total_staked);

    let current_user_reward = user_reward.number_reward + (user_staked.number_staked * (current_exchange_rate - user_reward.exr_last_update))/DECIMAL;
    
    // match user_reward {
    //     Some(user) => to_json_binary(&UserRewardsResponse {
    //         user_rewards: user.number_reward,
    //     }),
    //     None => to_json_binary(&UserRewardsResponse {
    //         user_rewards: Uint128::from(0u128),
    //     })
    // }
    to_json_binary(&UserRewardsResponse {
        user_rewards: current_user_reward,
    })
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}


// pub mod query {
//     use super::*;

    
// }

// #[cfg(test)]
// mod tests {
    // use super::*;
    // use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    // use cosmwasm_std::{attr, coins, from_json, Addr, Uint128};

    // pub const ADDR1: &str = "orai1etpatp0s4899uxda844q8074q29f7w664924h3";
    // pub const ADDR2: &str = "orai1tyl0wdg7shhv9m44kq34za5gqanu7uwfz2xvrc";
    // pub const OCH_TOKEN: &str = "orai1hn8w33cqvysun2aujk5sv33tku4pgcxhhnsxmvnkfvdxagcx0p8qa4l98q";
    // pub const USDC_TOKEN: &str = "orai15un8msx3n5zf9ahlxmfeqd2kwa5wm0nrpxer304m9nd5q6qq0g6sku5pdd";
    

    // #[test]
    // fn test_instantiate() {
    //     let mut deps = mock_dependencies();
    //     let env = mock_env();
    //     let time = env.block.time.to_string();
    //     let info = mock_info(ADDR2, &vec![]);
    //     let msg = InstantiateMsg { admin: Some(String::from(ADDR1)), och_token: OCH_TOKEN.to_string(), usdc_token: USDC_TOKEN.to_string() };  

    //     let reward_per_second = Uint128::new(100);  
    //     let res = instantiate(deps.as_mut(), env, info, msg, reward_per_second).unwrap();

    //     assert_eq!(
    //         res.attributes,
    //         vec![attr("action", "instantiate"), 
    //             attr("admin", ADDR1),
    //             attr("och_token", OCH_TOKEN),
    //             attr("usdc_token", USDC_TOKEN),
    //             attr("exchange_rate", "0"),
    //             attr("last_update_time", time),
    //             attr("reward per second", "100"),
    //             attr("total staked", "0")
    //             ]
    //     )
    // }

    // #[test]
    // fn test_admin_set_reward_per_second() {
    //     let mut deps = mock_dependencies();
    //     let env = mock_env();
    //     let info = mock_info(ADDR1, &vec![]);
    //     let msg = InstantiateMsg { admin: Some(String::from(ADDR1)), och_token: OCH_TOKEN.to_string(), usdc_token: USDC_TOKEN.to_string() };    
    //     let _ = instantiate(deps.as_mut(), env, info, msg, Uint128::from(0u64)).unwrap();

    //     let env = mock_env();
    //     let info = mock_info(ADDR1, &vec![]);
    //     let reward_per_second = Uint64::from(100u64);
    //     let msg = ExecuteMsg::AdminSetRewardPerSecond { reward_per_second };
    //     let res = execute(deps.as_mut(), env, info, msg).unwrap();

    //     assert_eq!(
    //         res.attributes,
    //         vec![attr("method", "execute_admin_set_reward_per_second"), 
    //             attr("new_reward_per_second", "100")
    //             ]
    //     )
    // }

    
//}
