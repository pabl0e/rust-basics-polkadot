fn find_maximum(numbers: &[i32]) -> i32 {
    //numbers.iter().max().unwrap()
    let mut max = numbers[0];
    for &num in numbers {
        if num > max {
            max = num;
        }
    }
    max        
}

fn find_minimum(numbers: &[i32]) -> i32 {
    //numbers.iter().min().unwrap()
    let mut min = numbers[0];
    for &num in numbers {
        if num < min {
            min = num;
        }
    }
    min
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let (max, min) = find_maximum_and_minimum(&numbers);
    println!("Maximum: {}", max);
    println!("Minimum: {}", min);
}

fn find_maximum_and_minimum(numbers: &[i32]) -> (i32, i32) {
    let max = find_maximum(numbers);
    let min = find_minimum(numbers);
    (max, min)
}
