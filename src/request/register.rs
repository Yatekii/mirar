extern crate hyper;
extern crate serde_json;

use self::hyper::{
    client,
    Client,
};
use std::io::Read;

pub enum Kind {
    Guest,
    User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub session: String,
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub username: String,
    pub bind_email: bool,
    pub password: String,
    pub auth: Auth,
}

pub fn new<'a>(url: String, kind: Kind, data: &Register) -> client::RequestBuilder<'a> {
    let client = Client::new();
    let assembled_url = format!(
        "{}/register/{}",
        url,
        match kind{
            self::Kind::Guest => "guest",
            self::Kind::User => "user",
        }
    );
    let request_builder = client.post(
        &assembled_url
    );
    request_builder.body(serde_json::to_string(data).unwrap())
}