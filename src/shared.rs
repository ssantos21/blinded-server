use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Client -> SE
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct DepositMsg1 {
    pub auth: String,
    pub proof_key: String,
}

// schema struct for Uuid
#[derive(JsonSchema)]
#[schemars(remote = "Uuid")]
pub struct UuidDef(String);


/// Client -> SE
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct DepositMsg2 {
    #[schemars(with = "UuidDef")]
    pub shared_key_id: Uuid,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Default)]
pub struct UserID {
    #[schemars(with = "UuidDef")]
    pub id: Uuid,
    pub challenge: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Default)]
pub struct StatechainID {
    #[schemars(with = "UuidDef")]
    pub id: Uuid,
}

/// Statechain entity operating information
/// This struct is returned containing information on operating requirements
/// of the statechain entity which must be conformed with in the protocol.
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[schemars(example = "Self::example")]
pub struct StateEntityFeeInfoAPI {
    /// The Bitcoin address that the SE fee must be paid to
    pub address: String, // Receive address for fee payments
    /// The deposit fee, which is specified as a proportion of the deposit amount in basis points
    pub deposit: i64,    // basis points
    /// The withdrawal fee, which is specified as a proportion of the deposit amount in basis points
    pub withdraw: u64,   // basis points
    /// The decementing nLocktime (block height) interval enforced for backup transactions
    pub interval: u32,   // locktime decrement interval in blocks
    /// The initial nLocktime from the current blockheight for the first backup
    pub initlock: u32,   // inital backup locktime
    /// The minumum wallet version required
    pub wallet_version: String,
    /// Message to display to all wallet users on startup
    pub wallet_message: String,
}

impl StateEntityFeeInfoAPI{
    pub fn example() -> Self{
        Self{
            address: "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq".to_string(),
            deposit: 0,
            withdraw: 300,
            interval: 144,
            initlock: 14400,
            wallet_version: "0.4.65".to_string(),
            wallet_message: "Warning".to_string(),
        }
    }
}