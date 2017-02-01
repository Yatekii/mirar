extern crate hyper;
extern crate serde_json;

use respond;

use self::hyper::{
    Client,
    status
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

pub fn issue(url: String, kind: Kind, data: &Register) -> Result<respond::register::Register, status::StatusCode> {
    let client = Client::new();
    let assembled_url = format!(
        "http://{}/register",
        url
    );
    println!("{}", assembled_url);
    let body;
    let mut request_builder = client.post(&assembled_url);

    body = serde_json::to_string(data).unwrap();
    request_builder = request_builder.body(&body);
    
    let response = request_builder.send();
    match response {
        Ok(mut v) => {
            match v.status {
                hyper::Ok => {
                    let mut body = String::new();
                    v.read_to_string(&mut body).unwrap();
                    match serde_json::from_str(&body) {
                        Ok(v) => Ok(v),
                        Err(v) => {println!("{}", v); Err(status::StatusCode::Unregistered(0))},
                    }
                },
                _ => Err(v.status),
            }
        },
        Err(v) => Err(status::StatusCode::Unregistered(0)),
    }
}