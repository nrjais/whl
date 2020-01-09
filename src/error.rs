#[derive(Debug)]
pub enum Error {
  StdError(std::io::Error),
  YmlError(serde_yaml::Error),
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
