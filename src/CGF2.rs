use proconio::input;

fn main(){
    input!{
        n:usize,
        mut a:[i32; n],
    }
    //降順にソート
    a.sort_by(|a,b| b.cmp(a));
    let mut sum=0;
    for i in 0..n {
        if i % 2 == 0 {
            sum += a[i];
        } else {
            sum -= a[i];
        }
    }
    println!("{}",sum);
}
