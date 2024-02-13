// use derive_more::From;
// use tracing::subscriber::SetGlobalDefaultError;
// pub type Result<T> = core::result::Result<T, Error>;

// #[derive(Debug, From)]
// pub enum Error {
//     Generic(String),

//     #[from]
//     IoError(std::io::Error),

//     #[from]
//     GlobalError(SetGlobalDefaultError)
// }