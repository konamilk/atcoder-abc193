use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input! {
        n: i32,
        mut apx: [(i32, i32, i32); n]
    }

    let mut ans = std::i32::MAX;
    let mut flg = false;

    for elem in apx{
        if elem.2 - elem.0 > 0 {
            ans = std::cmp::min(ans, elem.1);
            flg = true
        }
    }

    if !flg {
        ans = -1
    }

    println!("{}", ans)
}
