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

impl super::RequestBody for RequestBody {
}

impl super::ResponseBody for ResponseBody {
}

impl super::ResponseBody for ErrorResponseBody {
}