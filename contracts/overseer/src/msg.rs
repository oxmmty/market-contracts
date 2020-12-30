use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::HumanAddr;
use moneymarket::TokensHuman;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InitMsg {
    /// Initial owner address
    pub owner_addr: HumanAddr,
    /// Oracle contract address for collateral tokens
    pub oracle_contract: HumanAddr,
    /// Market contract address to receive missing interest buffer
    pub market_contract: HumanAddr,
    /// Liquidation model contract address to compute liqudation amount
    pub liquidation_contract: HumanAddr,
    /// The base denomination used when fetching oracle price,
    /// reward distribution, and borrow
    pub stable_denom: String,
    /// # of blocks per epoch period
    pub epoch_period: u64,
    /// Distribute interest buffer to market contract,
    /// when deposit_rate < distribution_threshold
    pub distribution_threshold: Decimal256,
    /// Target deposit rate.
    /// When current deposit rate is bigger than this,
    /// Custody contracts send rewards to interest buffer
    pub target_deposit_rate: Decimal256,
    /// Ratio to be distributed from the interest buffer
    pub buffer_distribution_rate: Decimal256,
    /// Valid oracle price timeframe
    pub price_timeframe: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    ////////////////////
    /// Owner operations
    ////////////////////

    /// Update Configs
    UpdateConfig {
        owner_addr: Option<HumanAddr>,
        oracle_contract: Option<HumanAddr>,
        liquidation_contract: Option<HumanAddr>,
        distribution_threshold: Option<Decimal256>,
        target_deposit_rate: Option<Decimal256>,
        buffer_distribution_rate: Option<Decimal256>,
        epoch_period: Option<u64>,
        price_timeframe: Option<u64>,
    },

    /// Create new custody contract for the given collateral token
    Whitelist {
        collateral_token: HumanAddr, // bAsset token contract
        custody_contract: HumanAddr, // bAsset custody contract
        ltv: Decimal256,             // Loan To Value ratio
    },
    /// Update registered whitelist info
    UpdateWhitelist {
        collateral_token: HumanAddr,         // bAsset token contract
        custody_contract: Option<HumanAddr>, // bAsset custody contract
        ltv: Option<Decimal256>,             // Loan To Value ratio
    },

    /// Claims all staking rewards from the bAsset contracts
    /// and also do a epoch basis updates
    /// 1. Distribute interest buffers to depositors
    /// 2. Invoke [Custody] DistributeRewards
    ExecuteEpochOperations {},

    ////////////////////
    /// User operations
    ////////////////////
    LockCollateral {
        collaterals: TokensHuman, // <(Collateral Token, Amount)>
    },
    UnlockCollateral {
        collaterals: TokensHuman, // <(Collateral Token, Amount)>
    },

    /////////////////////////////
    /// Permissionless operations
    /////////////////////////////
    LiquidateCollateral { borrower: HumanAddr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    EpochState {},
    Whitelist {
        collateral_token: Option<HumanAddr>,
        start_after: Option<HumanAddr>,
        limit: Option<u32>,
    },
    Collaterals {
        borrower: HumanAddr,
    },
    AllCollaterals {
        start_after: Option<HumanAddr>,
        limit: Option<u32>,
    },
    DistributionParams {},
    BorrowLimit {
        borrower: HumanAddr,
        block_time: Option<u64>,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner_addr: HumanAddr,
    pub oracle_contract: HumanAddr,
    pub market_contract: HumanAddr,
    pub liquidation_contract: HumanAddr,
    pub distribution_threshold: Decimal256,
    pub target_deposit_rate: Decimal256,
    pub buffer_distribution_rate: Decimal256,
    pub stable_denom: String,
    pub epoch_period: u64,
    pub price_timeframe: u64,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WhitelistResponseElem {
    pub ltv: Decimal256,
    pub custody_contract: HumanAddr,
    pub collateral_token: HumanAddr,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WhitelistResponse {
    pub elems: Vec<WhitelistResponseElem>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralsResponse {
    pub borrower: HumanAddr,
    pub collaterals: TokensHuman, // <(Collateral Token, Amount)>
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllCollateralsResponse {
    pub all_collaterals: Vec<CollateralsResponse>,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DistributionParamsResponse {
    pub deposit_rate: Decimal256,
    pub target_deposit_rate: Decimal256,
    pub distribution_threshold: Decimal256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BorrowLimitResponse {
    pub borrower: HumanAddr,
    pub borrow_limit: Uint256,
}
