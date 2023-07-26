use proconio::input;

fn main(){
    input!{
        n:usize,
        v:[(i64,i64);n],
    }
    let mut ans=0;
    for &i in &v{
        ans+=(i.0+i.1) * (i.1-i.0 + 1) / 2;
    }
    print!("{}",ans)
}