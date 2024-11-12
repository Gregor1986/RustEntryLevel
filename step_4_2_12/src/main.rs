// Напишите программу, которая считывает два вещественных числа и выводит основные арифметические операции для 
// них: сложение, вычитание, умножение, деление и нахождение остатка от деления. Для операций / и % результат 
// вывести до 3 знаков, а  остальные без дробной части. Гарантируется, что второе число не равно 0!
use std::io;
fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Не удалось прочитать строку");
    let a = input_string.trim().parse::<f64>().expect("Это не число"); 
    let mut input_string_1 = String::new();
    io::stdin().read_line(&mut input_string_1).expect("Не удалось прочитать строку");
    let b = input_string_1.trim().parse::<f64>().expect("Это не число");
    println!("{} + ({}) = {}", a as i32, b as i32, a + b);
    println!("{} - ({}) = {}", a as i32, b as i32, a - b);
    println!("{} * ({}) = {}", a as i32, b as i32, a * b);
    println!("{} / ({}) = {:.3}", a as i32, b as i32, a / b);
    println!("{} % ({}) = {:.3}", a as i32, b as i32, a % b);
} 
