use proconio::input;

fn main() {
    input! {
        x: i32,
    }
    let s1=x/100;
    let s2=x%100/10;
    let s3=x%10;
    let y = s1+s2+s3;
    print!("{}",y);
}