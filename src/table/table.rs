/*
 * Copyright (C) 2018, Hao Hou
 *
 * The table object
 */
use table::schema::TableSchema;
use table::row::Row;
use table::output::{Output, OutputResult};
use table::primitive::PrimitiveData;

#[allow(dead_code)]
pub struct Table<'cell, 'schema:'cell> {
    schema : &'schema TableSchema,
    rows   : Vec<Row<'cell, 'schema>>
}

impl <'cell, 'schema:'cell> Table <'cell, 'schema> {
    #[allow(dead_code)]
    pub fn empty(schema: &'schema TableSchema) -> Table<'cell, 'schema> 
    {
        return Table {
            schema : schema,
            rows   : Vec::new()
        };
    }

    pub fn num_colmns(&self) -> usize
    {
        return self.schema.num_columns();
    }

    #[allow(dead_code)]
    pub fn num_rows(&self) -> usize
    {
        return self.rows.len();
    }

    pub fn get_cell(&self, row:usize, col:usize) -> &'cell PrimitiveData 
    {
        return self.rows[row].value_at(col);
    }

    pub fn append(&mut self, row:Row<'cell, 'schema>) -> bool 
    {
        // TODO: check if the shema matches
        self.rows.push(row);
        return true;
    }

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
