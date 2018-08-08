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
pub enum PrimitiveData {
    /// There's nothing here
    Nothing(),
    /// We got an integer
    Int(i64),
    /// We got a float
    Float(f64),
    /// We got a string
    //StrRef(&'a String)
    Str(String)
}

impl PrimitiveData {
    pub fn to_human_readable(&self) -> String
    {
        match self {
            &PrimitiveData::Nothing()   => "".to_string(),
            &PrimitiveData::Int(what)   => format!("{}", what),
            &PrimitiveData::Float(what) => format!("{}", what),
            &PrimitiveData::Str(ref what)   => what.clone()
        }
    }
}

impl Default for PrimitiveData {
    fn default() -> PrimitiveData
    {
        return PrimitiveData::Nothing();
    }
}

pub trait PrimitiveValueT<T> {
    fn to_primitive_value(val:T) -> PrimitiveData;
    fn schema_type() -> PrimitiveSchema;
}

impl PrimitiveValueT<i64> for i64 {
    fn to_primitive_value(val:i64) -> PrimitiveData {  PrimitiveData::Int(val) }
    fn schema_type() -> PrimitiveSchema { PrimitiveSchema::Int }
}

impl PrimitiveValueT<f64> for f64 {
    fn to_primitive_value(val:f64) -> PrimitiveData { PrimitiveData::Float(val) }
    fn schema_type() -> PrimitiveSchema { PrimitiveSchema::Float }
}

/*
impl <'b, 'a:'b>  PrimitiveValueT<'b, &'a String> for &'a String {
    fn to_primitive_value(val:&'a String) -> PrimitiveData<'b> { PrimitiveData::Str(val) }
    fn schema_type() -> PrimitiveSchema { PrimitiveSchema::Str }
}*/
impl PrimitiveValueT<String> for String {
    fn to_primitive_value(val:String) -> PrimitiveData { PrimitiveData::Str(val) }
    fn schema_type() -> PrimitiveSchema { PrimitiveSchema::Str }
}

