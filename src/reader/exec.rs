/*
 * Copyright (C) 2018, Hao Hou
 *
 * The line reader that makes the standard output as a structured table
 */
use std::process::{Command, ChildStdout, Stdio};
use std::io::{BufReader, BufRead};
use reader::linetext::{LineTextReader, LineParser};
use table::input::Input;
use table::schema::TableSchema;
use table::row::Row;

/**
 * The reader object that executes a command
 **/
pub struct ExecReader<TParser : LineParser> {
    /// The actual line reader we used to parse the result
    line_reader: LineTextReader<ChildStdout , TParser>
}

impl <TParser:LineParser> ExecReader<TParser> {
    /**
     * Create a new execute reader
     *
     * * `program`: The program we want to call
     * * `param`: args The arguments 
     * * `param` skip How many lines we want to skip before parsing
     * * `param` parser The line parser instance
     *
     * Returns the newly created execution reader
     **/
    #[allow(dead_code)]
    pub fn create<'x, 'y, 'z>(program:&'x str, args:&'y[&'y str], skip:usize, schema:&'z String, parser:TParser) -> Option<ExecReader<TParser>>
    {
        if let Ok(child) = Command::new(program).args(args).stdout(Stdio::piped()).spawn()
        {
            if let Some(stdout) = child.stdout
            {
                let mut br  = BufReader::new(stdout);

                let mut skipped = 0;

                while skip > skipped
                {
                    match br.read_line(&mut String::new()) 
                    {
                        Err(_) => return None,
                        Ok(_)  => skipped += 1
                    }
                }

                if let Some(lp) = LineTextReader::create_parser(schema, br, parser)
                {
                    return Some(ExecReader {
                        line_reader : lp
                    });
                }
            }
        }
        return None;
    }
}

impl <TParser : LineParser> Input for ExecReader<TParser> {
    fn determine_table_schema(&mut self) -> Option<TableSchema> { self.line_reader.determine_table_schema() }
    fn parse_next_row<'a>(&mut self, schema:&'a TableSchema) -> Option<Row<'a>> { self.line_reader.parse_next_row(schema) }
}

