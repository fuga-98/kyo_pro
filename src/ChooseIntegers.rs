use proconio::input;
fn main(){
    input!{
        A:i32,
        B:i32,
        C:i32
    }
    let mut count=0;
    let mut remainders: Vec<i32> = Vec::new();
    loop{
        let  x = count*A;
        let remainder=x%B;
        if remainder==C{
            println!{"YES"}
            break;
        }
        if remainders.contains(&remainder){
            println!{"No"}
            break;
        }
        count+=1;
        remainders.push(remainder);
    }
}
