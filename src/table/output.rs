/*
 * Copyright (C) 2018, Hao Hou
 *
 * The trait for all output plugin
 **/
use table::table::Table;

/**
 * @brief The result for the output plugin
 **/
#[allow(dead_code)]
pub enum OutputResult {
    /// When the output procedure is sucessfully done
    Success(),
    /// When something is wrong
    Fail()
}

impl OutputResult {
    /**
     * @brief monad bind
     * @param output The output object
     * @param how The continuation
     * @return the result
     **/
    pub fn then<T>(&self, output:&mut T, how:&Fn(&mut T) -> OutputResult) -> OutputResult
        where T : Output
    {
        match self {
            &OutputResult::Success() => how(output),
            &OutputResult::Fail()    => OutputResult::Fail()
        }
    }
}

/**
 * @brief The trait for a output plugin
 **/
pub trait Output {
    /**
     * @brief The result for the output plugin. Some plugin might procedure a string, this 
     *        can be used for the result data
     **/
    type IOResult;
    /**
     * @brief Write the table schema
     * @param table The table to write
     **/
    fn write_schema(&mut self, table:&Table) -> OutputResult;
    /**
     * @brief Scan the table before we actually started.
     * @note This is used when we want to adjust the column width
     * @return Result
     **/
    fn preprocess(&mut self, table:&Table) -> OutputResult;
    /**
     * @brief write a single row
     * @param table The table to write
     * @param idx The row index
     * @return write result
     **/
    fn write_record(&mut self, table:&Table, idx:usize) -> OutputResult;

    /**
     * @brief Get the output result
     * @return The output resut
     **/
    fn get_output_result(&mut self) -> Self::IOResult;
}
