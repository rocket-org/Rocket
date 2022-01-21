#![deny(warnings)]

use rocket::{
    async_trait, get,
    request::{FromRequest, Outcome},
    routes, Request,
};
use std::convert::Infallible;

struct State(String);

#[async_trait]
impl<'r> FromRequest<'r> for State {
    type Error = Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut state: Option<&String> = req.try_local_cache();
        if state.is_none() {
            let _ = req.local_cache(|| String::from("state set"));
        }
        let _ = req.local_cache(|| String::from("no state reset"));
        state = req.try_local_cache();

        Outcome::Success(State(state.expect("some state").clone()))
    }
}

#[get("/")]
fn top_route(state: State) -> String {
    state.0
}

#[cfg(test)]
mod request_local_cache_tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::{Build, Rocket};
    use rocket_http::Status;

    fn rocket() -> Rocket<Build> {
        rocket::build().mount("/", routes![top_route])
    }

    #[test]
    fn test_request_local_cache() {
        let client = Client::debug(rocket()).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string();
        assert_eq!(body, Some("state set".into()));
    }
}
