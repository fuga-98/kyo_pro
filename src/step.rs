use proconio ::input;

fn main(){
    //n: usize と v: [T; n] が，あわせて v: [T] と書ける
    input!{
        a:[i64]
    }
// 5
// 2 1 5 4 3
// を入力してprint!("{:?}",a)すると
// [2, 1, 5, 4, 3]
// 
    let mut checker=0;
    let mut sum=0;
    for &i in &a{
        if i<checker{
            sum+=checker-i;
        }
        else{
            checker=i
        }
    }
    print!("{}",sum);
}