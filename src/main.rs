use std::io::stdin;

fn main() {
    println!("Введите трёхзначное неотрицательное число:");
    let x: u16 = read_var();

    let rang = 100..999;

    if rang.contains(&x) {
        let x: Vec<u16> = num_to_vec(x);
        let mut y: u16 = 0;

        for i in x {
            y += i;
        }

        let x: u16 = y % 2; 

        if x == 0 {
            println!("Сумма цифр чётна и равна {y}");
        } else {
            println!("Сумма цифр нечётна и равна {y}");
        }
        pause();
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
    let mut dig = Vec::new();
    let mut n = n;
    while n > 9 {
        dig.push(n % 10);
        n = n / 10;
    }
    dig.push(n);
    dig.reverse();
    dig
}

fn pause() { //фукция паузы
    println!("нажмите Enter чтобы выйти.");
    let mut q = String::new();
    stdin().read_line(&mut q).expect("ошибка");
}