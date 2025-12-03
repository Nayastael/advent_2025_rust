#![allow(warnings)]
use crate::day::common::*;

const CFG_DAY: &str = "13";

pub fn solve() -> Result<(), AdventError> {
    //data
    let lines = get_lines_as_vec_string(format!(r"ressources/day_{}", CFG_DAY).to_owned())?;

    let res = todo_1();

    print_sol_1(res);

    print_sol_2(res);

    Ok(())
}

pub fn todo_1() -> u32 {
    let mut res = 0;

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_1_ok() {
        let _lines =
            get_lines_as_vec_string(format!(r"ressources/test/day_{}", CFG_DAY).to_owned())
                .unwrap();
        //solve
        let checksum = 0;
        assert_eq!(
            22,
            checksum,
            "{}",
            format!(
                r"\nTest data for partie 1.\nsee https://adventofcode.com/2025/day/{}",
                CFG_DAY
            )
        );
    }
}
