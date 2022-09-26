use tokio::test;
use std::sync::Mutex;
use replit_db::db::Db;
use lazy_static::lazy_static;
const TEST_URL: &str = "https://kv.replit.com/v0/eyJhbGciOiJIUzUxMiIsImlzcyI6ImNvbm1hbiIsImtpZCI6InByb2Q6MSIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJjb25tYW4iLCJleHAiOjE2NjM5OTIyNDAsImlhdCI6MTY2Mzg4MDY0MCwiZGF0YWJhc2VfaWQiOiI4OWFiZThkOS1lZGMxLTQ1ODgtOGIzMS0wZWI0MGRjOGFiNjMiLCJ1c2VyIjoiRHVja1F1YWNrIiwic2x1ZyI6IlJlcGxpdC1Ub2tlbi1TY2FubmVyIn0.YlYPck2tdNHMJYGb17nmCNUIyYsSHaQUZx4iky5DIe0HwoQbNbGmuUB7qDruCVQtOl9N2pFnRBP-Mwj-t30OKQ";
lazy_static!{
    static ref TEST_LOCK: Mutex<()> = Mutex::new(());
}
#[test]
async fn test_all() {
    let db = Db::new_with_url(TEST_URL.to_string()).await;
    db.insert("k", "v").await.unwrap();
    let var = db.get("k").await.unwrap();
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