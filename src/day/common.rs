pub use crate::errors::AdventError;
pub use itertools::{self, Itertools};
pub use std::fmt::Debug;
pub use std::fs::File;
use std::io::Read;
pub use std::io::{BufRead, BufReader};
pub use std::fmt;

pub fn get_lines_as_vec_string(path: String) -> Result<Vec<String>, AdventError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut list = Vec::new();
    for line in reader.lines() {
        list.push(line?);
    }
    Ok(list)
}

pub fn get_content_as_string(path: String) -> Result<String, AdventError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    //let mut list = Vec::new();
    //for line in reader.lines() {
    //    list.push(line?);
    //}
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_map_char(path: String) -> Result<Vec<Vec<char>>, AdventError> {
    let lines = get_lines_as_vec_string(path)?;
    let mut list = Vec::new();
    for line in lines {
        list.push(line.chars().collect());
    }
    Ok(list)
}

pub fn get_map_i32(path: String) -> Result<Vec<Vec<i32>>, AdventError> {
    let lines = get_lines_as_vec_string(path)?;
    let mut list = Vec::new();
    for line in lines {
        list.push(
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }
    Ok(list)
}

pub fn get_line(path: String) -> Result<Vec<char>, AdventError> {
    let lines = get_lines_as_vec_string(path)?;
    Ok(lines.first().unwrap().chars().collect())
}

pub fn get_line_split_ws_as_string(path: String) -> Result<Vec<String>, AdventError> {
    let lines = get_lines_as_vec_string(path)?;
    let temp: Vec<&str> = lines.first().unwrap().split_whitespace().collect();
    let mut res: Vec<String> = vec![];
    for elem in temp {
        res.push(elem.to_string());
    }
    Ok(res)
}

pub fn print_sol_1<T: Debug>(sol: T) {
    println!("Part 1 solution : {:?}", sol);
}

pub fn print_sol_2<T: Debug>(sol: T) {
    println!("Part 2 solution : {:?}", sol);
}


#[derive(Debug, Default, Clone)]
struct _Pos {
    val: i32,
    i: usize,
    j: usize,
}

impl _Pos {
    fn _new(val: i32, pos: (usize, usize)) -> _Pos {
        _Pos {
            val: val,
            i: pos.0,
            j: pos.1,
        }
    }
    fn _north(self) -> (usize, usize) {
        (self.i - 1, self.j)
    }
    fn _south(self) -> (usize, usize) {
        (self.i + 1, self.j)
    }
    fn _east(self) -> (usize, usize) {
        (self.i, self.j + 1)
    }
    fn _west(self) -> (usize, usize) {
        (self.i, self.j - 1)
    }
}
