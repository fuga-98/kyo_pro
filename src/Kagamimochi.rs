use proconio::input;

fn main(){
    input!{
        n:usize,    //i32型とusize型が存在する。競技プログラミングでは整数はusizeでとるのが良い。
    }
    let mut vector= Vec::new(); //可変じゃないとpush()ができない
    'check_number:
     for i in 0..n{
        input!{
            a:i32,
        }
        if i==0{
            vector.push(a);
            continue; //breakの違いを確認
        }
        //vectorの範囲外に出るとエラーになるため0..iだとだめ
        for j in 0..vector.len(){
            if a==vector[j]{continue 'check_number;}
        }
        vector.push(a)
    }
    println!("{}",vector.len());
}
