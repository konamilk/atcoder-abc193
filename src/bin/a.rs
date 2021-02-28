use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        a: f64,
        b: f64
    }

    println!("{}", (a - b) / a * 100.0)
}
