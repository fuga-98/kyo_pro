use proconio::input;

fn main(){
    for i in 1..=5{
        input!{
           x:i32, 
        }
        if x==0{
            print!("{}",i)
        }

    }
}