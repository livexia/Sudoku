use std::error::Error;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::Result;

#[derive(Debug)]
pub struct Sudoku {
    grid: [Option<u8>; 81],
}

impl Default for Sudoku {
    fn default() -> Self {
        Self { grid: [None; 81] }
    }
}

impl Sudoku {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fill(&mut self, i: usize, j: usize, v: u8) {
        if (0..9).contains(&i) && (0..9).contains(&j) && (1..=9).contains(&v) {
            self.grid[i * 9 + j] = Some(v);
        } else {
            println!("wrong fill")
        }
    }

    fn get(&self, i: usize, j: usize) -> Option<u8> {
        self.grid[i * 9 + j]
    }

    fn get_3by3(&self, i: usize, j: usize) -> u16 {
        let (mid_i, mid_j) = (i / 3 * 3 + 1, j / 3 * 3 + 1);
        (-1..=1)
            .flat_map(|offset_i| {
                (-1..=1).filter_map(move |offset_j| {
                    self.get(
                        (mid_i as i32 + offset_i) as usize,
                        (mid_j as i32 + offset_j) as usize,
                    )
                })
            })
            .fold(0, |r, v| r | (1 << v))
    }

    fn get_vertical(&self, _i: usize, j: usize) -> u16 {
        (0..9)
            .filter_map(|i| self.get(i, j))
            .fold(0, |r, v| r | (1 << v))
    }

    fn get_horizontal(&self, i: usize, _j: usize) -> u16 {
        (0..9)
            .filter_map(|j| self.get(i, j))
            .fold(0, |r, v| r | (1 << v))
    }
    pub fn options(&self, i: usize) -> Option<u16> {
        let (i, j) = (i / 9, i % 9);
        match self.get(i, j) {
            Some(_) => None,
            None => {
                Some(!(self.get_vertical(i, j) | self.get_horizontal(i, j) | self.get_3by3(i, j)))
            }
        }
    }

    pub fn solve(&mut self) -> bool {
        self.dfs(0)
    }

    pub fn dfs(&mut self, i: usize) -> bool {
        if let Some(ops) = self.options(i) {
            if ops == 0 {
                return false;
            }
            for mask in 1..10 {
                if (ops >> mask) & 1 != 0 {
                    self.grid[i] = Some(mask);
                    if i == 80 || self.dfs(i + 1) {
                        return true;
                    }
                    self.grid[i] = None;
                }
            }
            false
        } else {
            i == 80 || self.dfs(i + 1)
        }
    }
}

impl FromStr for Sudoku {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut grid = [None; 81];
        for p in s.split(',') {
            if p.trim().len() == 3 {
                let mut p = p.trim().bytes();
                let i = p.next().unwrap() - b'0';
                let j = p.next().unwrap() - b'0';
                let n = p.next().unwrap() - b'0';
                grid[i as usize * 9 + j as usize] = Some(n);
            }
        }
        Ok(Self { grid })
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("    0   1   2   3   4   5   6   7   8\n")?;
        f.write_str("  ┏━━━┯━━━┯━━━┳━━━┯━━━┯━━━┳━━━┯━━━┯━━━┓\n")?;
        for i in 0..9 {
            if i == 3 || i == 6 {
                f.write_str("  ┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫\n")?;
            }
            if i % 3 != 0 {
                f.write_str("  ┠───┼───┼───╂───┼───┼───╂───┼───┼───┨\n")?;
            }
            f.write_fmt(format_args!("{} ", i))?;
            for j in 0..9 {
                let v = match self.get(i, j) {
                    Some(v) => &v.to_string(),

                    None => " ",
                };
                match j % 3 == 0 {
                    true => f.write_fmt(format_args!("┃ {} ", v))?,
                    false => f.write_fmt(format_args!("│ {} ", v))?,
                }
            }
            f.write_str("┃\n")?;
        }
        f.write_str("  ┗━━━┷━━━┷━━━┻━━━┷━━━┷━━━┻━━━┷━━━┷━━━┛\n")
    }
}

pub fn nums_to_bits(v: &[u8]) -> u16 {
    v.iter().fold(0, |r, i| r | (1 << i))
}

pub fn bits_to_nums(r: u16) -> Vec<u8> {
    let mut v = vec![];
    for i in 1..10 {
        if (r >> i) & 1 == 1 {
            v.push(i);
        }
    }

    v
}

#[cfg(test)]
mod test {
    use crate::sudoku::{bits_to_nums, nums_to_bits, Sudoku};

    #[test]
    fn test_new() {
        let s = Sudoku::new();
        assert_eq!(s.grid, [None; 81])
    }

    #[test]
    fn test_fill() {
        let mut s = Sudoku::new();
        s.fill(0, 0, 8);
        assert_eq!(s.get(0, 0), Some(8))
    }

    #[test]
    fn test_gets() {
        let mut s = Sudoku::new();
        s.fill(0, 0, 8);
        assert_eq!(s.get_vertical(0, 0), nums_to_bits(&[8]));
        assert_eq!(s.get_horizontal(0, 0), nums_to_bits(&[8]));
        assert_eq!(s.get_3by3(0, 0), nums_to_bits(&[8]));
        assert_eq!(s.options(0), None);
        assert_eq!(s.options(10), Some(nums_to_bits(&[1, 2, 3, 4, 5, 6, 7, 9])));
    }

    #[test]
    fn test_nums_to_bits() {
        let v = vec![1, 2, 4, 5];
        assert_eq!(nums_to_bits(&v), 0b110110);
    }

    #[test]
    fn test_bits_to_nums() {
        let v = vec![1, 2, 4, 5];
        let i = 0b110110;
        assert_eq!(bits_to_nums(i), v);
    }

    #[test]
    fn test_parse() {
        let s = "001,022,033,044,149,888";
        let sudoku: Sudoku = s.parse().unwrap();
        println!("{}", sudoku);
        assert_eq!(sudoku.get(0, 0), Some(1));
        assert_eq!(sudoku.get(0, 2), Some(2));
        assert_eq!(sudoku.get(0, 3), Some(3));
        assert_eq!(sudoku.get(0, 4), Some(4));
        assert_eq!(sudoku.get(1, 4), Some(9));
        assert_eq!(sudoku.get(8, 8), Some(8));
    }

    #[test]
    fn test_parse2() {
        let mut s = Sudoku::new();
        s.fill(0, 1, 4);
        s.fill(0, 2, 3);
        s.fill(0, 4, 8);
        s.fill(0, 6, 2);
        s.fill(0, 7, 5);

        s.fill(1, 0, 6);

        s.fill(2, 5, 1);
        s.fill(2, 7, 9);
        s.fill(2, 8, 4);

        s.fill(3, 0, 9);
        s.fill(3, 5, 4);
        s.fill(3, 7, 7);

        s.fill(4, 3, 6);
        s.fill(4, 5, 8);

        s.fill(5, 1, 1);
        s.fill(5, 3, 2);
        s.fill(5, 8, 3);

        s.fill(6, 0, 8);
        s.fill(6, 1, 2);
        s.fill(6, 3, 5);

        s.fill(7, 8, 5);

        s.fill(8, 1, 3);
        s.fill(8, 2, 4);
        s.fill(8, 4, 9);
        s.fill(8, 6, 7);
        s.fill(8, 7, 1);

        let s1: Sudoku = "014,023,048,062,075,106,251,279,284,309,354,377,436,458,\
            511,532,583,608,612,635,785,813,824,849,867,871"
            .parse()
            .unwrap();
        assert_eq!(s.grid, s1.grid);
    }
}
