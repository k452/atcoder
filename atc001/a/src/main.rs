use proconio::input;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
enum CoordinateType {
    START,
    GOAL,
}

type CoordinateMap = HashMap<CoordinateType, (isize, isize)>;

fn main() {
    input! {
        h: usize,
        w: usize,
        c_origin: [String; h]
    };

    // 入力を探索に必要な形に前処理
    // coordinate_mapはスタートとゴールの座標を保持
    let (mut c, coordinate_map) = pretreatment_for_map(c_origin);

    // スタート地点の座標を取得
    let (s_raw_idx, s_column_idx) = coordinate_map.get(&CoordinateType::START).unwrap();
    // ゴール地点の座標を取得
    let (g_raw_idx, g_column_idx) = coordinate_map.get(&CoordinateType::GOAL).unwrap();

    // 探索
    dfs(s_column_idx, s_raw_idx, &h, &w, &mut c, &coordinate_map);

    // println!("{:?}", &c);

    // 探索済みのマップのゴール地点の座標が-1かどうかで到達可能性を判定
    match c[*g_raw_idx as usize][*g_column_idx as usize] == 'g' {
        true => println!("No"),
        false => println!("Yes"),
    }
}

/// マップを１文字ずつに分割し、スタート地点とゴール地点の座標についてはhashMapに記録
/// マップはc[行][列]のように指定できる
fn pretreatment_for_map(c_origin: Vec<String>) -> (Vec<Vec<char>>, CoordinateMap) {
    let mut c = vec![];
    let mut coordinate_map: CoordinateMap = HashMap::new();

    for (raw_idx, v) in c_origin.iter().enumerate() {
        let splited_raw = v.chars().collect::<Vec<char>>();

        // sが含まれる時にスタートの座標をhashMapに記録
        if let Some(column_idx) = splited_raw.iter().position(|&s| s == 's') {
            coordinate_map.insert(
                CoordinateType::START,
                (raw_idx.try_into().unwrap(), column_idx.try_into().unwrap()),
            );
        }
        // gが含まれる時にゴールの座標をhashMapに記録
        if let Some(column_idx) = splited_raw.iter().position(|&s| s == 'g') {
            coordinate_map.insert(
                CoordinateType::GOAL,
                (raw_idx.try_into().unwrap(), column_idx.try_into().unwrap()),
            );
        }

        c.push(splited_raw);
    }

    (c, coordinate_map)
}

/// 与えられた座標(x,y)から深さ優先探索を再帰的に行う
/// 探索順は↑→↓←
/// 訪問したところは壁にする(cを上書きする)
fn dfs<'a>(
    x: &'a isize,
    y: &'a isize,
    h: &'a usize,
    w: &'a usize,
    c: &'a mut Vec<Vec<char>>,
    coordinate_map: &'a CoordinateMap,
) {
    // マップの範囲外の時はreturn
    if *x < 0 || *y < 0 || *x >= *w as isize || *y >= *h as isize {
        return;
    };

    // 現在地が壁の場合はreturn
    if c[*y as usize][*x as usize] == '#' {
        return;
    };

    // 現在地を訪問済み(壁)にする
    c[*y as usize][*x as usize] = '#';

    // 同期的に再帰を実行しているため↑に進み続けてreturnされると直前の分岐に戻って→に進むという形で深さ優先になる
    dfs(&x, &(*y - 1), &h, &w, c, &coordinate_map); //↑
    dfs(&(*x + 1), &y, &h, &w, c, &coordinate_map); //→
    dfs(&x, &(*y + 1), &h, &w, c, &coordinate_map); //↓
    dfs(&(*x - 1), &y, &h, &w, c, &coordinate_map); //←
}
