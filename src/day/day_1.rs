#![allow(warnings)]
use crate::day::common::*;

const CFG_DAY: &str = "1";

pub fn solve() -> Result<(), AdventError> {
    //data
    let lines = get_lines_as_vec_string(format!(r"ressources/day_{}", CFG_DAY).to_owned())?;

    let res = count_0(lines.clone());
    print_sol_1(res);
    let res = count_0_0x434C49434B(lines);
    print_sol_2(res);

    Ok(())
}

pub fn count_0(lines: Vec<String>) -> u32 {
    let mut pos: i32 = 50;
    let mut res = 0;
    for mut line in lines {
        match line.remove(0) {
            'R' => {
                let mut num: i32 = line.parse().expect("Not a valid i32");
                while num > 0 {
                    pos += 1;
                    num -= 1;
                    if pos > 99 {
                        pos = 0;
                    }
                }
            }
            'L' => {
                let mut num: i32 = line.parse().expect("Not a valid i32");
                while num > 0 {
                    pos -= 1;
                    num -= 1;
                    if pos < 0 {
                        pos = 99;
                    }
                }
            }
            _ => {}
        }
        if pos == 0 {
            res += 1;
        }
    }
    res
}

pub fn count_0_0x434C49434B(lines: Vec<String>) -> u32 {
    let mut pos: i32 = 50;
    let mut res = 0;
    for mut line in lines {
        match line.remove(0) {
            'R' => {
                let mut num: i32 = line.parse().expect("Not a valid i32");
                while num > 0 {
                    pos += 1;
                    num -= 1;
                    if pos > 99 {
                        pos = 0;
                        res += 1;
                    }
                }
            }
            'L' => {
                let mut num: i32 = line.parse().expect("Not a valid i32");
                while num > 0 {
                    pos -= 1;
                    num -= 1;
                    if pos < 0 {
                        pos = 99;
                        res += 1;
                    }
                }
            }
            _ => {}
        }
        // if pos == 0 {
        //     res += 1;
        // }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //#[ignore]
    fn test_part_1_ok() {
        let lines = get_lines_as_vec_string(format!(r"ressources/test/day_{}", CFG_DAY).to_owned())
            .unwrap();
        //solve
        let checksum = count_0(lines);
        assert_eq!(
            3,
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
        let lines = get_lines_as_vec_string(format!(r"ressources/test/day_{}", CFG_DAY).to_owned())
            .unwrap();
        //solve
        let checksum = count_0_0x434C49434B(lines);
        assert_eq!(
            6,
            checksum,
            "{}",
            format!(
                r"\nTest data for partie 2.\nsee https://adventofcode.com/2025/day/{}",
                CFG_DAY
            )
        );
    }
}
