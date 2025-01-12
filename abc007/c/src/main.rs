use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        sy_origin: usize,
        sx_origin: usize,
        gy_origin: usize,
        gx_origin: usize,
        c_origin: [String; h]
    }

    // 座標が1始まりで与えられるため0始まりにする
    let sy = sy_origin - 1;
    let sx = sx_origin - 1;
    let gy = gy_origin - 1;
    let gx = gx_origin - 1;

    // 入力を2次元配列に変換
    let mut c = vec![];
    for raw in c_origin.iter() {
        c.push(raw.chars().collect::<Vec<char>>());
    }

    // スタートからの歩数を保持するスライス
    let mut distance = vec![vec![-1; w]; h];

    // 幅優先探索
    bfs(&w, &h, &sy, &sx, &mut c, &mut distance);

    println!("{}", distance[gy][gx]);
}

fn bfs(
    w: &usize,
    h: &usize,
    sy: &usize,
    sx: &usize,
    c: &mut Vec<Vec<char>>,
    distance: &mut Vec<Vec<isize>>,
) {
    // 探索予定の座標(x,y)を保持するキュー
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    // スタート地点は歩数0
    distance[*sy][*sx] = 0;

    // 訪問済みの地点は壁にする
    c[*sy][*sx] = '#';

    // スタート地点をエンキュー
    q.push_back((*sx, *sy));

    // 4方向に移動する際に加算する座標
    let around_coordinate: [(isize, isize); 4] = [
        (0, -1), // ↑
        (1, 0),  // →
        (0, 1),  // ↓
        (-1, 0), // ←
    ];

    // キューから１つずつ取り出し、その座標の周囲4方向について処理を行う
    while !q.is_empty() {
        let (now_x, now_y) = q.pop_front().unwrap();

        for (to_add_x, to_add_y) in around_coordinate {
            // 移動先の座標を算出
            let x = (now_x as isize + to_add_x) as usize;
            let y = (now_y as isize + to_add_y) as usize;

            // 移動先が範囲外や壁の場合は何もしない
            if x >= *w || y >= *h || c[y][x] == '#' {
                continue;
            };

            // 移動先が未訪問かつ壁でない場合は歩数をインクリメントしてキューに追加する
            if distance[y][x] == -1 && c[y][x] == '.' {
                distance[y][x] = distance[now_y][now_x] + 1;
                q.push_back((x, y));
            }
        }
    }
}
