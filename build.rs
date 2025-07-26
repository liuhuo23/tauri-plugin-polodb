const COMMANDS: &[&str] = &[
    "inser_one",
    "insert_many",
    "find",
    "find_one",
    "delete_many",
    "count_documents",
    "drop_collection",
    "list_collection_names",
    "create_collection",
    "update_one",
    "update_many",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
