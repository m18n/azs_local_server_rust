use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id_user: i32,
    pub admin: bool,
    pub exp: usize,
}
pub fn create_token(id_user_:i32,admin_:bool)->String{
    let my_claims = Claims {
        id_user:id_user_,
        admin:admin_,
        exp:10000000000
    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))
        .unwrap();
    token
}
