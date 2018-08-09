/*
 * Copyright (C) 2018, Hao Hou
 */
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::mem::swap;
use ::table::input::Input;
use ::table::schema::{TableSchema, PrimitiveSchema};
use ::table::row::Row;
use ::table::primitive::PrimitiveValueT;

/**
 * Represent the schema staus of this parser
 **/
enum SchemaStatus {
    /// We are still waiting for the schema spec
    Undeterminend,
    /// We just determined the schema
    Determined(TableSchema),
    /// We have passed out the schema already
    Passed
}

impl SchemaStatus {
    /**
     * Check if we need to parse the schema
     * @return result
     **/
    fn should_determine_schema(&self) -> bool 
    {
        match self 
        {
            &SchemaStatus::Undeterminend   => true,
            _                              => false
        }
    }
}

pub trait LineParser {
    /**
     * @brief Parse the next line into a vector of fields
     * @param s The string to parse
     * @param schema The schema we are using for current table
     * @return The parsed vector or None
     **/
    fn parse_next_line<'text, 'schema>(&self, s:&'text String, schema:&'schema TableSchema) -> Option<Vec<&'text str>>;
}

/**
 * @brief The line text parser is used to interpret the 
 *        line based text as a table
 **/
pub struct LineTextReader<T:Read, P: LineParser> {
    /// The file we want to read
    fp       : BufReader<T>,
    /// The schema 
    schema   : SchemaStatus,
    /// The line parser
    parser   : P
}

impl <T:Read, P: LineParser> LineTextReader<T, P> {
    /**
     * @brief Parse the schema from the first line of the input
     * @return The parse result
     **/
    fn parse_schema_from_input(&mut self) -> bool
    {
        let mut schema_line = String::new();
        if let Ok(_) = self.fp.read_line(&mut schema_line)
        {
            if let Some(schema) = TableSchema::from_spec(&schema_line) 
            {
                self.schema = SchemaStatus::Determined(schema);
                return true
            }
        }
        return false;
    }
    /**
     * @brief Create the default parser, which have n string columns 
     * @param n The number of columns
     * @param fp The data source pointer
     * @return The newly created line text reader
     **/
    #[allow(dead_code)]
    pub fn create_default(n:usize, fp: BufReader<T>, parser:P) -> LineTextReader<T, P>
    {
        return LineTextReader {
            fp        : fp,
            parser    : parser,
            schema    : SchemaStatus::Determined(TableSchema {
                sort_keys : Vec::new(),
                sorted    : false,
                types     : {
                    let mut vec = Vec::<(String, PrimitiveSchema)>::new();
                    for i in 0..n
                    {
                        let name = format!("field_{}", i);
                        vec.push((name, PrimitiveSchema::Str));
                    }
                    vec
                }
            })
        };
    }
    /**
     * @brief Create a new self-explain line text, which means the first line of the text is the 
     * schema description
     * @param fp The file we want to read from
     * @return The newly created reader
     **/
    #[allow(dead_code)]
    pub fn create_self_explain_parser(fp: BufReader<T>, parser:P) -> LineTextReader<T, P>
    {
        let ret = LineTextReader {
            fp         : fp,
            parser     : parser,
            schema     : SchemaStatus::Undeterminend 
        };
        return ret;
    }
    /**
     * @brief Create a line text parser
     * @param schema The schema string
     * @param fp The file pointer
     * @return The newly created parser
     **/
    #[allow(dead_code)]
    pub fn create_parser(schema:&String, fp: BufReader<T>, parser:P) -> Option<LineTextReader<T, P>>
    {
        if let Some(schema) = TableSchema::from_spec(schema)
        {
            return Some(LineTextReader {
                fp         : fp,
                parser     : parser,
                schema     : SchemaStatus::Determined(schema)
            });
        }
        return None;
    }

}

impl <T:Read, P: LineParser> Input for LineTextReader<T, P> {
    fn determine_table_schema(&mut self) -> Option<TableSchema>
    {
        if self.schema.should_determine_schema() && self.parse_schema_from_input()
        {
            self.schema = SchemaStatus::Passed;
            return None;
        }

        let mut ret = SchemaStatus::Passed;
        swap(&mut ret, &mut self.schema);

        if let SchemaStatus::Determined(schema) = ret
        {
            return Some(schema);
        }

        return None;
    }

    fn parse_next_row<'a>(&mut self, schema:&'a TableSchema) -> Option<Row<'a>>
    {
        fn try_parse<T,U>(row:&mut Row, idx:usize, parse:&Fn() -> Result<T,U>) -> bool
            where T: PrimitiveValueT<T>
        {
            if let Ok(value) = parse() 
            {
                return row.set(idx, value) 
            }
            return false;
        }

        let mut line = String::new();
        let mut row = Row::empty(schema);
        while let Ok(size) = self.fp.read_line(&mut line)
        {
            if line == "\n" 
            {
                continue;
            }

            if size == 0
            {
                return None;
            }

            let mut invalid = false;

            if let Some(result) = self.parser.parse_next_line(&line, schema)
            {
                for (field_idx, field_str) in result.iter().enumerate()
                {
                    if !match schema.field_type(field_idx){
                        &PrimitiveSchema::Int    => try_parse(&mut row, field_idx, &| | {field_str.parse::<i64>()}),
                        &PrimitiveSchema::Float  => try_parse(&mut row, field_idx, &| | {field_str.parse::<f64>()}),
                        &PrimitiveSchema::Str    => try_parse::<String, ()>(&mut row, field_idx, &| | {Ok(field_str.to_string())})
                    }
                    {
                        invalid = true;
                    }
                }
            } 

            if !invalid
            {
                break;
            }
        }
        return Some(row);
    }
}
