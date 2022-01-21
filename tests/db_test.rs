use tokio::test;
use std::sync::Mutex;
use lazy_static::lazy_static;
lazy_static!{
    static ref TEST_LOCK: Mutex<()> = Mutex::new(());
}
#[test]
async fn test() {
    let _guard = TEST_LOCK.lock();
    let db = replit::db::Db::new().unwrap();
    db.insert("k", "v").await.unwrap();
    assert_eq!(db.get("k").await.unwrap(), "v");
    let list = db.list(None).await.unwrap();
    assert_eq!(list[0], "k");
    let list = db.list(Some("k")).await.unwrap();
    assert_eq!(list[0], "k");
    db.delete("k").await.unwrap();
}

#[test]
#[should_panic]
async fn test_get(){
    let _guard = TEST_LOCK.lock();
    let db = replit::db::Db::new().unwrap();
    assert_eq!(db.get("k").await.unwrap(), "v");
}

#[test]
#[should_panic]
async fn test_list(){
    let _guard = TEST_LOCK.lock();
    let db = replit::db::Db::new_with_url("https://example.com".to_string());
    let list = db.list(None).await.unwrap();
    assert_eq!(list[0], "k");
}


#[test]
#[should_panic]
async fn test_delete(){
    let _guard = TEST_LOCK.lock();
    let db = replit::db::Db::new().unwrap();
    db.delete("k").await.unwrap();
}