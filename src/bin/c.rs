use proconio::input;
// use proconio::marker::Chars;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: i64
    }

    let n_sqrt = ((n as f32)+ 1.0).sqrt() as usize;

    let mut tbl  = BTreeSet::new();

    for a in 2..=n_sqrt + 1{
        for b in 2..n{
            let x = a.pow(b as u32) as i64;
            if x > n { break }
            tbl.insert(x);
        }
    }

    println!("{}", n - tbl.len() as i64)

}