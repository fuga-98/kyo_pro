use proconio::input;

fn main(){
    input!{
        sx:f32,
        sy:f32,
        gx:f32,
        gy:f32,
    }
    //gyを反転
    let mirror=-gy;
    //tilt:傾き ただしx=tilt*y　+　切片
    let tilt=(sx - gx) / (sy - mirror);
    //y切片 https://atcoder.jp/contests/abc183/editorial/287別解
    let xslice=sx - tilt*sy;
    print!("{}",xslice);
}