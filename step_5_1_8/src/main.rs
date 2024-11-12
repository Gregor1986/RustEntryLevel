// Корректор
// В редакторе кода представлен кортеж с числами. Данные значения были получены по сети, но во время передачи 
// произошло разъединение соединения. Дополните программу считыванием пяти вещественных чисел. Замените каждое 
// старое значение на новое, начиная с первого элемента, и выведите целые части у чисел в кортеже, как показано 
// в примере.
use std::io;
fn main() {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Не удалось прочитать строку");
    let a = input_string.trim().parse().expect("Это не число"); 
    let mut input_string_1 = String::new();
    io::stdin().read_line(&mut input_string_1).expect("Не удалось прочитать строку");
    let b = input_string_1.trim().parse().expect("Это не число");
    let mut input_string_2 = String::new();
    io::stdin().read_line(&mut input_string_2).expect("Не удалось прочитать строку");
    let c = input_string_2.trim().parse().expect("Это не число");
    let mut input_string_3 = String::new();
    io::stdin().read_line(&mut input_string_3).expect("Не удалось прочитать строку");
    let d = input_string_3.trim().parse().expect("Это не число");
    let mut input_string_4 = String::new();
    io::stdin().read_line(&mut input_string_4).expect("Не удалось прочитать строку");
    let e = input_string_4.trim().parse().expect("Это не число");
    let mut tup = (10.0, 5.0, -2.0, 100.0, 2000.0, 0.0);
    tup.0 = a;
    tup.1 = b;
    tup.2 = c;
    tup.3 = d;
    tup.4 = e;
    println!("{}, {}, {}, {}, {}, {}", tup.0 as i32, tup.1 as i32, tup.2 as i32, tup.3 as i32, tup.4 as i32, tup.5 as i32);
} 