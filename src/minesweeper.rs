use proconio::input;

fn main() {
    //入力は問題なし
    input!{
        h:usize,
        w:usize,     
    }
    let mut vec = Vec::new();  // 空のベクタを作成します
    for i in 0..h{
        let mut nums = Vec::new();  // 空のベクタを作成します
        input!{
           mut row:String,
        }
        //inputから数値型にしちゃおう.また#を9にする。mineの最大値は８のため
        for c in row.chars(){
            if c=='.'{
                nums.push(0)
            } else{
                nums.push(9)
            }
        }
        vec.push(nums)
    }
    assert_eq!(vec[1],9);  

    //.を0に置き換えて、#が横にあったら+する。同様に縦にあったら+する
    //路線変更　周りに９があったらプラスする。
    for i in 0..h{
        for j in 0..w{
            if vec[i][j]==9{continue;}
            //(i,j)の周りを回る。(i,j)が含まれているが、ifの条件を満たさないためそのまま.参照だけ気を付ける。
            //usize関係がめんどくさい
            for k in -1..1{ 
                if i-k<0 || j-k<0{continue;}
                if vec[i-k][j-k]==9{vec[i][j]+=1;}
            }
        }
    }
    print!("{:?}",vec);
}
//以下chatGPT作成
//    ベクタへのインデックス操作（vec[i][j]）が配列の範囲外に行く可能性があります。 if i-k<0 || j-k<0 {continue;} で範囲外アクセスを防いでいますが、上限に対するチェックが欠けています。すなわち、 i+k または j+k が配列のサイズ（hまたはw）を超える場合のチェックが必要です。

// 周囲のセルをチェックするループ for k in -1..1 が誤っています。このループは -1 と 0 の2回しか実行されません。それは .. 演算子は右側の値を含まないからです。したがって、一般的には for k in -1..=1 を使って -1 から 1 までを含む範囲を作る必要があります。

// また、 (i, j) の周りを見るとき、一つの変数 k だけでなく、2つの変数（dx と dy を例に）を使って周囲8方向をチェックすることが一般的です。

// assert_eq!(vec[1],9); はエラーを引き起こします。 vec[1] はベクタ（Vec<usize>）を返すため、これを 9 と比較することはできません。

// 以上を踏まえて、コードは次のように改良できます：
// use proconio::input;

// fn main() {
//     input! {
//         h: usize,
//         w: usize,     
//     }
//     let mut vec = Vec::new();
//     for _ in 0..h {
//         let mut nums = Vec::new();
//         input! {
//            row: String,
//         }
//         for c in row.chars() {
//             if c == '.' {
//                 nums.push(0)
//             } else {
//                 nums.push(9)
//             }
//         }
//         vec.push(nums);
//     }

//     let dx = vec![-1, 0, 1, -1, 1, -1, 0, 1];
//     let dy = vec![-1, -1, -1, 0, 0, 1, 1, 1];

//     for i in 0..h {
//         for j in 0..w {
//             if vec[i][j] == 9 { continue; }
//             for dir in 0..8 {
//                 let nx = i as i32 + dx[dir];
//                 let ny = j as i32 + dy[dir];
//                 if nx < 0 || nx >= h as i32 || ny < 0 || ny >= w as i32 { continue; }
//                 if vec[nx as usize][ny as usize] == 9 { vec[i][j] += 1; }
//             }
//         }
//     }
//     for i in 0..h {
//         for j in 0..w {
//             if vec[i][j] == 9 {
//                 print!("#");
//             } else {
//                 print!("{}", vec[i][j]);
//             }
//         }
//         println!("");
//     }
// }

