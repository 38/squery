mod table;
//use table::row::Row;
use table::table::{Table, TableDataSource};
use table::input::Input;

mod writer;
use writer::tablewriter::TableOutputer;

mod reader;
use reader::linetext::LineTextReader;



fn main() {

    let mut br = std::io::BufReader::new("plumber 12345 1.0\n".as_bytes());

    let mut parser = LineTextReader::create_parser(&".name:String .pid:Int .time:Float".to_string(), &mut br);

    parser.add_field_sep('\t');
    parser.add_field_sep(' ');
    parser.add_field_sep('\n');


    let schema = parser.determine_table_schema();

    let mut table = Table::empty(schema.as_ref().unwrap(), TableDataSource::Parser(&mut parser, false));
    
    let mut outputer = TableOutputer::create();

    table.dump(&mut outputer).unwrap().print_text_table(160, 70);

}
