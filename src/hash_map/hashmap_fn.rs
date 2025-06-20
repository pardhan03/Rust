use std::collections::{ HashMap } ;

pub fn hash_func(){

    let mut scores: HashMap<String, i32>=HashMap::new();

    let blue_team =String::from("blue");
    let yellow_team =String::from("yellow");

    scores.insert(blue_team, 10);
    scores.insert(yellow_team, 20);

    let score = scores.get(&String::from("blue"));

    for (key, value) in &scores {
    println!("Key: {:?}, Value: {:?}", key, value);
}

    println!("in this we will learn about Hashmap")
}