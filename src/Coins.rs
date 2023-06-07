use proconio::input;

fn main(){
    input!{
        a:i32,
        b:i32,
        c:i32,
        x:i32,
    }
    let mut count =0;
    for x in 0..a+1{
        for y in 0..b+1{
            for z in 0..c+1{
                let num=x*500+y*100+z*50;
                if num==x{
                    count+=1;
                }
            }
        }
    }
    print!("{}",count);   
}
