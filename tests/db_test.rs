use tokio::test;
use std::sync::Mutex;
use replit_db::db::Db;
use lazy_static::lazy_static;
const TEST_URL: &str = "";
lazy_static!{
    static ref TEST_LOCK: Mutex<()> = Mutex::new(());
}
#[test]
async fn test_all() {
    let db = Db::new_with_url(TEST_URL.to_string()).await;
    db.insert("k", "v").await.unwrap();
    assert_eq!(db.get("k").await.unwrap(), "v");
    let gar = &db["k"];
    assert_eq!(gar, "v");
    let list = db.list(None).await.unwrap();
    assert_eq!(list[0], "k");
    let list = db.list(Some("k")).await.unwrap();
    assert_eq!(list[0], "k");
    db.remove("k").await.unwrap();
}

#[test]
#[should_panic]
async fn test_get(){
    let _guard = TEST_LOCK.lock();
    let db = Db::new_with_url(TEST_URL.to_string()).await;
    assert_eq!(db.get("k").await.unwrap(), "v");
}

#[test]
#[should_panic]
async fn test_list(){
    let _guard = TEST_LOCK.lock();
    let db = Db::new_with_url(TEST_URL.to_string()).await;
    let list = db.list(None).await.unwrap();
    assert_eq!(list[0], "k");
}


#[test]
#[should_panic]
async fn test_delete(){
    let _guard = TEST_LOCK.lock();
    let db = Db::new().await.unwrap();
    db.remove("k").await.unwrap();
}