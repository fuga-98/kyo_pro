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
            for k in -1..1{ 
                if i-k<0 || j-k<0{continue;}
                if vec[i-k][j-k]==9{vec[i][j]+=1;}
            }
        }
    }
    print!("{:?}",vec);
}