fn calculate_average(numbers: & Vec<i32>) -> f32 {
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len();

    return (sum as f32) / (count as f32);
}

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let average: f32 = calculate_average(&numbers);

    println!("Average = {}", average);
}