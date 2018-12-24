use std::collections::HashMap;
mod practise;

fn main() {
    hash_map();
    hash_map_update_value();
    hash_map_count_words();

    // practise
    let input = vec![1, 3];
    let mean = practise::vector::find_mean(&input);
    if let Some(_) = mean {
        println!("mean is {:?}", mean);
    }

    let median = practise::vector::find_median(&input);
    if let Some(_) = median {
        println!("median is {:?}", median);
    }
}

fn hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 20);

    // let team_name = String::from("Green");
    let score = scores.get("Green");

    if let Some(_) = score {
        println!("{:?}", score);
    };

    // loop thru the hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn hash_map_update_value() {
    println!("Starting hash_map_update_value");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn hash_map_count_words() {
    let text = String::from("Hello world wonderful world");
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
