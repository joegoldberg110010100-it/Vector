mod function;
use function::vector;
use std::io;
fn main() {
    let n:i32 = loop {
        println!("Enter the number of elements:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error of reads");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            },
        };
    };

    let mut my_vec: Vec<i32> = Vec::new();

    for i in 0..n {
        println!("Enter element {}:", i + 1);

        let num: i32 = loop {
            let mut element = String::new();
            io::stdin()
                .read_line(&mut element)
                .expect("Error of reads");
                match element.trim().parse() {
                    Ok(num) => break num,
                    Err(_) => {
                        println!("Write the number!");
                        continue;
                    }
                }
        };
        
        my_vec.push(num);
    }

    println!("Vector: {:?}", my_vec);
    println!("Sum: {}", vector::vector_sum(&my_vec));
    println!("Max: {}", vector::find_max(&my_vec));
    println!("Min: {}", vector::find_min(&my_vec));
    println!("Average: {:.2}", vector::average(&my_vec));
}