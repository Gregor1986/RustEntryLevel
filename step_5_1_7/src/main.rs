// Упаковщик
// Напишите программу, которая считывает пять строк, сохраняет их в той же последовательности в кортеж и затем выводит получившуюся коллекцию с помощью спецификатора формата :?.
use std::io;
fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Не удалось прочитать строку");
    let a = input_string.parse().expect("Это не число"); 
    let mut input_string_1 = String::new();
    io::stdin().read_line(&mut input_string_1).expect("Не удалось прочитать строку");
    let b = input_string_1.parse().expect("Это не число");
    let mut input_string_2 = String::new();
    io::stdin().read_line(&mut input_string_2).expect("Не удалось прочитать строку");
    let c = input_string_2.parse().expect("Это не число");
    let mut input_string_3 = String::new();
    io::stdin().read_line(&mut input_string_3).expect("Не удалось прочитать строку");
    let d = input_string_3.parse().expect("Это не число");
    let mut input_string_4 = String::new();
    io::stdin().read_line(&mut input_string_4).expect("Не удалось прочитать строку");
    let e = input_string_4.parse().expect("Это не число");
    let mut tup = ("".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string());
    tup.0 = a;
    tup.1 = b;
    tup.2 = c;
    tup.3 = d;
    tup.4 = e;
    println!("{:?}", tup);
} 