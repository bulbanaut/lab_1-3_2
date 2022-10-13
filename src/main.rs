use std::io::stdin;

fn main() {
    println!("Введите трёхзначное неотрицательное число:");
    let x: u16 = read_var();

    let rang = 0..999;

    if rang.contains(&x) {
        let x: Vec<u16> = num_to_vec(x);
        let mut y: u16 = 0;
        for i in x {
            y += i;
        }
        println!("{y}")
    } else {
        println!("Переменная дожна быть трёхзначной");
        main();
    }
}

fn read_var() -> u16 {
    loop {
        let mut x =String::new();
        stdin()
            .read_line(&mut x)
            .expect("Ошибка чтения ввода"); //считывание ввода и запись его в переменную X (с точки зрения компилятора это одна строка)

        let x: u16 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Переменная должна быть равна неотрицательному числу");
                continue;
            },
        }; //преобразование ввода со string в u16 с перезапуском loop в случае ошибки
        break x;
    }
}

fn num_to_vec(n: u16) -> Vec<u16> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}