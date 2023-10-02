// use async_trait::async_trait;
// use hmac::digest::KeyInit;
// use serde::Deserialize;
// use serde::Serialize;
// use hmac::Hmac;
// use crate::entities::token::Token;
// use crate::services::base::OurService;
// use crate::services::base::OurResult;
//
// pub struct GenerateTokenService {
//     private_key: String
// }
//
// pub struct Params<'a, T>
// where T: Serialize + Deserialize<'a> {
//     content: T
// }
//
// #[async_trait]
// impl<'a, T> OurService<Params<'a, T>, Token> for GenerateTokenService {
//     async fn execute(self, params: Params<T>) -> OurResult<Token> {
//         let key = Hmac::new_from_slice(self.private_key.as_bytes())?;
//
//         Ok(Token {
//             value: "".to_owned(),
//             created_at: Default::default(),
//         })
//     }
// }
//
