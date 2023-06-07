use proconio::input;

fn main(){
    input!{
        n: usize,
        v: [u64; n],
    }
    let mut min =2^160;
    let first=0;
    for num in &v{
        let a=*num;
        let  times= count(a,first);
        if times<min{
            min = times;
        } 
    }
    println!("{}",min)
}
fn count(num:u64,times:u64)->u64{
    if num % 2==0{
        count(num/2,times+1)
        
    } else{
        return times
    }
}