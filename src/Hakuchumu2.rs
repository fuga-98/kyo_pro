use proconio::input;
fn main() {
    input!{
        x:String,
    }
    let patterns = ["eraser", "erase", "dreamer", "dream"];
    let s = patterns.iter().fold(read::<String>(), |s, x| s.replace(x, ""));
    println!("{}", if s.is_empty() { "YES" } else { "NO "});
}