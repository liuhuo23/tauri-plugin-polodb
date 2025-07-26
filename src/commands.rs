use polodb_core::bson::Document;
use tauri::{command, AppHandle, Runtime};

use crate::PolodbExt;
use crate::Result;

#[command]
pub(crate) async fn inser_one<R: Runtime>(
    app: AppHandle<R>,
    collection: &str,
    payload: Document,
) -> Result<bool> {
    app.polodb().insert_one(collection, payload)?;
    Ok(true)
}

#[command]
pub(crate) async fn insert_many<R: Runtime>(
    app: AppHandle<R>,
    collection: &str,
    payloads: Vec<Document>,
) -> Result<bool> {
    app.polodb().insert_many(collection, payloads)?;
    Ok(true)
}

#[command]
pub(crate) async fn find<R: Runtime>(
    app: AppHandle<R>,
    collection: &str,
    filter: Option<Document>,
    limit: Option<u64>,
    skip: Option<u64>,
    sort: Option<Document>,
) -> Result<Vec<Document>> {
    let results = app.polodb().find(collection, filter, limit, skip, sort)?;
    Ok(results)
}

#[command]
pub(crate) async fn delete_many<R: Runtime>(
    app: AppHandle<R>,
    collection: &str,
    filter: Document,
) -> Result<bool> {
    app.polodb().delete_many(collection, filter)?;
    Ok(true)
}

#[command]
pub(crate) async fn count_documents<R: Runtime>(
    app: AppHandle<R>,
    collection: &str,
) -> Result<u64> {
    let count = app.polodb().count_documents(collection)?;
    Ok(count)
}

#[command]
pub(crate) async fn drop_collection<R: Runtime>(
    app: AppHandle<R>,
    collection: &str,
) -> Result<bool> {
    app.polodb().drop_collection(collection)?;
    Ok(true)
}

#[command]
pub(crate) async fn list_collection_names<R: Runtime>(app: AppHandle<R>) -> Result<Vec<String>> {
    let names = app.polodb().list_collection_names()?;
    Ok(names)
}

#[command]
pub(crate) async fn create_collection<R: Runtime>(app: AppHandle<R>, name: &str) -> Result<()> {
    app.polodb().create_collection(name)?;
    Ok(())
}

#[command]
pub(crate) async fn update_one<R: Runtime>(
    app: AppHandle<R>,
    collection: &str,
    filter: Document,
    update: Document,
) -> Result<Option<Document>> {
    app.polodb().update_one(collection, filter, update)
}

#[command]
pub(crate) async fn update_many<R: Runtime>(
    app: AppHandle<R>,
    collection: &str,
    filter: Document,
    update: Document,
) -> Result<Vec<Document>> {
    app.polodb().update_many(collection, filter, update)
}

#[command]
pub async fn find_one<R: Runtime>(
    app: AppHandle<R>,
    collection: &str,
    filter: Document,
) -> Result<Option<Document>> {
    app.polodb().find_one(collection, filter)
}
