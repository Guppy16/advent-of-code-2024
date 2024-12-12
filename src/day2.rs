fn report_is_safe(report: Vec<i32>) -> bool {
    // Returns true if the report is safe

    if report.len() < 2 {
        return true; // by default
    }

    // Check if first difference is positive or negative
    let mut is_increasing = false;
    if report[1] > report[0] {
        is_increasing = true;
    } else if report[1] < report[0] {
        is_increasing = false;
    } else {
        return false; // return false because report is neither incr or decr
    }

    // Check if remaining numbers increase or decrease by 1 - 3 (incl)
    for i in 1..report.len() {
        let difference = report[i] - report[i - 1];
        // print!("{} ", difference);

        if difference > 0 && !is_increasing {
            return false;
        } else if difference < 0 && is_increasing {
            return false;
        }

        let abs_difference = difference.abs();

        if abs_difference < 1 || abs_difference > 3 {
            return false;
        }
    }
    // println!();

    return true;
}


pub fn count_safe_reports() {
    // Read and parse the input file into vectors of int split by spaces

    let input = include_str!("../data/day2_1.txt");

    let mut safe_reports = 0;

    for line in input.lines() {
        // Split the line by whitespace and collect into a vector
        let str_numbers: Vec<&str> = line.split_whitespace().collect();

        // Parse the numbers into integers
        let mut numbers = Vec::new();

        for num in str_numbers {
            if let Ok(n) = num.parse::<i32>() {
                numbers.push(n);
            }
        }
        // println!("{:?}", numbers);

        if report_is_safe(numbers) {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {}", safe_reports);
}
