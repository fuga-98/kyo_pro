use proconio::input;

fn main(){
    input!{
        N:i32,
        A:i32,
        B:i32,
    }
    let mut sum=0;
    for t in 0..=N{
        let sum_digit = calc_digit(t);
        if sum_digit<A || B<sum_digit{
            continue;
        }
        sum+=t;
    }
    println!("{}",sum);
}
fn calc_digit(mut t:i32)->i32{
    //tを一桁ずつ割る
    let mut sum_digit = 0;
    while t >=1 {
        sum_digit += t % 10;
        t /= 10;
    }
    sum_digit
}