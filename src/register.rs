/* * * * * * * * * * * * * * * * * * * *
 * I N C L U D E S
 */

extern crate hyper;
extern crate serde_json;

use self::hyper::{
    Client
};
use std::io::Read;

/* * * * * * * * * * * * * * * * * * * *
 * T Y P E S
 */

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
pub struct Request {
    pub username: String,
    pub bind_email: bool,
    pub password: String,
    pub auth: Auth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub access_token: String,
    pub home_server: String,
    pub user_id: String,
    pub refresh_token: String,
}

/* * * * * * * * * * * * * * * * * * * *
 * I M P L E M E N T A T I O N S
 */

impl super::Request for Request {
    type R = Response;
    fn issue<T>(&self, url: String, args: T) -> Result<Self::R, super::Error> {
        let _ = args;
        let client = Client::new();
        let assembled_url = format!(
            "http://{}/register",
            url
        );
        println!("{}", assembled_url);
        let body;
        let mut request_builder = client.post(&assembled_url);

        body = serde_json::to_string(self).unwrap();
        request_builder = request_builder.body(&body);
        
        let response = request_builder.send();
        match response {
            Ok(mut v) => {
                let mut body = String::new();
                match v.status {
                    hyper::Ok => {
                        v.read_to_string(&mut body).unwrap();
                        match serde_json::from_str(&body) {
                            Ok(v) => Ok(v),
                            Err(_) => {
                                Err(super::Error::Query)
                            },
                        }
                    },
                    hyper::BadRequest => {
                        v.read_to_string(&mut body).unwrap();
                        match serde_json::from_str(&body) {
                            Ok(v) => Err(v),
                            Err(_) => {
                                Err(super::Error::Query)
                            },
                        }
                    },
                    _ => {
                        Err(super::Error::Matrix{
                            rest_errorcode: v.status.to_u16(),
                            errorcode: "".into(),
                            error: "".into(),
                        })
                    }
                }
            },
            _ => {
                Err(super::Error::Transport)
            },
        }
    }
}

impl super::Response for Response {
     fn is_valid() -> bool {
         // TODO: implement
         return true;
     }
}