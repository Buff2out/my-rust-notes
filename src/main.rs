fn main() {
    let x: Option<i8> = Some(5);
    let y: Option<i8> = Some(5);

    let sum = match (x, y) {
        (Some(a), Some(b)) => Some(a + b), // Если оба значения есть, складываем их
        _ => None,                       // В остальных случаях возвращаем None
    };

    println!("{:?}", sum); // Выводит: Some(10)
}