use rocket::{State, serde::json::Json};

use crate::{server::StateChainEntity, shared::{DepositMsg1, UserID}, error, protocol::deposit::Deposit, storage::db::MercuryPgDatabase};

#[post("/deposit/init", format = "json", data = "<deposit_msg1>")]
pub async fn deposit_init(sc_entity: &State<StateChainEntity>, db: MercuryPgDatabase, deposit_msg1: Json<DepositMsg1>) -> Result<Json<UserID>, error::SEError> {

    let deposit_msg1 = deposit_msg1.0;

    match sc_entity.deposit_init(db, deposit_msg1).await {
        Ok(res) => return Ok(Json(res)),
        Err(e) => return Err(e),
    }
}