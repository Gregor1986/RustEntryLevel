// Третий лишний
// Если бы все программисты эффективно использовали ресурсы, то внутренняя память наших устройств не забивалась 
// бы на 80% в течение "двух" дней использования приложений и программ.
// Напишите программу, которая считывает два целых числа и меняет их значения местами без использования третьей
// переменной.
// Так, если переменные a = 5, b = 10, то после обмена значениями должно стать a = 10, а b = 5 без использования
// дополнительной переменной.
use std::io;
fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Не удалось прочитать строку");
    let a = input_string.trim().parse::<i8>().expect("Это не число"); // 2
    let mut input_string_1 = String::new();
    io::stdin().read_line(&mut input_string_1).expect("Не удалось прочитать строку");
    let b = input_string_1.trim().parse::<i8>().expect("Это не число"); // 3
    let a = a - b; 
    let b = a + b; 
    let a = b - a;
    println!("{}", a);
    println!("{}", b);
} 