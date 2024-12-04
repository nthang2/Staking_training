use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cw20::{Cw20ReceiveMsg, Expiration};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub och_token: String,
    pub usdc_token: String,
    pub reward_per_second: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AdminSetRewardPerSecond { 
        new_reward_per_second: Uint128 
    },
    UserSendStake {
        amount_stake: Uint128
    },
    Receive (Cw20ReceiveMsg),
    UserWithdrawStake {
        amount_withdraw: Uint128
    },
    UserClaimReward {
    }

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    RewardPerSecond {},
    TotalStaked {},
    UserStaked {
        user_address: Addr
    },
    UserRewards {
        user_address: Addr
    }
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RewardPerSecondResponse {
    pub reward_per_second: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TotalStakedResponse {
    pub total_staked: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]

pub struct UserStakedResponse {
    pub user_staked: Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserRewardsResponse {
    pub user_rewards: Uint128
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MigrateMsg {}