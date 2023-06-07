use proconio::input;

fn main(){
    input!{
        s: String,
    }
    //chars()で文字列をイテレータに変換、revで逆順に、collectで収集
    let reverse_str=s.chars().rev().collect::<String>();
    let mut n=0;

    //>=ではなく==にして中でbreakする
    while n<reverse_str.len(){
        // let part_five = reverse_str.chars().skip(n).take(5).collect::<String>();
        // let part_six = reverse_str.chars().skip(n).take(6).collect::<String>();
        // let part_seven = reverse_str.chars().skip(n).take(7).collect::<String>();
    //n~n+5,6.7文字で捜査
        if reverse_str.chars().skip(n).take(5).collect::<String>()=="maerd"|| reverse_str.chars().skip(n).take(5).collect::<String>()=="esare"{
            n+=5;
            //print!("{}",n);
        } else if reverse_str.chars().skip(n).take(6).collect::<String>()=="resare"{
            n+=6;
            //print!("{}",n);
        } else if reverse_str.chars().skip(n).take(7).collect::<String>()=="remaerd"{
            n+=7;
            //print!("{}",n);
        } else{
            break;
        }
    }   
    if n==reverse_str.len(){print!("YES");}
    else{print!("NO");}
}

