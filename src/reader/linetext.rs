/*
 * Copyright (C) 2018, Hao Hou
 */
use std::fs::File;
use std::collections::HashSet;
//use ::table::input::Input;
//use ::table::schema::TableSchema;
#[allow(dead_code)]
pub enum SchemaType {
    AllString,
    FromInput,
    FromString
}

#[allow(dead_code)]
pub struct LineTextReader<'file, 'field_sep> {
    fp: &'file mut File,
    field_sep: &'field_sep HashSet<char> 
}
/*
impl <'file, 'sep> Input for LineTextReader<'file, 'sep> {
   fn determine_table_schema(&mut self) -> TableSchema
   {
       let mut ret = TableSchema {
       };
   }
}*/
