use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[usize; w]; h]
    }

    let sumed_a = sum_row_and_column(&h, &w, &mut a);
    let result = sum_map(&h, &w, sumed_a);

    for row in result {
        println!(
            "{}",
            row.iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

/// Aより各行と各列の合計値を算出してAのW列とH行に追加したVecを返す
fn sum_row_and_column<'a>(
    h: &'a usize,
    w: &'a usize,
    a: &'a mut Vec<Vec<usize>>,
) -> &'a Vec<Vec<usize>> {
    // 合計値用の行を追加
    a.push(vec![0; *w]);

    // 各行の合計値を追加
    for row in a.iter_mut() {
        row.push(row.iter().sum());
    }

    // 各列の合計値を追加
    for col in 0..*w {
        a[*h][col] = a.iter().map(|row| row[col]).sum();
    }

    a
}

/// 各列と行の合計値をもとに各セルの値を算出
fn sum_map<'a>(h: &'a usize, w: &'a usize, sumed_a: &'a Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut result = vec![vec![0; *w]; *h];

    for i1 in 0..*h {
        for i2 in 0..*w {
            result[i1][i2] = sumed_a[*h][i2] + sumed_a[i1][*w] - sumed_a[i1][i2];
        }
    }

    result
}
