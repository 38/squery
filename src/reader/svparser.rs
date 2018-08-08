/*
 * Copyright (C) 2018, Hao Hou
 */
use std::collections::HashSet;
use reader::linetext::LineParser;
use ::table::schema::TableSchema;

pub struct SepValParser {
    /// The set of field seperators
    field_sep: HashSet<char>
}

impl SepValParser {
    pub fn create(delim:&String) -> SepValParser 
    {
        let mut ret = SepValParser {
            field_sep : HashSet::<char>::new()
        };

        for ch in delim.chars()
        {
            ret.field_sep.insert(ch);
        }

        return ret;
    }
}


impl LineParser for SepValParser {

    fn parse_next_line<'text, 'schema>(&self, line:&'text String, schema:&'schema TableSchema) -> Option<Vec<&'text str>>
    {
        let mut ret = Vec::<&str>::new(); 
        let mut to_parse = &line[0..];
        let mut field_idx = 0;
        while field_idx < schema.types.len() && to_parse.len() > 0
        {
            while to_parse.len() > 0
            {
                let (head, tail) = to_parse.split_at(1);
                if !self.field_sep.contains(&head.chars().next().unwrap())
                {
                    break;
                }
                to_parse = tail
            }

            let mut idx = 0;
            let mut field = None as Option<&str>;
            while to_parse.len() >= idx
            {
                let (first, rem) = to_parse.split_at(idx);
                if rem.len() == 0 || self.field_sep.contains(&rem.chars().next().unwrap())
                {
                    field = Some(first);
                    to_parse = rem;
                    break;
                }
                idx += 1
            }

            if let Some(field_value) = field
            {
                ret.push(field_value);
                field_idx += 1;
            }
            else
            {
                break;
            }
        }

        return Some(ret);
    }
}
