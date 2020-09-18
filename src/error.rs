#[derive(Debug)]
pub enum Error {
  StdError(std::io::Error),
  YmlError(serde_yaml::Error),
  HttpClientError(actix_web::client::SendRequestError),
  JsonPayloadError(awc::error::JsonPayloadError)
}

macro_rules! impl_from {
  ($a:ty, $b:expr) => {
    impl From<$a> for Error {
      fn from(e: $a) -> Self {
        $b(e)
      }
    }
  };
}

impl_from!(std::io::Error, Error::StdError);
impl_from!(serde_yaml::Error, Error::YmlError);
impl_from!(actix_web::client::SendRequestError, Error::HttpClientError);
impl_from!(awc::error::JsonPayloadError, Error::JsonPayloadError);
