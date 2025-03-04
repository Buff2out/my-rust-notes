pub fn hashmap_example() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    dbg!(&scores);
    println!("{scores:#?}");
}
