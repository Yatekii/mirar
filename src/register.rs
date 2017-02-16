/* * * * * * * * * * * * * * * * * * * *
 * I N C L U D E S
 */

extern crate hyper;
extern crate serde_json;

use self::hyper::{
    Client
};
use std::io::Read;
use std::collections::HashMap;

/* * * * * * * * * * * * * * * * * * * *
 * T Y P E S
 */

 #[derive(Eq, PartialEq, Hash, Serialize, Deserialize, Debug)]
pub enum QueryKey {
    kind,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum QueryValue {
    guest,
    user,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub session: String,
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub username: String,
    pub bind_email: bool,
    pub password: String,
    pub auth: Auth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    pub access_token: String,
    pub home_server: String,
    pub user_id: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponseBody {
    pub errorcode: String,
    pub error: String,
}

/* * * * * * * * * * * * * * * * * * * *
 * I M P L E M E N T A T I O N S
 */

impl super::QueryKey for QueryKey {
}

impl super::QueryValue for QueryValue {
}

impl super::RequestBody for RequestBody {
}

impl super::ResponseBody for ResponseBody {
}

impl super::ResponseBody for ErrorResponseBody {
}

pub fn construct(username: String, password: String, bind_email: bool, session: String, typ: String,) -> super::Request<RequestBody, ResponseBody> {
    return super::Request {
        url: "register".into(),
        request_type: super::RequestType::POST,
        arguments: HashMap::<QueryKey, QueryValue>::new(),
        request: RequestBody {
            username: username,
            bind_email: bind_email,
            password: password,
            auth: Auth {
                session: session,
                typ: typ,
            },
        },
        response: ResponseBody {
            access_token: "".into(),
            home_server: "".into(),
            user_id: "".into(),
            refresh_token: "".into(),
        },
        status_code: super::Status::Ok,
    }
}