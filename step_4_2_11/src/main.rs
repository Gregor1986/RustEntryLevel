// Копейка рубль бережёт 
// Напишите программу, которая считывает вещественное число и выводит его целое и дробную часть (до 3 знаков).
// Например, если введено число 123.456789, программа должна вывести:
// 123
// 0.457
use std::io;
fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Не удалось прочитать строку");
    let a = input_string.trim().parse::<f64>().expect("Это не число"); 
    let c = a as i32;
    println!("{}", c);
    println!("{:.3}", a - c as f64);
} 
