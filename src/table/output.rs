/*
 * Copyright (C) 2018, Hao Hou
 *
 * The trait for all output plugin
 **/
use table::table::Table;

#[allow(dead_code)]
pub enum OutputResult {
    Success(),
    Fail()
}

impl OutputResult {
    pub fn then<T>(&self, output:&mut T, how:&Fn(&mut T) -> OutputResult) -> OutputResult
        where T : Output
    {
        match self {
            &OutputResult::Success() => how(output),
            &OutputResult::Fail()    => OutputResult::Fail()
        }
    }
}

pub trait Output {
    type IOResult;
    fn write_schema(&mut self, table:&Table) -> OutputResult;
    fn preprocess(&mut self, table:&Table) -> OutputResult;
    fn write_record(&mut self, table:&Table, idx:usize) -> OutputResult;
    fn get_output_result(&mut self) -> Self::IOResult;
}
