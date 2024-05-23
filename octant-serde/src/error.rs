// #[derive(Debug)]
// pub enum Error {
//     Custom(String),
//     Json(serde_json::Error),
// }
//
// impl From<serde_json::Error> for Error {
//     fn from(error: serde_json::Error) -> Self {
//         Error::Json(error)
//     }
// }
//
// impl Error{
//     pub fn custom(){
//     }
// }

pub type Error=anyhow::Error;