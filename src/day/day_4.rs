#![allow(warnings)]
use itertools::iproduct;

use crate::day::common::*;

const CFG_DAY: &str = "4";

pub fn solve() -> Result<(), AdventError> {
    //data
    let mut map: Vec<Vec<char>> = get_map_char(format!(r"ressources/day_{}", CFG_DAY).to_owned())?;

    let res = fewer_neigh_than(map.clone(), 4);

    print_sol_1(res);

    let mut fin = 0;
    let mut res2 = 1;
    while res2 > 0 {
        (res2, map) = fewer_neigh_than_recu(map, 4);
        fin += res2;
    }
    print_sol_2(fin);

    Ok(())
}

pub fn fewer_neigh_than(map: Vec<Vec<char>>, neigh: u32) -> u32 {
    let mut res = 0;
    let mut map = map;
    let neigh: [i32; 3] = [-1, 0, 1];

    for i in &mut map {
        i.insert(0, '.');
        i.push('.');
    }

    map.insert(0, vec!['.'; map[1].len()]);
    map.push(vec!['.'; map[1].len()]);

    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            if map[i][j] == '@' {
                let mut neigh_number = 0;
                for (a, b) in iproduct!(neigh, neigh) {
                    if a == 0 && b == 0 {
                        continue;
                    }
                    if map[(i as i32 + a) as usize][(j as i32 + b) as usize] == '@' {
                        neigh_number += 1;
                    }
                }
                if neigh_number < 4 {
                    res += 1;
                }
            }
        }
    }

    res
}

pub fn fewer_neigh_than_recu(map: Vec<Vec<char>>, neigh: u32) -> (u32, Vec<Vec<char>>) {
    let mut res = 0;
    let mut map = map;
    let neigh: [i32; 3] = [-1, 0, 1];
    let mut to_empty = vec![];

    for i in &mut map {
        i.insert(0, '.');
        i.push('.');
    }

    map.insert(0, vec!['.'; map[1].len()]);
    map.push(vec!['.'; map[1].len()]);

    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            if map[i][j] == '@' {
                let mut neigh_number = 0;
                for (a, b) in iproduct!(neigh, neigh) {
                    if a == 0 && b == 0 {
                        continue;
                    }
                    if map[(i as i32 + a) as usize][(j as i32 + b) as usize] == '@'
                    //|| map[(i as i32 + a) as usize][(j as i32 + b) as usize] == 'a'
                    {
                        neigh_number += 1;
                    }
                }
                if neigh_number < 4 {
                    res += 1;
                    to_empty.push((i, j));
                    //map[i][j] = 'a';
                }
            }
        }
    }

    // for i in 1..map.len() - 1 {
    //     for j in 1..map[0].len() - 1 {
    //         if map[i][j] == 'a' {
    //             map[i][j] = '.';
    //         }
    //     }
    // }

    for (i, j) in to_empty {
        map[i][j] = '.';
    }

    (res, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_ok() {
        let _lines = get_map_char(format!(r"ressources/test/day_{}", CFG_DAY).to_owned()).unwrap();
        //solve
        let checksum = fewer_neigh_than(_lines, 4);
        assert_eq!(
            13,
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
        let mut _lines =
            get_map_char(format!(r"ressources/test/day_{}", CFG_DAY).to_owned()).unwrap();
        //solve
        let mut res = 1;
        let mut checksum = 0;
        while res > 0 {
            (res, _lines) = fewer_neigh_than_recu(_lines, 4);
            println!("{res}");
            checksum += res;
        }
        assert_eq!(
            43,
            checksum,
            "{}",
            format!(
                r"\nTest data for partie 1.\nsee https://adventofcode.com/2025/day/{}",
                CFG_DAY
            )
        );
    }
}
