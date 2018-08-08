/*
 * Copyright (C) 2018, Hao Hou
 *
 * The table object
 */
use std::iter::Iterator;
use std::mem::swap;
use std::mem::transmute;
use table::schema::TableSchema;
use table::row::Row;
use table::output::{Output, OutputResult};
use table::primitive::PrimitiveData;
use table::input::Input;

/**
 * @brief Represent how we can get the data for the table
 *        if this is empty, the data is fully loaded to memory,
 *        thus we are ready to performe random access
 **/
pub enum TableDataSource<'parser> {
    /// No input
    Empty,
    /// Use a parser as Input, the second bool represent if we need to keep all the data we have
    /// read
    Parser(&'parser mut Input, bool)
}

impl <'a> TableDataSource<'a> {
    pub fn is_empty(&self) -> bool
    {
        match self 
        {
            &TableDataSource::Empty => true,
            _     => false
        }
    }
}

/**
 * @brief The table data struture
 * @todo add lazy evaluation support
 **/
#[allow(dead_code)]
pub struct Table<'schema, 'parser> {
    /// The table's schema
    pub schema : &'schema TableSchema,
    /// The row data
    rows   : Vec<Row<'schema>>,
    /// The data source we want to use
    data_source: TableDataSource<'parser>,
    /// The cursor for the seq access 
    cursor:usize
}

pub struct TableRandomAccessor<'table, 'schema:'table, 'parser:'table> {
    table:&'table mut Table<'schema, 'parser>
}

impl <'table, 'schema:'table, 'parser:'table> Iterator for &'table mut Table<'schema, 'parser> {
    type Item = &'table Row<'schema>;

    fn next<'a>(&'a mut self) -> Option<Self::Item>
    {
        if self.data_source.is_empty() 
        {
            if self.cursor >= self.rows.len()
            {
                return None;
            }

            self.cursor += 1;

            return Some(unsafe{
                transmute::<&'a Row<'schema>, Self::Item>(&self.rows[self.cursor - 1])
            });
        }
        else
        {
            if let &mut TableDataSource::Parser(ref mut parser, ref _keep_used) = &mut self.data_source
            {
                if let Some(new_row) = parser.parse_next_row(self.schema)
                {
                    if 0 == self.rows.len() || *_keep_used
                    {
                        self.rows.push(new_row);
                    }
                    else
                    {
                        self.rows[0] = new_row;
                    }
                    return Some(unsafe {
                        transmute::<&'a Row<'schema>, Self::Item>(&self.rows[self.rows.len() - 1])
                    });
                }
                return None;
            }
            return None;
        }
    }

}

impl <'table, 'schema:'table, 'parser:'table> TableRandomAccessor<'table, 'schema, 'parser> {
    /**
     * @brief Get the number of rows in the table
     * @return The result
     **/
    #[allow(dead_code)]
    pub fn num_rows(&self) -> usize
    {
        return self.table.rows.len();
    }

    /**
     * @brief Get a cell from the table
     * @param row The row index
     * @param col The column index
     * @return The reference to the cell data
     **/
    pub fn get_cell<'a>(&'a self, row:usize, col:usize) -> Option<&'a PrimitiveData>
    {
        return Some(self.table.rows[row].value_at(col));
    }

    /**
     * @brief append a new row to the table
     * @param row The row to append
     * @return if the operation success
     **/
    pub fn append(&mut self, row:Row<'schema>) -> bool 
    {
        if row.validate_schema(self.table.schema)
        {
            self.table.rows.push(row);
            return true;
        }
        return false;
    }
}

impl <'schema, 'parser> Table <'schema, 'parser> {
    /**
     * @brief Create an empty table
     * @param schema The schema of the table
     * @return the newly create table
     **/
    #[allow(dead_code)]
    pub fn empty(schema: &'schema TableSchema, data_source : TableDataSource<'parser>) -> Table<'schema, 'parser> 
    {
        return Table {
            schema : schema,
            rows   : Vec::new(),
            data_source : data_source,
            cursor : 0
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
     * @brief Dump the table
     * @param output_handle The output plugin instance 
     * @return The output result
     **/
    #[allow(dead_code)]
    pub fn dump<TOut>(&mut self, output_handle:&mut TOut) -> Option<TOut::IOResult>
        where TOut : Output
    {
        match output_handle.write_schema(self).then(self, output_handle, &|this ,output_handle|  
        {
            output_handle.preprocess(this).then(this, output_handle, &|this, output_handle| 
            {
                match output_handle.write_records(this) 
                {
                    OutputResult::Fail()    => return OutputResult::Fail(),
                    OutputResult::Success() => OutputResult::Success()
                };

                return OutputResult::Success();
            })
        }) 
        {
            OutputResult::Fail()    => None,
            OutputResult::Success() => Some(output_handle.get_output_result())
        }
    }

    /**
     * @brief Get the random accessor
     * @return The table accessor created from the table
     **/
    pub fn get_random_accessor<'a>(&'a mut self) -> TableRandomAccessor<'a, 'schema, 'parser>
    {
        if !self.data_source.is_empty()
        {
            if let &mut TableDataSource::Parser(ref mut parser, ref _keep_used) = &mut self.data_source
            {
                loop 
                {
                    if let Some(row) = parser.parse_next_row(self.schema)
                    {
                        self.rows.push(row);
                    }
                    else 
                    {
                        break;
                    }
                }

            }
            
            let mut new_val = TableDataSource::Empty;

            swap(&mut new_val, &mut self.data_source);
        }

        return TableRandomAccessor {
            table : self
        };
    }

}
