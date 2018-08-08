mod table;
//use table::row::Row;
use table::table::{Table, TableDataSource};
use table::input::Input;

mod writer;
use writer::tablewriter::TableOutputer;

mod reader;
use reader::linetext::LineTextReader;
use reader::svparser::SepValParser;



fn main() {

    let mut br = std::io::BufReader::new("plumber 12345 1.0\n".as_bytes());

    if let Some(mut parser) = LineTextReader::create_parser(&".name:String .pid:Int .time:Float".to_string(), &mut br, SepValParser::create(&" \t\n".to_string()))
    {

        let schema = parser.determine_table_schema();

        let mut table = Table::empty(schema.as_ref().unwrap(), TableDataSource::Parser(&mut parser, false));
        
        let mut outputer = TableOutputer::create();

        table.dump(&mut outputer).unwrap().print_text_table(160, 70);
    }

}
