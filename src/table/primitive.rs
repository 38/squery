/*
 * Copyright (C) 2018, Hao Hou
 *
 * The primitive data for a table
 **/

use std::default::Default;
use table::schema::PrimitiveSchema;

/**
 * @brief The data type for a primitive data
 * @todo implement the data support
 **/
#[derive(Debug, Clone)]
pub enum PrimitiveData<'a> {
    /// There's nothing here
    Nothing(),
    /// We got an integer
    Int(i64),
    /// We got a float
    Float(f64),
    /// We got a string
    Str(&'a String)
}

impl <'a> Default for PrimitiveData<'a> {
    fn default() -> PrimitiveData<'a>
    {
        return PrimitiveData::Nothing();
    }
}

pub trait PrimitiveValueT<'a, T> {
    fn to_primitive_value(val:T) -> PrimitiveData<'a>;
    fn schema_type() -> PrimitiveSchema;
}

impl <'a> PrimitiveValueT<'a, i64> for i64 {
    fn to_primitive_value(val:i64) -> PrimitiveData<'a> {  PrimitiveData::Int(val) }
    fn schema_type() -> PrimitiveSchema { PrimitiveSchema::Int }
}

impl <'a> PrimitiveValueT<'a, f64> for f64 {
    fn to_primitive_value(val:f64) -> PrimitiveData<'a> { PrimitiveData::Float(val) }
    fn schema_type() -> PrimitiveSchema { PrimitiveSchema::Float }
}

impl <'b, 'a:'b>  PrimitiveValueT<'b, &'a String> for &'a String {
    fn to_primitive_value(val:&'a String) -> PrimitiveData<'b> { PrimitiveData::Str(val) }
    fn schema_type() -> PrimitiveSchema { PrimitiveSchema::Str }
}

