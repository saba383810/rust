extern crate rand;


use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {

    println!("数あてゲームをはじめるよ！!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("乱数の値は:{}",secret_number);

    loop{

        println!("数字をにゅうりょくしてね！");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("読み込みに失敗しました");
    
        let guess: u32 = guess.trim().parse().expect("数字を入力して！");
        
        println!("あなたの入力は{}",guess);

        match guess.cmp(&secret_number){

            Ordering::Less => println!("小さいよ！"),
            Ordering::Greater => println!("大きいよ！"),
            Ordering::Equal =>{
                println!("あたり！！！");
                break;
            }
        }
    }
}
