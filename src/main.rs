
#[macro_use] extern crate rocket;

mod schema;
mod server;
mod config;
mod protocol;
mod error;
mod shared;
mod endpoints;
mod storage;

use rocket::{serde::json::{Value, json}};
use rocket_sync_db_pools::diesel;
use server::StateChainEntity;
use storage::db::MercuryPgDatabase;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!\n"
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[rocket::main]
async fn main() {
    let sc_entity = StateChainEntity::load();
    
    let _ = rocket::build()
        .mount("/", routes![
            hello,
            endpoints::deposit::deposit_init,
            endpoints::util::get_fees
        ])
        .register("/", catchers![
            not_found
        ])
        .attach(MercuryPgDatabase::fairing())
        .manage(sc_entity)
        .launch()
        .await;
}
