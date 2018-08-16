
#[macro_use]
mod reader;
mod writer;
mod table;
mod schema;

use schema::loader::SchemaManager;



fn main() {
    let mut schema_man = SchemaManager::new();

    schema_man.push_schema_path("./data".to_string());

    schema_man.query("wc", &["-l"]);
}
