/*
 * Copyright (C) 2018, Hao Hou
 *
 * The implmenetation of the table schema
 */

/**
 * @brief The primitive colomn types
 * @todo Add date support
 **/
#[derive(Debug)]
#[allow(dead_code)]
#[derive(PartialEq)]
pub enum PrimitiveSchema {
    /// The column is an integer
    Int, 
    /// The column is a float
    Float, 
    /// The column is a string
    Str
}

/**
 * @brief Represent a table schema 
 **/
#[allow(dead_code)]
pub struct TableSchema { 
    /// The list of key index that we used for sorting the table
    pub sort_keys: Vec<usize>,
    /// If the table is naturally sorted
    pub sorted: bool,
    /// The actual type definition
    pub types:Vec<(String, PrimitiveSchema)>
}

impl TableSchema {
    /**
     * @brief Get the number of columns of the table
     * @return The number of columns
     **/
    pub fn num_columns(&self) -> usize
    {
        return self.types.len();
    }

    /**
     * @brief Check if the schema assignment make sense
     * @param idx The cell index
     * @param schema The type we have got 
     * @return the check result
     **/
    pub fn check_schema(&self, idx: usize, schema:PrimitiveSchema) -> bool 
    {
        if self.types.len() <= idx
        {
            return false;
        }

        return self.types[idx].1 == schema;
    }
}


