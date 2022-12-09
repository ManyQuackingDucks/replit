use crate::error::{DBErrors};
use hyper::client::connect::HttpConnector;
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use log::{debug, error};
lazy_static::lazy_static! {
    pub static ref DB: Db = Db::new().unwrap();
}
pub type Result<T> = std::result::Result<T, crate::error::DBErrors>;

#[derive(Clone)]
pub struct Db {
    uri: String,
    client: Client<HttpsConnector<HttpConnector>>,
}
impl Db {
    ///Create a new struct that can be used to interact with the db.
    fn new() -> Result<Self> {
        Ok(Self {
            uri: std::env::var("REPLIT_DB_URL")?,
            client: Client::builder().build::<_, hyper::Body>(HttpsConnector::new()),
        })
    }

    ///Insert's a key into the db
    pub async fn insert(&self, k: &str, v: &str) -> Result<()> {
        debug!("{}={}", k, v);
        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}/{}={}", self.uri, k, v).parse::<Uri>().unwrap())
            .body(Body::from(""))
            .expect("request builder");
        let res = self.client.request(req).await?;
        if res.status().is_success() {
            Ok(())
        } else {
            error!(
                "Failed to insert key into db. Status code: {}, Input: {}={}",
                res.status(),
                k,
                v
            );
            Err(DBErrors::NotSucc)
        }
    }

    ///Deletes a key from the db.
    pub async fn remove(&self, k: &str) -> Result<()> {
        let req = Request::builder()
            .method(Method::DELETE)
            .uri(format!("{}/{}", self.uri, k).parse::<Uri>().unwrap())
            .body(Body::from(""))
            .expect("request builder");
        let res = self.client.request(req).await?;
        if res.status().is_success() {
            Ok(())
        } else {
            error!(
                "Failed to remove key from db. Status code: {}, Input: {}",
                res.status(),
                k
            );
            Err(DBErrors::NotFound(k.to_string()))

        }

    }
    ///Gets a key from the db.
    pub async fn get(&self, k: &str) -> Result<String> {
        let mut res = self
            .client
            .get(format!("{}/{}", self.uri, k).parse::<Uri>().unwrap())
            .await?;
        if res.status().is_success() {
            let string = hyper::body::to_bytes(res.body_mut()).await?.to_vec();
            let string = std::str::from_utf8(&string)?.to_string();
            Ok(string) //returns a borrowed string that lasts the Db's lifetime
        } else {
            error!(
                "Failed to get key from db. Status code: {}, Input: {}",
                res.status(),
                k
            );
            Err(DBErrors::NotFound(k.to_string()))
        }
    }
    
    ///Lists keys begining with an optional prefix.
    ///If None is supplied then list will output all keys.
    pub async fn list(&self, prefix: Option<&str>) -> Result<Vec<String>> {
        let uri = match prefix {
            Some(k) => format!("{}?prefix={}", self.uri, k),
            None => format!("{}?prefix=", self.uri),
        };
        let res = self.client.get(uri.parse::<Uri>().unwrap()).await?;
        if res.status().is_success() {
            let buf = hyper::body::to_bytes(res).await?.to_vec();
            let string = std::str::from_utf8(&buf)?;
            let vec: Vec<String> = string.lines().map(|i| i.to_string()).collect();
            Ok(vec)
        } else {
            error!(
                "Failed to list keys from db. Status code: {}, Input: {:?}",
                res.status(),
                prefix
            );
            Err(DBErrors::NotSucc)
        }
    }
}
