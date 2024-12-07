use std::collections::HashMap;

pub fn parse_input() -> (Vec<i32>, Vec<i32>) {
    // Read and parse the input file into two vectors

    let input = include_str!("../data/day1_1.txt"); // read the contents of the file in compile time

    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in input.lines() {
        // Split the line into two parse
        let numbers: Vec<&str> = line.split_whitespace().collect();

        // Parse the numbers into integers
        if numbers.len() != 2 {
            // Panic and output line
            panic!("Invalid input: {}", line);
        }

        // Push the numbers into the vectors
        if let (Ok(num1), Ok(num2)) = (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
            col1.push(num1);
            col2.push(num2);
        } else {
            // Panic and output line
            panic!("Invalid input: {}", line);
        }
    }

    (col1, col2)
}

pub fn day1_1() {
    // Calculate the sorted distance between the two vectors
    let (mut col1, mut col2) = parse_input();

    // Sort the two vectors in ascending order
    col1.sort();
    col2.sort();

    // Find the |abs| of the difference between the two vectors
    let distance: i32 = col1
        .iter()
        .zip(col2.iter())
        .map(|(&a, &b)| (a - b).abs())
        .sum();

    println!("Distance: {}", distance);
}

pub fn day1_2() {
    let (mut col1, mut col2) = parse_input();
    
    // Count frequences of number
    let mut counter1 = HashMap::new();

    for item in &col1 {
        *counter1.entry(item).or_insert(0) += 1;
    }

    let mut counter2 = HashMap::new();

    for item in &col2 {
        *counter2.entry(item).or_insert(0) += 1;
    }

    let mut total = 0;

    for (key, value) in &counter1 {
        // println!("Key: {}, Value: {}", key, value);
        let v2 = counter2.get(key).unwrap_or(&0);
        total += (*key) * value * v2;
    }

    println!("Similarity: {}", total);
}