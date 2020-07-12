#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        n: usize,
    }
    let mut ans = 0;
    for i in 0..(n / r) + 1 {
        for j in 0..(n / g) + 1 {
            if n >= i * r + j * g {
                let temp = n - i * r - j * g;
                if temp % b == 0 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
