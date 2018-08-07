/*
 * Copyright (C) 2018, Hao Hou
 */
use table::row::Row;
use table::schema::TableSchema;

pub trait Input {
    fn determine_table_schema(&mut self) -> TableSchema;
    fn parse_next_row(&mut self) -> Option<Row>;
}

