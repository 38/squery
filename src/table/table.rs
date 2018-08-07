/*
 * Copyright (C) 2018, Hao Hou
 *
 * The table object
 */
use table::schema::TableSchema;
use table::row::Row;
use table::output::{Output, OutputResult};
use table::primitive::PrimitiveData;

/**
 * @brief The table data struture
 * @todo add lazy evaluation support
 **/
#[allow(dead_code)]
pub struct Table<'cell, 'schema:'cell> {
    /// The table's schema
    pub schema : &'schema TableSchema,
    /// The row data
    rows   : Vec<Row<'cell, 'schema>>
}

impl <'cell, 'schema:'cell> Table <'cell, 'schema> {
    /**
     * @brief Create an empty table
     * @param schema The schema of the table
     * @return the newly create table
     **/
    #[allow(dead_code)]
    pub fn empty(schema: &'schema TableSchema) -> Table<'cell, 'schema> 
    {
        return Table {
            schema : schema,
            rows   : Vec::new()
        };
    }

    /**
     * @brief Get the number of columns in the table
     * @return The result
     **/
    pub fn num_columns(&self) -> usize
    {
        return self.schema.num_columns();
    }

    /**
     * @brief Get the number of rows in the table
     * @return The result
     **/
    #[allow(dead_code)]
    pub fn num_rows(&self) -> usize
    {
        return self.rows.len();
    }

    /**
     * @brief Get a cell from the table
     * @param row The row index
     * @param col The column index
     * @return The reference to the cell data
     **/
    pub fn get_cell(&self, row:usize, col:usize) -> &'cell PrimitiveData 
    {
        return self.rows[row].value_at(col);
    }

    /**
     * @brief append a new row to the table
     * @param row The row to append
     * @return if the operation success
     **/
    pub fn append(&mut self, row:Row<'cell, 'schema>) -> bool 
    {
        // TODO: check if the shema matches
        self.rows.push(row);
        return true;
    }

    /**
     * @brief Dump the table
     * @param output_handle The output plugin instance 
     * @return The output result
     **/
    #[allow(dead_code)]
    pub fn dump<TOut>(&self, output_handle:&mut TOut) -> Option<TOut::IOResult>
        where TOut : Output
    {
        match output_handle.write_schema(self).then(output_handle, &|output_handle|  
        {
            output_handle.preprocess(self).then(output_handle, &|output_handle| 
            {
                //for(i = 0; i < self.rows.len(); i ++)
                for i in 0..(self.rows.len())
                {
                    match output_handle.write_record(self, i) 
                    {
                        OutputResult::Fail()    => return OutputResult::Fail(),
                        OutputResult::Success() => OutputResult::Success()
                    };
                }

                return OutputResult::Success();
            })
        }) 
        {
            OutputResult::Fail() => None,
            OutputResult::Success() => Some(output_handle.get_output_result())
        }
    }

}
