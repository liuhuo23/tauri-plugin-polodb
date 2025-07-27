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
            commands::update_one,
            commands::update_many,
            commands::find_one,
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
