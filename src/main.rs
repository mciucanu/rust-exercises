use std::io;

// fn exercise1() {
//     println!("Hello, world!");
// }

// fn exercise2() {
//     println!("Please input your name.");
//     let mut name = String::new();
//     io::stdin()
//         .read_line(&mut name)
//         .expect("Failed to read name");
//     println!("Your name is {}", name);
// }

// fn exercise3() {
//     println!("Please input your name.");
//     let mut name = String::new();
//     io::stdin()
//         .read_line(&mut name)
//         .expect("Failed to read name");

//     if name.trim() == "Bob" {
//         println!("Your name is {}", name);
//     } else if name.trim() == "Alice" {
//         println!("Your name is {}", name);
//     } else {
//         println!("LEAVE!");
//     }
// }

// fn exercise4() {
//     println!("Please input a number.");
//     let mut num = String::new();
//     io::stdin()
//         .read_line(&mut num)
//         .expect("Failed to read line");

//     let num: u32 = match num.trim().parse() {
//         Ok(numb) => numb,
//         Err(_) => 0,
//     };

//     let mut sum: u32 = 0;
//     for i in 0..num {
//         sum = sum + i + 1;
//     }
//     println!("The sum of 1 to {} = {}", num, sum);
// }

// fn exercise5() {
//     println!("Please input a number.");
//     let mut num = String::new();
//     io::stdin()
//     .read_line(&mut num)
//     .expect("Failed to read line");
//     let num: u32 = match num.trim().parse() {
//         Ok(numb) => numb,
//         Err(_) => 0,
//     };
//     let mut sum: u32 = 0;
//     for i in 0..num {
//         if ((i + 1) % 3) == 0 {
//             sum = sum + i + 1;
//         } else if ((i + 1) % 5) == 0 {
//             sum = sum + i + 1;
//         } else {
//             continue;
//         }
//     }
//     println!("The sum of 1 to {} = {}", num, sum);
// }

fn exercise6() {
    println!("Please input a number.");
    let mut num = String::new();
    let mut compute_type = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line num");
    println!("Please input a computing type.");
    io::stdin()
        .read_line(&mut compute_type)
        .expect("Failed to read line type");

    let num: u32 = match num.trim().parse() {
        Ok(numb) => numb,
        Err(_) => 0,
    };

    let mut sum: u32 = 0;
    let mut product: u32 = 1;
    for i in 0..num {
        if compute_type.trim() == "sum" {
            sum = sum + i + 1;
        } else if compute_type.trim() == "product" {
            product = product * (i + 1);
        } else {
            break;
        }
    }

    if compute_type.trim() == "sum" {
        println!("The sum of 1 to {} = {}", num, sum);
    } else if compute_type.trim() == "product" {
        println!("{}! = {}", num, product);
    } else {
        println!("Please input sum or product");
    }
}

fn main() {
    // exercise1();
    // exercise2();
    // exercise3();
    // exercise4();
    // exercise5();
    exercise6();
}
