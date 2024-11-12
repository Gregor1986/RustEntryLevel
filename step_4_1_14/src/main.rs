// Конвертер
// В мире компьютеров существует множество различных систем счисления, таких как двоичная, восьмеричная и 
// шестнадцатеричная. Каждая из них имеет свои особенности и применения. Иногда требуется конвертировать числа 
// из одной системы счисления в другую.
// Напишите программу, которая считывает целое число с консоли в десятичной системе и выводит его в двоичной, 
// восьмеричной и шестнадцатеричной системах счисления с префиксом.
use std::io;
fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Не удалось прочитать строку");
    let a = input_string.trim().parse::<i32>().expect("Это не число"); 
    println!("{:#b}", a);
    println!("{:#o}", a);
    println!("{:#x}", a);
}