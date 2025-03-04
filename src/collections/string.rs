pub fn string_example_tostring() {
    // let v = vec![1, 2, 3];
    // let s = String::from(v);
    // let s = v.to_string();
    let mut s = String::new();
    s.push_str("something like that");
    s.push('\'');
    println!("s is {s}");

    // // Конкатенация
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // //Вспоминаем ownership

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}