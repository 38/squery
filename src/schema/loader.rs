// Copyright (C) 2018, Hao Hou
//
// The schema loader
extern crate regex;

use ::reader::svparser::SepValParser;

use std::fs::File;
use std::path::Path;
use std::io::Read;


/**
 * The object that is used to manage the schema files on the disk
 **/
pub struct SchemaManager{
    /// The schema search path
    schema_path_list : Vec<String>
}

/**
 * The line parser we are using to parse the result
 **/
#[allow(dead_code)]
pub enum Parser {
    SepVal(SepValParser)
}

/**
 * The data structure used to carry the schema query result
 **/
#[allow(dead_code)]
pub struct SchemaQueryResult {
    /// The schema of the command output
    schema : String,
    /// The line parser we should use
    line_parser: Parser
}

impl SchemaManager {
    pub fn new() -> SchemaManager
    {
        return SchemaManager {
            schema_path_list: Vec::new()
        };
    }

    pub fn push_schema_path(&mut self, path:String) -> &mut SchemaManager 
    {
        self.schema_path_list.push(path);
        return self;
    }

    pub fn query(&self, program : &str, _args : &[&str]) -> Option<SchemaQueryResult>
    {
        for ref path in &self.schema_path_list 
        {
            let rule_path = String::new() + path + "/" +program + ".yml";
            let path_obj = Path::new(&rule_path);
            if path_obj.exists() && path_obj.is_file()
            {
                if let Ok(mut file) = File::open(&rule_path)
                {
                    let mut content = String::new();

                    if let Ok(_) = file.read_to_string(&mut content)
                    {
                        /* TODO: parse the rule file */
                    }
                }
            }
        }

        return None;
    }
}
