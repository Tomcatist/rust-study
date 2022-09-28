use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    // 生成随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // new 一个String变量
        let mut guess = String::new();

        // 接收用户输入数据
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // String转int32 并处理异常情况
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input number.");
                println!("try again!");
                continue;
            }
        };

        //println!("您的猜测是: {guess}");
        println!("You guessed: {}",guess);

        // 输入的数和随机数比较
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }


}
