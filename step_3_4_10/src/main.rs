// Напишите программу, которая считывает имя и затем выводит приветственное сообщение с использованием этого имени:
// Привет, {имя}!
use std::io;
fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Не удалось прочитать строку");
    println!("Привет, {}!", input_string.trim());
}