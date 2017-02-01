pub mod register;

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