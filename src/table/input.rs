/*
 * Copyright (C) 2018, Hao Hou
 */
use table::row::Row;
use table::schema::TableSchema;

pub trait Input{
    fn determine_table_schema(&mut self) -> Option<TableSchema>;
    fn parse_next_row<'schema> (&mut self, schema:&'schema TableSchema) -> Option<Row<'schema>>;
}

