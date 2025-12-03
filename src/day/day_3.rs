#![allow(warnings)]
use std::collections::HashMap;

use crate::day::common::*;

const CFG_DAY: &str = "3";

pub fn solve() -> Result<(), AdventError> {
    //data
    let lines: Vec<String> =
        get_lines_as_vec_string(format!(r"ressources/day_{}", CFG_DAY).to_owned())?;

    let res = largest_inside_line(lines.clone());

    print_sol_1(res);

    let res = largest_inside_line_n(lines, 100);

    print_sol_2(res);

    Ok(())
}

pub fn largest_inside_line(lines: Vec<String>) -> u32 {
    let mut res = 0;
    for line in lines {
        let mut pos1 = 0;
        let mut c1 = 0;
        let mut c2 = 0;

        //println!("line {:?}", line);
        //Start
        for (p, c) in line.chars().enumerate() {
            let c_int = c.to_string().parse::<u32>().expect("Not a valid u32");
            if c_int > c1 {
                c1 = c_int;
                pos1 = p;
            }
        }

        // println!("c1 {:?} pos1 {:?}", c1, pos1);

        //End
        if pos1 + 1 == line.len() {
            //Left
            for c in line[..pos1].chars() {
                //println!("Left c {:?} ", c);
                let c_int = c.to_string().parse::<u32>().expect("Not a valid u32");
                if c_int > c2 {
                    c2 = c_int;
                }
                //println!("Left c2 {:?} ", c2);
            }
            res += (c2 * 10) + c1;
        } else {
            //Right
            for c in line[pos1 + 1..].chars() {
                //println!("Right c {:?} ", c);
                let c_int = c.to_string().parse::<u32>().expect("Not a valid u32");
                if c_int > c2 {
                    c2 = c_int;
                }
                //println!("Right c2 {:?} ", c2);
            }
            res += (c1 * 10) + c2;
        }
        //println!("res {:?} ", res);
    }

    res
}

pub fn largest_inside_line_n(lines: Vec<String>, n: usize) -> u64 {
    let to_select = 12;
    let max_len = n;
    let mut res = 0;

    for line in lines {
        //println!("line {:?}", line);
        let line_int: Vec<u64> = line
            .chars()
            .map(|c| c.to_string().parse::<u64>().expect("Not a valid u32"))
            .collect();

        let mut left_windows = 0;
        let mut right_windows = max_len - to_select + 1; // start at 89 (pick one inside) + 11

        let mut found: Vec<u64> = Vec::new();
        while found.len() < to_select {
            //println!(
            //    "line_int[left_windows..right_windows] {:?}",
            //    &line_int[left_windows..right_windows]
            //);
            let iter = line_int[left_windows..right_windows].iter();
            let (pos, max_of_window) = iter.enumerate().rev().max_by_key(|(v, &p)| p).unwrap();
            found.push(*max_of_window);
            //println!("pos {:?}    max_of_window {:?} ", pos, max_of_window);
            left_windows = left_windows + pos + 1;
            right_windows += 1;
        }
        let value = found.iter().fold(0, |acc, &numb| acc * 10 + numb);
        //println!("{:?}", value);
        res += value;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_ok() {
        let _lines =
            get_lines_as_vec_string(format!(r"ressources/test/day_{}", CFG_DAY).to_owned())
                .unwrap();
        //solve
        let checksum = largest_inside_line(_lines);
        assert_eq!(
            357,
            checksum,
            "{}",
            format!(
                r"\nTest data for partie 1.\nsee https://adventofcode.com/2025/day/{}",
                CFG_DAY
            )
        );
    }
    #[test]
    fn test_part_2_ok() {
        let _lines =
            get_lines_as_vec_string(format!(r"ressources/test/day_{}", CFG_DAY).to_owned())
                .unwrap();
        //solve
        let checksum = largest_inside_line_n(_lines, 15);
        assert_eq!(
            3121910778619,
            checksum,
            "{}",
            format!(
                r"\nTest data for partie 1.\nsee https://adventofcode.com/2025/day/{}",
                CFG_DAY
            )
        );
    }
}
