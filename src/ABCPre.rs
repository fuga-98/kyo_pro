use proconio::input;

fn main(){
    input!{
        a: [i32; 4],
    }
    let mut ans=100;
    for &i in &a{
        if i<ans{
            ans=i
        }
    }
    println!("{}",ans);
}