use std::collections::HashMap;

fn main() {
    // median mode exercise the function could be improve
    println!(
        "The result of median_mode {:?}",
        median_mode(vec![11, 2, 12, 5, 53, 100, 100, 100, 23])
    );
}

fn median_mode(mut arr: Vec<i32>) -> (i32, i32) {
    arr.sort();
    let count = arr.len();
    let middle_index = count.div_ceil(2) - 1;
    let median = arr[middle_index];

    let mut map: HashMap<i32, i32> = HashMap::new();
    for value in arr {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    let mut current_mode = 0;
    for (key, value) in &map {
        if value > map.get(&current_mode).unwrap_or(&0) {
            current_mode = *key;
        }
    }

    (median, current_mode)
}
