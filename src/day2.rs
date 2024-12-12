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

fn parse_input() -> impl Iterator<Item = Vec<i32>> {
    let input = include_str!("../data/day2_1.txt");

    input.lines().map(|line| {
        line.split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect()
    })
}

pub fn count_safe_reports() {
    let safe_reports = parse_input()
        .filter(|report| report_is_safe(report.to_vec()))
        .count();

    println!("Safe reports: {}", safe_reports);
}

pub fn count_tolerate_reports() {
    // If the report fails, try removing a number (one at a time) to see if it succeeds

    let mut count = 0;
    for report in parse_input() {
        if report_is_safe(report.to_vec()){
            count += 1;
        } else {
            for i in 0..report.len(){
                let mut vec = report.to_vec();
                vec.remove(i);
                if report_is_safe(vec){
                    count += 1;
                    break;
                }
            }
        }
    }

    println!("Tolerable safe reports: {}", count);
}