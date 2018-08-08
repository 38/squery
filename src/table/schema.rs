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
#[derive(Debug)]
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

    /**
     * @brief Parse a schema specificication from the string
     * @param spec The schema specification
     * @note The specicication's format defined as follow
     *           .name1:type1 [.name2:type2 .... .nameN:typeN] [sort|sorted:key1,key2...,keyM]
     * @return The newly created schema
     **/
    pub fn from_spec(spec:&String) -> Option<TableSchema> 
    {
        fn parse_ws(s:&str) -> &str
        {
            let mut ret = s;
            while ret.len() > 0
            {
                let (head, tail) = ret.split_at(1);
                if head != "\r" && head != "\n" && head != " " && head != "\t"
                {
                    break;
                }

                ret = tail;
            }
            return ret;
        }

        fn expect<'a, 'b>(s:&'a str, what:&'b str) -> Option<&'a str>
        {
            if s.len() < what.len()
            {
                return None;
            }

            let (head,tail) = s.split_at(what.len());
            if head == what
            {
                return Some(tail);
            }

            return None;
        }

        fn parse_token<'a,'b>(s:&'a str, leading:&'b str) -> Option<(&'a str,&'a str)>
        {
            if let Some(mut begin) = expect(parse_ws(s), leading)
            {
                begin = parse_ws(begin);
                let mut to_parse = begin;
                let mut sp = 0;
                while to_parse.len() > 0
                {
                    let (head, tail) = to_parse.split_at(1);

                    if head != ":" && head != "\r" && head != "\n" && head != "\t" && head != " " && head != "." && head != ","
                    {
                        sp += 1;
                        to_parse = tail;
                    }
                    else
                    {
                        break;
                    }
                }

                return Some(begin.split_at(sp));
            }
            return None;
        }

        fn parse_field_name(s:&str) -> Option<(&str, &str)>
        {
            return parse_token(s, ".");
        }

        fn parse_field_type(s:&str) -> Option<(PrimitiveSchema, &str)>
        {
            if let Some((typename, next)) = parse_token(s, ":")
            {
                return match typename 
                {
                    "Int"    => Some((PrimitiveSchema::Int, next)),
                    "Float"  => Some((PrimitiveSchema::Float, next)),
                    "String" => Some((PrimitiveSchema::Str, next)),
                    _        => None
                }
            }
            else
            {
                return None;
            }
        }

        fn parse_field_schema(s:&str) -> Option<((String, PrimitiveSchema), &str)>
        {
            if let Some((name, next)) = parse_field_name(s) 
            {
                if let Some((ptype, next)) = parse_field_type(next)
                {
                    return Some(((name.to_string(), ptype), next));
                }
            }
            return None;
        }

        fn parse_field_schema_list(s:&str) -> (Vec<(String, PrimitiveSchema)>, &str)
        {
            let mut to_parse = s;
            let mut ret = Vec::<(String, PrimitiveSchema)>::new();
            while let Some((field_schema, next)) = parse_field_schema(to_parse)
            {
                ret.push(field_schema);
                to_parse = next;
            }
            return (ret, to_parse); 
        }

        fn parse_key_list<'a,'b>(s:&'a str, fields:&'b Vec<(String, PrimitiveSchema)>) -> Option<(Vec<usize>, &'a str)>
        {
            let mut ret = Vec::<usize>::new();
            let mut first_time = true;
            let mut to_parse = s;
            while let Some((field_name, next)) = parse_token(to_parse, if first_time {":"} else {","})
            {
                first_time = false;

                let mut found = false;

                for (idx, &(ref name, _)) in fields.iter().enumerate()
                {
                    if name == field_name
                    {
                        found = true;
                        ret.push(idx);
                        break;
                    }
                }

                if !found
                {
                    return None;
                }

                to_parse = next;
            }

            return Some((ret, to_parse));
        }

        fn parse_sort_keys<'a,'b>(s:&'a str, schema:&'b mut TableSchema) -> Option<&'a str>
        {
            if let Some((keyword, next)) = parse_token(s, "")
            {
                match keyword 
                {
                    "sort"   =>   { schema.sorted = false; }
                    "sorted" =>   { schema.sorted = true; }
                    _        =>   { return None; }
                }

                if let Some((keys, mut rem)) = parse_key_list(next, &schema.types)
                {
                    schema.sort_keys = keys;
                    return Some(parse_ws(rem));
                }
            }

            return None;
        }

        let (schema_list, next) = parse_field_schema_list(spec);
        
        let mut ret = TableSchema {
            sort_keys : Vec::new(),
            sorted    : false,
            types     : schema_list
        };

        return match parse_sort_keys(next, &mut ret)
        {
            Some("") => Some(ret),
            _        => None
        }
    }
}

impl PartialEq for TableSchema {
    fn eq(&self, rhs: &TableSchema) -> bool 
    {
        if self as * const  _ == rhs as * const _
        {
            return true;
        }

        if self.sorted != rhs.sorted ||
           self.sort_keys != rhs.sort_keys ||
           self.types != rhs.types 
        {
            return false;
        }

        return true;
    }
}


