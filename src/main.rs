mod table;
use table::schema::{TableSchema, PrimitiveSchema};
use table::row::Row;
use table::table::Table;

mod writer;
use writer::tablewriter::TableOutputer;

fn main() {
    let schema = TableSchema {
        sort_keys : vec![],
        sorted    : false,
        types     : vec![("name".to_string(), PrimitiveSchema::Str), ("pid".to_string(), PrimitiveSchema::Int), ("time".to_string(), PrimitiveSchema::Float)]
    };


    let name = "plumber".to_string();
    let mut row = Row::empty(&schema);

    row.set(0, &name);
    row.set(1, 12345);
    row.set(2, 0.01);

    let mut table = Table::empty(&schema);

    table.append(row);

    let mut outputer = TableOutputer::create();
    let result = table.dump(&mut outputer);

    if result.is_some()
    {
        result.unwrap().print_text_table(160, 70);
    }

}
