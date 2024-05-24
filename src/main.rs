use std::collections::HashMap;

fn main() {
    let mut v = vec![6, 1, 2, 3, 4, 5, 6, 6];

    println!("The median is: {}", sort_and_find_median(&mut v));
    println!("The modes are: {:?}", find_mode(&mut v));
}

fn sort_and_find_median(v: &mut Vec<i32>) -> f64 {
    v.sort();

    let max_index = v.len() - 1;
    let median;
    
    if max_index % 2 == 1 {
        let average_index = max_index as f64 / 2.0;
        median =
            (v[average_index.floor() as usize] + v[average_index.ceil() as usize]) as f64 / 2.0;
    } else {
        let average_index = max_index / 2;
        median = v[average_index] as f64;
    }

    median
}

fn find_mode(v: &Vec<i32>) -> Vec<i32> {
    let mut modes = Vec::new();
    let mut counts = HashMap::new();

    for i in v {
        let count = counts.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    for (_, value) in &counts {
        if *value > max_count {
            max_count = *value;
        }
    }

    for (key, value) in counts {
        if value == max_count {
            modes.push(key);
        }
    }

    modes
}
