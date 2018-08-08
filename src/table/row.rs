/*
 * Copyright (C) Hao Hou, 2018
 *
 * The implementation of table row data
 **/
use table::schema::TableSchema;
use table::primitive::{PrimitiveData, PrimitiveValueT};

/**
 * @brief The data structure used to represent one row
 **/
pub struct Row<'cell, 'schema:'cell> {
    /// The schema we are using
    schema: &'schema TableSchema,
    /// The actual column data
    column_data: Vec<PrimitiveData<'cell>>
}

impl <'cell, 'schema> Row<'cell, 'schema> {

    /**
     * @brief check if the schema is the expected schema
     * @param schema The expected schema
     * @return check result
     **/
    pub fn validate_schema(&self, schema:&TableSchema) -> bool
    {

        return schema == self.schema;
    }
    /**
     * @brief Access the value of the column
     * @param idx The index to access
     * @return The reference to the Primitve data
     **/
    pub fn value_at(&self, idx:usize) -> &PrimitiveData<'cell>
    {
        return &self.column_data[idx];
    }

    /**
     * @brief Construct an empty JSON schema 
     * @param schema The table schema we want to use 
     * @return The newly created row data
     **/
    pub fn empty(schema:&TableSchema) -> Row 
    {
        let mut ret = Row {
            schema: schema,
            column_data: Vec::<PrimitiveData>::with_capacity(schema.num_columns())
        };

        ret.column_data.resize(schema.num_columns(), Default::default());

        return ret;
    }

    /**
     * @brief Assign a value to the column in this row
     * @param idx The column index
     * @param val The value
     * @return If this operation success
     **/
    pub fn set<T>(&mut self, idx: usize, val: T) -> bool
        where T: PrimitiveValueT<'cell, T> 
    {
        if self.schema.check_schema(idx, T::schema_type())
        {
            self.column_data[idx] = T::to_primitive_value(val);
            return true;
        }
        
        return false;
    }
}
