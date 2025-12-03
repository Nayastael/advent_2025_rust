#![allow(warnings)]
use crate::day::common::*;

const CFG_DAY: &str = "2";

pub fn solve() -> Result<(), AdventError> {
    //data
    let content: String = get_content_as_string(format!(r"ressources/day_{}", CFG_DAY).to_owned())?;

    let res = count_invalid_ID(content.clone());
    print_sol_1(res);

    let res = count_invalid_ID_2(content);
    print_sol_2(res);
    Ok(())
}

pub fn count_invalid_ID(content: String) -> u64 {
    let mut res = 0;
    for pair in content.split(',') {
        let (r1, r2) = pair.split_once('-').unwrap();
        let mut r1: u64 = r1.parse::<u64>().expect("Not a valid u32");
        let r2: u64 = r2.parse::<u64>().expect("Not a valid u32");
        while r1 <= r2 {
            let id_s = r1.to_string();
            let id = id_s.as_str();
            let id_l = id.len();
            if id_l % 2 == 0 {
                if id[..id_l / 2] == id[id_l / 2..] {
                    res += id.parse::<u64>().expect("Not a valid u64");
                }
            }
            r1 += 1;
        }
    }

    res
}

pub fn count_invalid_ID_2(content: String) -> u64 {
    let mut res = 0;
    for pair in content.split(',') {
        let (r1, r2) = pair.split_once('-').unwrap();
        let r1: u64 = r1.parse::<u64>().expect("Not a valid u32");
        let r2: u64 = r2.parse::<u64>().expect("Not a valid u32");
        'laa: for val in r1..=r2 {
            let id_s = val.to_string();
            let id: &str = id_s.as_str();
            let id_l = id.len();

            if id_l == 1 {
                continue 'laa;
            }

            let f = id.chars().nth(0).unwrap();
            if id.chars().all(|t| t == f) {
                //println!("FOUND for id {:?}", id);
                res += id.parse::<u64>().expect("Not a valid u64");
                continue 'laa;
            }

            for i in 2..=id_l / 2 {
                if id_l % i == 0 {
                    let evaluated = id[..i].parse::<u64>().expect("Not a valid u64");
                    let a_string = id_s.replace(&id[..i].to_string(), "a");
                    if a_string.chars().all(|t| t == 'a') {
                        // maybe the worst possible usage opf
                        //println!("FOUND {:?} for id {:?}", evaluated, id);
                        res += id.parse::<u64>().expect("Not a valid u64");
                        continue 'laa;
                    }
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_part_1_ok() {
        let _lines =
            get_content_as_string(format!(r"ressources/test/day_{}", CFG_DAY).to_owned()).unwrap();
        //solve
        let checksum = count_invalid_ID(_lines);
        assert_eq!(
            1227775554,
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
            get_content_as_string(format!(r"ressources/test/day_{}", CFG_DAY).to_owned()).unwrap();
        //solve
        let checksum = count_invalid_ID_2(_lines);
        assert_eq!(
            4174379265,
            checksum,
            "{}",
            format!(
                r"\nTest data for partie 2.\nsee https://adventofcode.com/2025/day/{}",
                CFG_DAY
            )
        );
    }
}
