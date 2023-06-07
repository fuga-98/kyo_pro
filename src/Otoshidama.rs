use proconio::input;

fn main(){
    input!{
        n:usize,
        y:usize,
    }
    let yukichi = y/10000;
    let higuchi = (y%10000)/5000;
    let noguchi = (y%5000)/1000;
    if yukichi+higuchi+noguchi <= n{
        print!("{}{}{}",yukichi,higuchi,noguchi);
    } else{print!("-1 -1 -1")}
}