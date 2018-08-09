mod table;
//use table::row::Row;
use table::table::{Table, TableDataSource};
use table::input::Input;

mod writer;
use writer::tablewriter::TableOutputer;

mod reader;
use reader::svparser::SepValParser;
use reader::exec::ExecReader;



fn main() {
    //let schema = ".pid:Int .tty:String .time:String sorted:pid".to_string();
    let schema = ".lines:Int .words:Int .chars:Int .file:String".to_string();
    //let br = std::io::BufReader::new("plumber 12345 1.0\n".as_bytes());
    
    if let Some(mut parser) = ExecReader::create("wc", &["src/main.rs", "src/reader/exec.rs"], 0, &schema, SepValParser::create(&" \t\n".to_string()))
    {

        let schema = parser.determine_table_schema();

        let mut table = Table::empty(schema.as_ref().unwrap(), TableDataSource::Parser(&mut parser, false));
        
        let mut outputer = TableOutputer::create();

        table.dump(&mut outputer).unwrap().print_text_table(160, 70);
    }

}
