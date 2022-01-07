use std::io;

fn main() {
    println!("Hello, world!");
    // write_user_name();
    // sum_of_numbers();
    sum_or_product();
}


fn write_user_name() {
    println!("What is your name?");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    if name.trim() == "Alice" || name.trim() == "Bob" {
        println!("Your name is {}", name);
    } else {
        println!("Your not Alice or Bob");
    }
}

fn sum_of_numbers() {
    println!("Type in a number");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if num == 0 {
        println!("Need a valid input");
    } else {
        let mut count = 0;

        for i in 0..num+1 {
            if i % 3 == 0 || i % 5 == 0 {
                count = count + i;
            } else {
                continue;
            }
            count = count + i;
        }
        println!("sum is {}", count);
    }
}

fn sum_or_product() {
    println!("Sum or Multiply?");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    println!("Type a number to {} to", choice);

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let chosen = choice.trim().to_lowercase();

    if num == 0 {
        println!("Need a valid input");
    } else if chosen == "sum" {
        let mut count = 0;

        for i in 0..num+1 {
            count = count + i;
        }

        println!("sum is {}", count);

    } else if chosen == "multiply" {
        let mut count = 1;

        for i in 1..num+1 {
            count = count * i;
        }

        println!("multiplied is {}", count);
    } else {
        println!("Need to choose Sum or Multiply");
    }
}
