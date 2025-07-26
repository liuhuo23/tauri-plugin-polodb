use std::sync::Mutex;

use polodb_core::bson::Document;
use polodb_core::Database;
use polodb_core::{bson::to_document, CollectionT};
use serde::{de::DeserializeOwned, Serialize};
use tauri::Manager;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    api: PluginApi<R, Config>,
) -> crate::Result<Polodb<R>> {
    let path = app.path().app_data_dir();
    if path.is_err() {
        return Err(crate::Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "App data directory not found",
        )));
    }
    let config = api.config();
    let path = path.unwrap().join(&config.db_name);
    let db = Database::open_path(path)?;
    Ok(Polodb {
        app: app.clone(),
        db: Mutex::new(db),
    })
}

/// Access to the polodb APIs.
pub struct Polodb<R: Runtime> {
    #[allow(dead_code)]
    app: AppHandle<R>,
    db: Mutex<Database>,
}

impl<R: Runtime> Polodb<R> {
    /// 插入单条数据到指定集合
    /// collection: 集合名称
    /// payload: 可序列化的数据结构
    /// 返回 true 表示插入成功
    pub fn insert_one<T: Serialize>(&self, collection: &str, payload: T) -> crate::Result<bool> {
        let doc = to_document(&payload)?;
        let db = self.db.lock().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        db.collection(collection).insert_one(doc)?;
        Ok(true)
    }

    /// 插入多条数据到指定集合
    /// collection: 集合名称
    /// payloads: 可序列化的数据结构数组
    /// 返回 true 表示全部插入成功
    pub fn insert_many<T: Serialize>(
        &self,
        collection: &str,
        payloads: Vec<T>,
    ) -> crate::Result<bool> {
        let mut docs = Vec::new();
        for payload in payloads {
            docs.push(to_document(&payload)?);
        }
        let db = self.db.lock().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        db.collection(collection).insert_many(docs)?;
        Ok(true)
    }

    /// 查询指定集合的数据
    /// collection: 集合名称
    /// filter: 可选查询条件（BSON 文档）
    /// limit: 可选最大返回条数
    /// skip: 可选跳过条数
    /// sort: 可选排序条件（BSON 文档）
    /// 返回查询结果数组（Vec<Document>）
    pub fn find(
        &self,
        collection: &str,
        filter: Option<polodb_core::bson::Document>,
        limit: Option<u64>,
        skip: Option<u64>,
        sort: Option<polodb_core::bson::Document>,
    ) -> crate::Result<Vec<polodb_core::bson::Document>> {
        let db = self.db.lock().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        let coll = db.collection::<polodb_core::bson::Document>(collection);
        let mut cursor = coll.find(filter.unwrap_or_default());
        if let Some(skip) = skip {
            cursor = cursor.skip(skip);
        }
        if let Some(limit) = limit {
            cursor = cursor.limit(limit);
        }
        if let Some(sort_doc) = sort {
            cursor = cursor.sort(sort_doc);
        }
        let mut results = cursor.run().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        let mut docs = Vec::new();
        while let Some(doc) = results.next() {
            match doc {
                Ok(d) => docs.push(d),
                Err(e) => {
                    return Err(crate::Error::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    )))
                }
            }
        }
        Ok(docs)
    }

    /// 获取单个数据
    /// collection: 集合名称
    /// filter: 查询条件（BSON 文档）
    /// 返回查询结果（Option<Document>）
    pub fn find_one(
        &self,
        collection: &str,
        filter: polodb_core::bson::Document,
    ) -> crate::Result<Option<polodb_core::bson::Document>> {
        let db = self.db.lock().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        let coll = db.collection(collection);
        let result = coll.find_one(filter).map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        Ok(result)
    }

    /// 删除指定集合中的数据
    /// collection: 集合名称
    /// filter: 删除条件（BSON 文档）
    /// 返回删除的文档数量
    pub fn delete_many(
        &self,
        collection: &str,
        filter: polodb_core::bson::Document,
    ) -> crate::Result<u64> {
        let db = self.db.lock().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        let coll = db.collection::<Document>(collection);
        let result = coll.delete_many(filter).map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        Ok(result.deleted_count)
    }

    /// 统计指定集合中的文档数量
    /// collection: 集合名称
    /// 返回文档数量
    pub fn count_documents(&self, collection: &str) -> crate::Result<u64> {
        let db = self.db.lock().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        let coll = db.collection::<Document>(collection);
        let count = coll.count_documents().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        Ok(count)
    }

    /// 删除指定集合
    /// collection: 集合名称
    /// 返回是否删除成功
    pub fn drop_collection(&self, collection: &str) -> crate::Result<bool> {
        let db = self.db.lock().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        let coll = db.collection::<Document>(collection);
        coll.drop().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        Ok(true)
    }

    /// 获取所有集合
    /// 返回集合名称列表
    pub fn list_collection_names(&self) -> crate::Result<Vec<String>> {
        let db = self.db.lock().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        let collections = db.list_collection_names().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        Ok(collections)
    }

    /// 创建新的集合
    /// collection: 集合名称
    /// 返回是否创建成功
    pub fn create_collection(&self, collection: &str) -> crate::Result<bool> {
        let db = self.db.lock().map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        db.create_collection(collection).map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        Ok(true)
    }
}
