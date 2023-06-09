use proconio::input;

fn main(){
    input!{
        n:i32
    }
    if n%2==0{
        //{}はいらない。
        print!("White");
    }else {
        print!("Black");
    }
}