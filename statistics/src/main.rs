use std::collections::HashMap;

fn main() {
    let vals = vec![19, 8, 29, 35, 19, 28, 15, 8, 8];
    let mean = mean(&vals);
    let median = median(&vals);
    let mode = mode(&vals);

    println!("values: {:?}", vals);
    println!(" mean: {}", mean);
    println!(" median: {}", median);
    println!(" mode: {}", mode);
}

fn mean(vals: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for val in vals {
        sum += val;
    }

    sum / vals.len() as i32
}

fn median(vals: &Vec<i32>) -> i32 {
    let mut sorted = vals.clone();
    sorted.sort();

    let middle = sorted.len() / 2;
    sorted[middle]
}

fn mode(vals: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for val in vals {
        let count = counts.entry(*val).or_insert(0);
        *count += 1;
    }

    let mut i = 0;
    let mut mode = 0;
    for (val, count) in &counts {
        if i == 0 || count > &counts[&mode] {
            mode = *val;
        }

        i += 1;
    }

    mode
}
