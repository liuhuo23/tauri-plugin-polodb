use polodb_core::bson::{doc, to_document, Document};
use polodb_core::{CollectionT, Database};
#[test]
fn test_insert_one() {
    // 使用临时数据库文件
    let db = Database::open_path("test_polodb.db").unwrap();

    // 插入文档
    let coll = db.collection("default");
    let payload = doc! { "name": "test", "value": 42 };
    let doc = to_document(&payload).unwrap();
    coll.insert_one(doc).unwrap();
    // 验证插入
    let count = coll.count_documents().unwrap();
    assert_eq!(count, 4);
    drop(db);
    // 清理测试文件
    std::fs::remove_dir_all("test_polodb.db").unwrap();
}

#[test]
fn test_insert_many() {
    // 使用临时数据库文件
    let db = Database::open_path("test_polodb.db").unwrap();
    // 插入多个文档
    let coll = db.collection("default");
    let payloads = vec![
        doc! { "name": "test1", "value": 10 },
        doc! { "name": "test2", "value": 20 },
        doc! { "name": "test3", "value": 30, "extra": "data" },
    ];
    let docs: Vec<Document> = payloads
        .into_iter()
        .map(|value: polodb_core::bson::Document| to_document(&value))
        .collect::<Result<Vec<Document>, _>>()
        .unwrap();
    coll.insert_many(docs).unwrap();
    // 验证插入
    let count = coll.count_documents().unwrap();
    assert_eq!(count, 6);
    drop(db);
    // 清理测试文件
    std::fs::remove_dir_all("test_polodb.db").unwrap();
}

#[test]
fn test_query() {
    // 使用临时数据库文件
    let db = Database::open_path("test_polodb.db").unwrap();
    // 插入文档
    let coll = db.collection("default");
    let payload = doc! { "name": "query_test", "value": 100 };
    let doc = to_document(&payload).unwrap();
    coll.insert_one(doc).unwrap();

    // 查询文档
    let query = doc! { "name": "query_test" };
    let result: Vec<Document> = coll
        .find(query)
        .run()
        .unwrap()
        .map(Result::unwrap)
        .collect();

    // 验证查询结果
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].get_str("name").unwrap(), "query_test");

    drop(db);
    // 清理测试文件
    std::fs::remove_dir_all("test_polodb.db").unwrap();
}
