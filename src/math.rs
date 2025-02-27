pub mod geometry; // Подключаем подмодуль geometry

pub fn calculate_area(side: i32) -> i32 {
    geometry::shapes::area_of_square(side)
}