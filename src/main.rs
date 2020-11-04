use rocket;
use rocket_contrib::databases::postgres;

#[rocket_contrib::database("hello")]
pub struct HelloDb(postgres::Client);


fn web() -> rocket::Rocket {
    rocket::ignite().attach(HelloDb::fairing())
}

#[rocket::main]
async fn main() {
    web().launch().await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::{web, HelloDb};
    use rocket::local::asynchronous::Client;


    #[rocket::async_test]
    async fn this_will_fail() {
        let rocket = web();
        let client = Client::tracked(rocket).await.unwrap();
        let conn = HelloDb::get_one(client.rocket()).await.unwrap();
    }
}
