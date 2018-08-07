mod table;
use table::schema::{TableSchema, PrimitiveSchema};
use table::row::Row;
use table::table::Table;
use table::output::{Output, OutputResult};

struct SimpleOutputer {}

impl Output for SimpleOutputer {
    type IOResult = ();

    fn write_schema(&mut self, _table:&Table) -> OutputResult
    {
        return OutputResult::Success();
    }

    fn preprocess(&mut self, _table:&Table) -> OutputResult
    {
        return OutputResult::Success();
    }

    fn write_record(&mut self, table:&Table, idx:usize) -> OutputResult
    {
        //TODO: typo
        let n = table.num_colmns();

        for i in 0..n
        {
            print!("{:?}\t", table.get_cell(idx, i));
        }

        print!("\n");
        return OutputResult::Success();
    }

    fn get_output_result(&mut self) {}
}

fn main() {
    let schema = TableSchema {
        sort_keys : vec![],
        sorted    : false,
        types     : vec![("name".to_string(), PrimitiveSchema::Str), ("pid".to_string(), PrimitiveSchema::Int), ("time".to_string(), PrimitiveSchema::Float)]
    };


    let name = "haohou".to_string();
    let mut row = Row::empty(&schema);

    print!("{}\n", row.set(0, &name));
    print!("{}\n", row.set(1, 12345));
    print!("{}\n", row.set(2, 0.01));

    let mut table = Table::empty(&schema);

    table.append(row);

    let mut outputer = SimpleOutputer{};

    table.dump(&mut outputer);

}
