use std::io;

fn main() {
    println!("Hello, this is my factorial example!");

    loop {
        println!("\nEnter a number (enter q to exit)");

        let mut num = String::new();
        io::stdin().read_line(&mut num)
            .expect("Failed to read line");

        let num = num.trim();

        if num == "q" {
            break;
        }

        let num: u32 = match num.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Number");
                continue;
            },
        };

        if num == 0 {
            println!("Factorial of {} is: 1", num);
            continue;
        }

        let mut fact: u64 = 1;

        for x in 1..(num + 1) {
            let x64 = x as u64;
            fact = x64 * fact;
        }

        println!("Factorial of {} is: {}", num, fact);
    }
}
