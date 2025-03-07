use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// Всегда реализуется для любого T
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Реализуется только для T, поддерживающих сравнение (PartialOrd) и вывод (Display)
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Наибольший элемент: x = {}", self.x);
        } else {
            println!("Наибольший элемент: y = {}", self.y);
        }
    }
}