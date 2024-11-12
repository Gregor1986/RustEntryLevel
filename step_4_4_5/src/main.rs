// Смайл Фейс
// Напишите программу, которая считывает строку, представленная в виде эмодзи и выводит его обратно следующим 
// образом:
// Мое настроение: эмодзи
use std::io;
fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Не удалось прочитать строку");
    let a = input_string.trim().parse::<char>().expect("Это не число"); 
    println!("Мое настроение: {}", a);
} 