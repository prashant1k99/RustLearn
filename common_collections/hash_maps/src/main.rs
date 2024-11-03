// We need to bring the hashmaps to scope
use std::collections::HashMap;
use std::iter::Copied;

fn main() {
    let mut score = HashMap::new();

    let blue_team = String::from("blue");
    let yellow_team = String::from("yellow");

    score.insert(&blue_team, 10);
    score.insert(&yellow_team, 20);

    println!("HashMap = {:?}", score);
    // HashMap = {"yellow": 20, "blue": 10}

    // To get the value from the hashmaps:
    let blue_score = score.get(&blue_team);
    match blue_score {
        Some(value) => println!("{value}"),
        None => println!("No value found"),
    };
    // 10

    // Or we can simply unwrap it
    let yellow_score = score.get(&yellow_team).copied().unwrap_or(0);
    // We are doing copied so that we do not have to handle references and it will give us a copy
    // of the value
    println!("{yellow_score}");
    // 20

    // To loop over a hash map:
    for (key, value) in &score {
        println!("{key} has score {value}");
    }
    // blue has score 10
    // yellow has score 20

    // To overwrite a value for a key:
    score.insert(&blue_team, 100);
    // This will overwrite the current value to the new value
    println!(
        "blue score: {}",
        score.get(&blue_team).copied().unwrap_or(0)
    );
    // blue score: 100

    // Now to only add value if the key does not exists in hash map:
    score.entry(&blue_team).or_insert(20);
    println!(
        "blue score: {}",
        score.get(&blue_team).copied().unwrap_or(0)
    );
    // blue score: 100

    // Update the value based on the old value:
    let text = "hello world wonderful world";
    let mut text_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("text_map: {:?}", text_map);
    // text_map: {"wonderful": 1, "hello": 1, "world": 2}

    // HashMap uses hashing function called SipHash that can provide resistance of
    // denial-of-service attacks invloving hash tables. This is not the fastest alggorithm
    // available, but the trade-off for better security that comes iwth the drop in performance is
    // worth it.
    // We can change the hashing function if we feel this is slow for us using BuildingHash
}
