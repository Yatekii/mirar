extern crate hyper;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub access_token: String,
    pub home_server: String,
    pub user_id: String,
    pub refresh_token: String,
}

impl Register {
     pub fn validate() -> bool {
         // TODO: implement
         return true;
     }
}