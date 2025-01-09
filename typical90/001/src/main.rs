use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n]
    }

    println!("{} {} {} {}", &n, &l, &k, &a[2])
}

fn calc() {}
