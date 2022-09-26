

use std::ops::Index;
use crossbeam::channel;


use crate::error::{DBErrors};
use hyper::client::connect::HttpConnector;
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use log::{debug, error};
pub type Result<T> = std::result::Result<T, crate::error::DBErrors>;

#[derive(Clone)]
pub struct Db {
    uri: String,
    client: Client<HttpsConnector<HttpConnector>>,
    runtime: tokio::runtime::Handle,
}
impl Db {
    ///Create a new struct that can be used to interact with the db.
    pub async fn new() -> Result<Self> {
        Ok(Self::new_with_url(std::env::var("REPLIT_DB_URL")?).await)
    }
    pub async fn new_with_url(url: String) -> Self {
        debug!("{}", url);
        Self {
            uri: url,
            client: Client::builder().build::<_, hyper::Body>(HttpsConnector::new()),
            runtime: tokio::runtime::Handle::current(),
        }
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
    pub async fn get(&self, k: &str) -> Result<String>{
        Ok(std::str::from_utf8(&self._get(k).await?).unwrap().to_string())
    }
    async fn _get(&self, k: &str) -> Result<Vec<u8>> {
        let res = self
            .client
            .get(format!("{}/{}", self.uri, k).parse::<Uri>().unwrap())
            .await?;
        if res.status().is_success() {
            let buf = hyper::body::to_bytes(res).await.unwrap().to_vec();
            
            Ok(buf) //returns a borrowed string that lasts the Db's lifetime
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

impl<'a> Index<&str> for Db
{
    type Output = str;
    /// Returns a reference to the value corresponding to the supplied key.
    /// db["key"] will return a reference to the value of the key.
    /// # Panics
    ///
    /// Panics if the key is not present.
    fn index(&self, key: &str) -> &str
    {
        let (tx, rx) = channel::bounded(1);
        let key = key.to_string();
        let rsrt = self.clone();
        self.runtime.spawn(async move {
            let res = rsrt.get(&key).await;
            tx.send(res).unwrap();
        });
        let res = rx.recv().unwrap().unwrap();
        Box::leak(Box::new(res))
    }
}
#[tokio::test]
async fn target(){
    let DB = Db::new_with_url("https://kv.replit.com/v0/eyJhbGciOiJIUzUxMiIsImlzcyI6ImNvbm1hbiIsImtpZCI6InByb2Q6MSIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJjb25tYW4iLCJleHAiOjE2NjQzMzMxODAsImlhdCI6MTY2NDIyMTU4MCwiZGF0YWJhc2VfaWQiOiI4OWFiZThkOS1lZGMxLTQ1ODgtOGIzMS0wZWI0MGRjOGFiNjMiLCJ1c2VyIjoiRHVja1F1YWNrIiwic2x1ZyI6IlJlcGxpdC1Ub2tlbi1TY2FubmVyIn0.EOe1NKJGRusI-v8-yQts01Q43qwFgQbP3Tw6aXzspA-FI_jeGCRS4Ud5k1YGnlmriEBpc5xrXQ6-NtE213--_w".to_string()).await;
    DB.insert("test", "test").await.unwrap();
    assert_eq!(&DB["test"], "test");
    DB.remove("test").await.unwrap();
}