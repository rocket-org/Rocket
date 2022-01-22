#![cfg(feature = "msgpack")]

use rocket::serde::{msgpack::MsgPack, Deserialize, Serialize};
use rocket::{get, routes};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
struct Data {
    framework: String,
    stars: usize,
}

#[get("/", format = "msgpack")]
fn top_route() -> MsgPack<Data> {
    MsgPack(Data {
        framework: "Rocket".to_string(),
        stars: 5,
    })
}

#[cfg(test)]
mod msgpack_tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::{Build, Rocket};
    use rocket_http::{ContentType, Status};

    fn rocket() -> Rocket<Build> {
        rocket::build().mount("/", routes![top_route])
    }

    #[cfg(feature = "msgpack-compact")]
    #[test]
    fn test_msgpack_compact() {
        let client = Client::debug(rocket()).unwrap();
        let response = client.get("/").header(ContentType::MsgPack).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::MsgPack));

        let data = response.into_msgpack::<Data>();
        assert_eq!(
            data,
            Some(Data {
                framework: "Rocket".to_string(),
                stars: 5,
            })
        );

        let response = client.get("/").header(ContentType::MsgPack).dispatch();

        let bytes = response.into_bytes();
        assert_eq!(
            bytes,
            Some(vec!(146_u8, 166, 82, 111, 99, 107, 101, 116, 5))
        );
    }

    #[cfg(not(feature = "msgpack-compact"))]
    #[test]
    fn test_msgpack_named() {
        let client = Client::debug(rocket()).unwrap();
        let response = client.get("/").header(ContentType::MsgPack).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::MsgPack));

        let data = response.into_msgpack::<Data>();
        assert_eq!(
            data,
            Some(Data {
                framework: "Rocket".to_string(),
                stars: 5,
            })
        );

        let response = client.get("/").header(ContentType::MsgPack).dispatch();

        let bytes = response.into_bytes();
        assert_eq!(
            bytes,
            Some(vec!(
                130_u8, 169, 102, 114, 97, 109, 101, 119, 111, 114, 107, 166, 82, 111, 99, 107,
                101, 116, 165, 115, 116, 97, 114, 115, 5
            ))
        );
    }
}
