// Что еще за букоффки
// Напишите программу, которая считывает пять строк, представленные символами и выводит их в одну строку.
use std::io;
fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Не удалось прочитать строку");
    let a = input_string.trim().parse::<char>().expect("Это не число"); 
    let mut input_string_1 = String::new();
    io::stdin().read_line(&mut input_string_1).expect("Не удалось прочитать строку");
    let b = input_string_1.trim().parse::<char>().expect("Это не число");
    let mut input_string_2 = String::new();
    io::stdin().read_line(&mut input_string_2).expect("Не удалось прочитать строку");
    let c = input_string_2.trim().parse::<char>().expect("Это не число");
    let mut input_string_3 = String::new();
    io::stdin().read_line(&mut input_string_3).expect("Не удалось прочитать строку");
    let d = input_string_3.trim().parse::<char>().expect("Это не число");
    let mut input_string_4 = String::new();
    io::stdin().read_line(&mut input_string_4).expect("Не удалось прочитать строку");
    let e = input_string_4.trim().parse::<char>().expect("Это не число");
    println!("{}{}{}{}{}", a, b, c, d, e);
} 