// Сумматор
// Напишите программу, которая считывает два целых числа и выводит их сумму.
use std::io;
fn main() {
    let mut input_string = String::new();
    let mut input_string_1 = String::new();
    io::stdin().read_line(&mut input_string).expect("Не удалось прочитать строку");
    let a = input_string.trim().parse::<i32>().expect("Это не число");
    io::stdin().read_line(&mut input_string_1).expect("Не удалось прочитать строку");
    let b = input_string_1.trim().parse::<i32>().expect("Это не число");
    println!("{}", a + b);
}