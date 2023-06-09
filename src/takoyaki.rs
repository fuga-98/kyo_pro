use proconio::input;

fn main(){
    input!{
        n:i32,
        x:i32,
        t:i32,
    }
    //このようなifをするときは｛｝に；を入れないこと
    //println!("{}", t * ((n - 1) / x + 1));別解なんでこれでいけるんや
    
    let times =if n%x==0{n/x} else {n/x+1};
    print!("{}",t*times);

}