// Арифметика систем
// Напишите программу, которая считывает два целых числа с консоли в десятичной системе и выводит в двоичной, 
// восьмеричной, шестнадцатеричной системах арифметические операции (+, -, *, /, %) над первым числом.
// Уточним, что каждая арифметическая операция должна быть выведена в каждой из систем счисления. При выводе 
// операций для следующей системы счисления используйте переход на новую строку. Для ясности посмотрите на 
// входные и выходные данные. Гарантируется, что второе число не равно 0!
use std::io;
fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Не удалось прочитать строку");
    let a = input_string.trim().parse::<i32>().expect("Это не число"); 
    let mut input_string_1 = String::new();
    io::stdin().read_line(&mut input_string_1).expect("Не удалось прочитать строку");
    let b = input_string_1.trim().parse::<i32>().expect("Это не число"); 
    println!("{:#b} + {:#b} = {:#b}", a, b, a + b);
    println!("{:#o} + {:#o} = {:#o}", a, b, a + b);
    println!("{:#x} + {:#x} = {:#x}", a, b, a + b);
    println!();
    println!("{:#b} - {:#b} = {:#b}", a, b, a - b);
    println!("{:#o} - {:#o} = {:#o}", a, b, a - b);
    println!("{:#x} - {:#x} = {:#x}", a, b, a - b);
    println!();
    println!("{:#b} * {:#b} = {:#b}", a, b, a * b);
    println!("{:#o} * {:#o} = {:#o}", a, b, a * b);
    println!("{:#x} * {:#x} = {:#x}", a, b, a * b);
    println!();
    println!("{:#b} / {:#b} = {:#b}", a, b, a / b);
    println!("{:#o} / {:#o} = {:#o}", a, b, a / b);
    println!("{:#x} / {:#x} = {:#x}", a, b, a / b);
    println!();
    println!("{:#b} % {:#b} = {:#b}", a, b, a % b);
    println!("{:#o} % {:#o} = {:#o}", a, b, a % b);
    println!("{:#x} % {:#x} = {:#x}", a, b, a % b);
} 