use std::cmp;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

use crate::row::Row;
#[derive(Default)]
pub struct Document{
    rows: Vec<Row>
}

impl Document {
    pub fn open(filepath: String) -> std::io::Result<Self> {
        let file = match File::open(&filepath) {
            Ok(f) => f,
            Err(_) => {
                match File::create(filepath) {
                    Ok(f) => f,
                    Err(_) => panic!("Unable to open or create file")
                }
            }
        };
        let reader = BufReader::new(file);

        let mut rows: Vec<Row> = vec![];

        for line in reader.lines() {
            if let Ok(str) = line {
                rows.push(Row::from(str));
            }
        }

        Ok(Self {rows})
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn insert_into_row(&mut self, row_index: usize, column_index: usize, c: char){
        if row_index < self.rows.len() && column_index < self.rows[row_index].len(){
            self.rows[row_index].insert_char(column_index, c);
        }
    }

    pub fn remove_from_row(&mut self, row_index: usize, column_index: usize){
        if row_index < self.rows.len(){
            if column_index < self.rows[row_index].len(){
                self.rows[row_index].remove_char(column_index);
            } else {
                let len = self.rows[row_index].len();
                self.rows[row_index].remove_char(len - 1);
            }
        }
    }

    pub fn remove_and_append_to_previous_row(&mut self, row_index: usize){
        if row_index > 0 && row_index < self.rows.len(){
            let curr_row = self.rows[row_index].clone();
            self.rows[row_index - 1] += curr_row;
            self.rows.remove(row_index);
        }
    }

    pub fn remove_row(&mut self, row_index: usize){
        if row_index < self.rows.len() {
            self.rows.remove(row_index);
        }
    }

    pub fn add_row(&mut self, row_index: usize){
        let row_index = cmp::min(row_index, self.rows.len() - 1);
        self.rows.insert(row_index, Row::default());
    }

    pub fn line_count(&self) -> usize {
        self.rows.len()
    }
    
    pub fn row_len(&self, row_index: usize) -> usize{
        if row_index < self.rows.len() {
            self.rows[row_index].len()
        } else {
            0
        }
    }
}