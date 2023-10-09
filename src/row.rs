use std::{cmp, ops::AddAssign};

#[derive(Default, Clone)]
pub struct Row {
    string: String
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self { string: value.to_string() }
    } 
}

impl From<String> for Row {
    fn from(value: String) -> Self {
        Self { string: value }
    } 
}

impl AddAssign for Row {
    fn add_assign(&mut self, rhs: Self) {
        self.string += &rhs.string;
    }
}

impl Row {
    pub fn render(&self, begin: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let begin = cmp::min(begin, end);
        self.string.get(begin..end).unwrap_or_default().to_string()
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }

    pub fn insert_char(&mut self, column_index: usize, c: char){
        let first_half = &self.string[0..column_index];
        let second_half = &self.string[column_index..];

        let mut new_string: String = first_half.to_string();
        new_string += &c.to_string();
        new_string += second_half;

        self.string = new_string.clone();
    }

    pub fn remove_char(&mut self, column_index: usize){
        let first_half = &self.string[0..column_index];
        let second_half = &self.string[column_index + 1..];

        let mut new_string: String = first_half.to_string();
        new_string += second_half;

        self.string = new_string.clone();
    }

}