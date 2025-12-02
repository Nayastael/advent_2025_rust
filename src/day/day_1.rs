#![allow(warnings)]
use crate::day::common::*;

const CFG_DAY: &str = "1";

pub fn solve() -> Result<(), AdventError> {
    //data
    let lines = get_lines_as_vec_string(format!(r"ressources/day_{}", CFG_DAY).to_owned())?;

    Ok(())
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
