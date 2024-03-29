use rocket_db_pools::{Connection, Database};
use rocket_db_pools::deadpool_redis::Pool;
use rocket_db_pools::deadpool_redis::redis::cmd;
use rocket::{build, get, main, routes};
use rocket::request::FromParam;

#[derive(Database)]
#[database("redis")]
struct PwnedPasswords(Pool);

struct Hash<'r> {
    value: &'r str,
}

impl<'r> FromParam<'r> for Hash<'r> {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        if 40 != param.chars().count() {
            return Err("Hash is not expected length.");
        }

        if !param.chars().all(|c| (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9')) {
            return Err("Hash is not valid.");
        }

        Ok(Hash { value: param })
    }
}

#[get("/<hash>")]
async fn pwned(mut redis: Connection<PwnedPasswords>, hash: Hash<'_>) -> String {
    let value: u8 = cmd("BF.EXISTS")
        .arg("pwned-bloom")
        .arg(hash.value)
        .query_async(&mut *redis)
        .await.unwrap();

    format!("{}", value)
}

#[main]
async fn main() {
    let _ = build().attach(PwnedPasswords::init()).mount("/api", routes![pwned]).launch().await;
}
