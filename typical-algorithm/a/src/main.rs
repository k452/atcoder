fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let result = binary_search(n, k, &a);
    // println!("回答: {:?}", result);
    println!("{}", result);
}

fn binary_search(n: usize, split_point: usize, a: &Vec<usize>) -> isize {
    let mut left: usize = 0;
    let mut right = n;

    let mut result: Option<isize> = None;

    loop {
        if &left >= &right {
            break;
        }

        let mid = (&left + &right) / 2;

        // println!("left={} right={} mid={}", &left, &right, &mid);

        if a[mid] == split_point {
            result = Some(mid.try_into().unwrap());
        } else if split_point < a[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    match result {
        Some(value) => value,
        _ => {
            if right == n {
                -1
            } else {
                right.try_into().unwrap()
            }
        }
    }
}
