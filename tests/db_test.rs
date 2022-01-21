
#[tokio::test]
async fn test() {
    let db = replit::db::Db::new().unwrap();
    db.insert("k", "v").await.unwrap();
    assert_eq!(db.get("k").await.unwrap(), "v");
    let list = db.list(None).await.unwrap();
    assert_eq!(list[0], "k");
    db.delete("k").await.unwrap();
}
