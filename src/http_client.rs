use awc::Client;
use serde::{de::DeserializeOwned, Serialize};

pub struct HttpClient(Client);

impl<'a> HttpClient {
  async fn post<T: Serialize, R: DeserializeOwned>(&self, url: &str, body: &T) -> crate::Result<R> {
    Ok(self.0.post(url).send_json(body).await?.json().await?)
  }
}
