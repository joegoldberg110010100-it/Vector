pub fn vector_sum(vec: &Vec<i32>) -> i32{
    vec.iter().copied().sum()  //return sum
}

pub fn find_max(vec: &Vec<i32>) -> i32 {
    let max_num: i32 = match vec.iter().max() {
        Some(&num) => num,
        None => {
            println!("Ur vector haven't numbers!");
            0
        },
    };
    max_num
}

pub fn find_min(vec: &Vec<i32>) -> i32{
    let min_number: i32 = match vec.iter().min() {
        Some(&num) => num,
        None => {
            println!("Ur vector haven't numbers!");
            0
        },
    };
    min_number 
}

pub fn difference_sums(&sum1: &i32, &sum2: &i32) -> i32 {
    sum1 - sum2
} // don't use here :)

pub fn average(vec: &Vec<i32>) -> f64 {
    if vec.is_empty() { return 0.0;};
    vec.iter().sum::<i32>() as f64 / vec.len() as f64
}
