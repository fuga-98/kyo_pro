use proconio::input;

fn main(){
    input!{
        r:f64,
    }
    //円周率はこんな感じで解くよ。f64とかを使うときに整数はエラーになるよ　2.0みたいに書こう。
    print!{"{}",r *2.0*std::f64::consts::PI};
}