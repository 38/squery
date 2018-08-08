/*
 * Copyright (C) 2018, Hao Hou
 */
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::collections::HashSet;
use std::mem::swap;
use ::table::input::Input;
use ::table::schema::{TableSchema, PrimitiveSchema};
use ::table::row::Row;
use ::table::primitive::PrimitiveValueT;

#[allow(dead_code)]
pub struct LineTextReader<'file, T:Read + 'file> {
    fp       : &'file mut BufReader<T>,
    field_sep: HashSet<char>,
    schema   : Option<TableSchema>
}

impl <'file, T:Read + 'file> LineTextReader<'file, T> {
    fn parse_schema_from_input(&mut self) -> Option<&TableSchema>
    {
        let mut schema_line = String::new();
        if let Ok(_) = self.fp.read_line(&mut schema_line)
        {
            if let Some(schema) = TableSchema::from_spec(&schema_line) 
            {
                self.schema = Some(schema);
                return self.schema.as_ref();
            }
        }
        return None;
    }
    #[allow(dead_code)]
    pub fn create_default(n:usize, fp: &'file mut BufReader<T>) -> LineTextReader<'file, T>
    {
        return LineTextReader {
            fp        : fp,
            field_sep : HashSet::<char>::new(),
            schema    : Some(TableSchema {
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
    #[allow(dead_code)]
    pub fn create_self_explain_parser(fp: &'file mut BufReader<T>) -> LineTextReader<'file, T>
    {
        let ret = LineTextReader {
            fp         : fp,
            field_sep  : HashSet::<char>::new(),
            schema     : None
        };
        return ret;
    }
    #[allow(dead_code)]
    pub fn create_parser(schema:&String, fp: &'file mut BufReader<T>) -> LineTextReader<'file, T>
    {
        return LineTextReader {
            fp         : fp,
            field_sep  : HashSet::<char>::new(),
            schema     : TableSchema::from_spec(schema)
        };
    }

    pub fn add_field_sep(&mut self, ch:char)
    {
        self.field_sep.insert(ch);
    }
}

impl <'file, T:Read + 'file> Input for LineTextReader<'file, T> {
    fn determine_table_schema(&mut self) -> Option<TableSchema>
    {
        if self.schema.is_none()
        {
            self.parse_schema_from_input();
        }
        let mut ret = None as Option<TableSchema>;
        swap(&mut ret, &mut self.schema);
        return ret;
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

        let mut field_idx = 0;
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

            let mut to_parse = &line[0..];
            while field_idx < schema.types.len() && to_parse.len() > 0
            {
                while to_parse.len() > 0
                {
                    let (head, tail) = to_parse.split_at(1);
                    if !self.field_sep.contains(&head.chars().next().unwrap())
                    {
                        break;
                    }
                    to_parse = tail
                }

                let mut idx = 0;
                let mut field = None as Option<&str>;
                while to_parse.len() >= idx
                {
                    let (first, rem) = to_parse.split_at(idx);
                    if rem.len() == 0 || self.field_sep.contains(&rem.chars().next().unwrap())
                    {
                        field = Some(first);
                        to_parse = rem;
                        break;
                    }
                    idx += 1
                }

                if let Some(field_str) = field 
                {
                    if match schema.field_type(field_idx){
                        &PrimitiveSchema::Int    => try_parse(&mut row, field_idx, &| | {field_str.parse::<i64>()}),
                        &PrimitiveSchema::Float  => try_parse(&mut row, field_idx, &| | {field_str.parse::<f64>()}),
                        &PrimitiveSchema::Str    => try_parse::<String, ()>(&mut row, field_idx, &| | {Ok(field_str.to_string())})
                    }
                    {
                        field_idx += 1;
                        continue;
                    }
                    else
                    {
                        return None;
                    }
                }
                else
                {
                    break;
                }
            }
            break;
        }
        return Some(row);
    }
}
