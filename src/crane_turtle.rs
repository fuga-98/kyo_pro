use proconio::input;

fn main(){
    input!{
        x:i32,
        y:i32,
    }
    //考慮漏れ　マイナスがある
    if y-2*x<0 || 4*x-y<0 || (y-2*x) % 2!=0 || (4*x - y)%2!=0
    {
        print!("No");
    } else{print!("Yes");}
}