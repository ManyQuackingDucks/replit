
use std::marker::PhantomData;
use std::mem::transmute;
use std::ops::Index;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;


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
    reer: Vec<u8>,
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
            reer: vec![],
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
        let (tx, rx) = mpsc::channel();
        let key = key.to_string();
        let rsrt = self.clone();
        let arg = thread::spawn(move ||{
            let ar = rsrt.runtime.clone();
            ar.block_on(async move {
                let res = rsrt._get(&key).await;
                tx.send(res).unwrap();
            });
        });
        arg.join().unwrap();
        println!("Waiting for response");
        #[allow(mutable_transmutes)] //Yes i know this unsafe but ive run out of ideas it keeps halting
        unsafe { transmute::<&Self, &mut Self>(self).reer = rx.recv_timeout(Duration::new(3, 0)).unwrap().unwrap();} 
        std::str::from_utf8(&self.reer).unwrap()
    }
}
