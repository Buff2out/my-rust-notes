pub fn vec_example_get() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(el) => println!("The third element is {el}"),
        None => println!("There is no third element."),
    }
}
