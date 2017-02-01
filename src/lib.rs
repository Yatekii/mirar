pub mod request;
pub mod respond;

#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub errorcode: String,
    pub error: String,
}