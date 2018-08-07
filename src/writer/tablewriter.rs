/*
 * Copyright (C) 2018, Hao Hou
 *
 * The table based output
 */
use std::mem::swap;
use ::table::output::{Output, OutputResult};
use ::table::table::Table;

type Cell = String;

type Row = Vec<Cell>;

pub struct TableResult {
    header: Row,
    body  : Vec<Row>
}

impl TableResult {
    fn determine_layout(&self, _max_width:usize, min_width:usize) -> Vec<usize>
    {
        let mut ret = vec![0usize; self.header.len()];

        let mut total_length = ret.len() + 1;

        for idx in 0..self.header.len()
        {
            if self.header[idx].len() > ret[idx]
            {
                ret[idx] = self.header[idx].len()
            }
        }

        for r in &self.body 
        {
            for (idx, val) in r.into_iter().enumerate()
            {
                if val.len() > ret[idx]
                {
                    ret[idx] = val.len();
                }
            }
        }

        for col in &ret 
        {
            total_length += col;
        }

        /* TODO: what if the total_legnth is larger than the max_width ? */

        if total_length < min_width
        {
            let rem = min_width - total_length;

            for i in 0..ret.len()
            {
                ret[i] += rem / ret.len() + (if rem % ret.len() < i { 1 } else { 0 });
            }
        }

        return ret
    }

    fn print_hl(&self, layout:&Vec<usize>)
    {
        print!("+");
        for i in 0..layout.len() 
        {
            for _j in 0..layout[i]
            {
                print!("-");
            }
            print!("+");
        }
        print!("\n");
    }

    fn print_row(&self, layout:&Vec<usize>, data:&Row)
    {
        print!("|");
        let mut rem = Vec::<usize>::new();
        let mut total_rem = 0;

        for i in 0..layout.len() 
        {
            rem.push(data[i].len());
            total_rem += rem[i];
        }

        while total_rem > 0
        {
            for i in 0..layout.len()
            {
                let width = layout[i];
                let text  = &data[i];

                let current = if width < rem[i] { width } else { rem[i] };
                let left = (width - current) / 2;
                let right = width - left - current;

                for _ in 0..left 
                {
                    print!(" ");
                }

                print!("{}", &text[(text.len() - rem[i])..(text.len() - rem[i] + current)]);
                
                for _ in 0..right
                {
                    print!(" ");
                }

                print!("|");
                
                rem[i] -= current;
                total_rem -= current;

            }
            print!("\n");
        }
    }

    pub fn print_text_table(&self, max_width:usize, min_width:usize)
    {
        let layout = self.determine_layout(max_width, min_width);

        self.print_hl(&layout);
        self.print_row(&layout, &self.header);
        self.print_hl(&layout);

        for row in &self.body 
        {
            self.print_row(&layout, row);
            self.print_hl(&layout);
        }
    }
}

pub struct TableOutputer {
    result : Option<TableResult>,
}

impl TableOutputer {
    pub fn create() -> TableOutputer
    {
        return TableOutputer {
            result: Some(TableResult {
                header : Vec::new(),
                body   : Vec::new()
            })
        };
    }
}

impl Output for TableOutputer {
    type IOResult = TableResult;

    fn write_schema(&mut self, table:&Table) -> OutputResult 
    {
        match &mut self.result
        {
            &mut None          => OutputResult::Fail(),
            &mut Some(ref mut result) => {
                let schema = table.schema;
                for &(ref name, _) in &schema.types 
                {
                    result.header.push(name.to_string());
                }
                OutputResult::Success()
            }
        }
    }

    fn preprocess(&mut self, _table:&Table) -> OutputResult 
    {
        return OutputResult::Success();
    }

    fn write_record(&mut self, table:&Table, idx:usize) -> OutputResult 
    {
        match &mut self.result
        {
            &mut None          => OutputResult::Fail(),
            &mut Some(ref mut result) => {
                let mut cur_row = Vec::new();
                for c in 0..table.num_columns()
                {
                    let cell_data = table.get_cell(idx, c);

                    cur_row.push(cell_data.to_human_readable());
                }
                result.body.push(cur_row);
                OutputResult::Success()
            }
        }
    }

    fn get_output_result(&mut self) -> TableResult 
    {
        let mut ret = None;

        swap(&mut self.result, &mut ret);

        ret.unwrap()
    }
}
