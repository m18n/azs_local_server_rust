use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id_user: i32,
    admin: bool,

}
pub fn create_token(id_user_:i32,admin_:bool)->String{
    let my_claims = Claims {
        id_user:id_user_,
        admin:admin_

    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))
        .unwrap();
    token
}
