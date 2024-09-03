pub mod sudoku;

use std::error::Error;

use crate::sudoku::Sudoku;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() {
    let mut s: Sudoku = "014,023,048,062,075,106,251,279,284,309,354,377,436,458,\
            511,532,583,608,612,635,785,813,824,849,867,871"
        .parse()
        .unwrap();

    println!("sudoku:\n{}", s);

    s.solve();
    println!("answer:\n{}", s);
}
