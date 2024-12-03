use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub och_token: Addr,
    pub usdc_token: Addr,
    pub contract_addr: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RewardPerSecond {
    pub reward_per_second: Uint128,
}   //update when admin change the reward per second

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct TotalStaked {
    pub total_staked: Uint128,
}   //update when user stake or unstake

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct UserStaked {
    pub number_staked: Uint128,
    pub last_time_update: Timestamp,
    pub last_exr_when_interact: Decimal,
}   //update when user stake or unstake

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct UserReward {
    pub number_reward: Uint128,   
    pub last_time_update: Timestamp,
    pub exr_last_update: Decimal,
}   
    //user_reward = user_staked * (exr_cur - exr_last_update)

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ExchangeRate {
    pub exchange_rate: Decimal,
    pub last_update_time: Timestamp,
}
pub const CONFIG: Item<Config> = Item::new("config");
pub const REWARD_PER_SECOND: Item<RewardPerSecond> = Item::new("reward_per_second");
pub const TOTAL_STAKED: Item<TotalStaked> = Item::new("total_staked");
pub const USER_STAKED: Map<Addr, UserStaked> = Map::new("user_staked");
pub const USER_REWARD: Map<Addr, UserReward> = Map::new("user_reward");
pub const EXCHANGE_RATE: Item<ExchangeRate> = Item::new("exchange_rate");
