use tokio::test;
use std::sync::Mutex;
use replit_db::db::DB;
use lazy_static::lazy_static;
const TEST_URL: &str = "https://kv.replit.com/v0/eyJhbGciOiJIUzUxMiIsImlzcyI6ImNvbm1hbiIsImtpZCI6InByb2Q6MSIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJjb25tYW4iLCJleHAiOjE2NjM5OTIyNDAsImlhdCI6MTY2Mzg4MDY0MCwiZGF0YWJhc2VfaWQiOiI4OWFiZThkOS1lZGMxLTQ1ODgtOGIzMS0wZWI0MGRjOGFiNjMiLCJ1c2VyIjoiRHVja1F1YWNrIiwic2x1ZyI6IlJlcGxpdC1Ub2tlbi1TY2FubmVyIn0.YlYPck2tdNHMJYGb17nmCNUIyYsSHaQUZx4iky5DIe0HwoQbNbGmuUB7qDruCVQtOl9N2pFnRBP-Mwj-t30OKQ";
//yes I this should not be shared but eh.
lazy_static!{
    static ref TEST_LOCK: Mutex<()> = {
        let _ = std::env::set_var("REPLIT_DB_URL", TEST_URL);//set the url to the test url
        Mutex::new(())
    };
}
#[test]
async fn test_all() {
    let _guard = TEST_LOCK.lock();
    DB.insert("k", "v").await.unwrap();
    assert_eq!(DB.get("k").await.unwrap(), "v");
    let list = DB.list(None).await.unwrap();
    assert_eq!(list[0], "k");
    let list = DB.list(Some("k")).await.unwrap();
    assert_eq!(list[0], "k");
    DB.remove("k").await.unwrap();
}

#[test]
#[should_panic]
async fn test_get(){
    let _guard = TEST_LOCK.lock();
    assert_eq!(DB.get("k").await.unwrap(), "v");
}

#[test]
#[should_panic]
async fn test_list(){
    let _guard = TEST_LOCK.lock();
    let list = DB.list(None).await.unwrap();
    assert_eq!(list[0], "k");
}


#[test]
#[should_panic]
async fn test_delete(){
    let _guard = TEST_LOCK.lock();
    DB.remove("k").await.unwrap();
}