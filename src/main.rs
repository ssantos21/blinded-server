
#[macro_use] extern crate rocket;

mod schema;

use diesel::{Insertable, RunQueryDsl, PgConnection};
use rand::Rng;
use rocket::{serde::json::{Value, json, Json}, response::status::Custom};
use rocket_sync_db_pools::{database, diesel};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[database("mercury_pg_db")]
struct MercuryPgDatabase(PgConnection);

// schema struct for Uuid
#[derive(JsonSchema)]
#[schemars(remote = "Uuid")]
pub struct UuidDef(String);

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Default)]
pub struct UserID {
    #[schemars(with = "UuidDef")]
    pub id: Uuid,
    pub challenge: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct DepositMsg1 {
    pub auth: String,
    pub proof_key: String,
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!\n"
}

#[derive(Deserialize, Insertable, Clone)]
#[diesel(table_name = schema::usersession)]
struct NewUserSession {
    id: Uuid, 
    authentication: String,
    proofkey: String,
    challenge: String,
}

fn create_user_session(db: &mut PgConnection, new_user_session: &NewUserSession) {

    diesel::insert_into(schema::usersession::table)
        .values(new_user_session)
        .execute(db)
        .expect("Error saving new user session");
}

fn create_lockbox_item(db: &mut PgConnection, user_id: &Uuid) {

    #[derive(Deserialize, Insertable, Clone)]
    #[diesel(table_name = schema::lockbox)]
    struct NewLockboxItem {
        id: Uuid
    }

    let new_lockbox_item = NewLockboxItem { id: user_id.clone() };

    diesel::insert_into(schema::lockbox::table)
        .values(new_lockbox_item)
        .execute(db)
        .expect("Error saving new lockbox item");
}

fn create_challenge() -> String {
    let mut rng = rand::thread_rng();
    let challenge_bytes = rng.gen::<[u8; 16]>();
    // let challenge = hex::encode(challenge_bytes);
    hex::encode(challenge_bytes)
}

#[post("/deposit/init", format = "json", data = "<deposit_msg1>")]
async fn deposit_init(db: MercuryPgDatabase, deposit_msg1: Json<DepositMsg1>) -> Result<Json<UserID>, Custom<Value>> {

    // Generate shared wallet ID (user ID)
    let user_id = Uuid::new_v4();

    let new_user_session = NewUserSession { 
        id: user_id.clone(),
        authentication: deposit_msg1.auth.to_string(),
        proofkey: deposit_msg1.proof_key.to_string(),
        challenge: create_challenge()
    };

    let challenge = new_user_session.challenge.clone();

    db.run(move |c| { create_user_session(c, &new_user_session) }).await;
    db.run(move |c| { create_lockbox_item(c, &user_id) }).await;

    Ok(Json(UserID {id: user_id, challenge: Some(challenge)}))
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            hello,
            deposit_init,
        ])
        .register("/", catchers![
            not_found
        ])
        .attach(MercuryPgDatabase::fairing())
        .launch()
        .await;
}
