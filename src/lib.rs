use polodb_core::bson::Document;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Polodb;
#[cfg(mobile)]
use mobile::Polodb;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the polodb APIs.
pub trait PolodbExt<R: Runtime> {
    fn polodb(&self) -> &Polodb<R>;
    fn inser_one(&self, collection: &str, payload: Document) -> crate::Result<bool> {
        self.polodb().insert_one(collection, payload)
    }
    fn insert_many(&self, collection: &str, payloads: Vec<Document>) -> crate::Result<bool> {
        self.polodb().insert_many(collection, payloads)
    }

    fn find(
        &self,
        collection: &str,
        filter: Option<Document>,
        limit: Option<u64>,
        skip: Option<u64>,
        sort: Option<Document>,
    ) -> crate::Result<Vec<Document>> {
        self.polodb().find(collection, filter, limit, skip, sort)
    }

    fn delete_many(&self, collection: &str, filter: Document) -> crate::Result<u64> {
        self.polodb().delete_many(collection, filter)
    }
    fn count_documents(&self, collection: &str) -> crate::Result<u64> {
        self.polodb().count_documents(collection)
    }
    fn drop_collection(&self, collection: &str) -> crate::Result<bool> {
        self.polodb().drop_collection(collection)
    }
}

impl<R: Runtime, T: Manager<R>> crate::PolodbExt<R> for T {
    fn polodb(&self) -> &Polodb<R> {
        self.state::<Polodb<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    Builder::<R, Config>::new("polodb")
        .invoke_handler(tauri::generate_handler![
            commands::inser_one,
            commands::insert_many,
            commands::find,
            commands::delete_many,
            commands::count_documents,
            commands::drop_collection,
            commands::list_collection_names,
            commands::create_collection,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let polodb = mobile::init(app, api)?;
            #[cfg(desktop)]
            let polodb = desktop::init::<R, Config>(app, api)?;
            app.manage(polodb);
            Ok(())
        })
        .build()
}
