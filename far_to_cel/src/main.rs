use std::io;

fn main() {
    println!("Введите количество градусов по Фаренгейту");

    let mut far = String::new();

    io::stdin()
        .read_line(&mut far)
        .expect("Failed to read line");

    let far: u32 = match far.trim().parse() {
        Ok(num) => num,
        Err(_) => todo!()
    };

    let cel: f64 = ((far-32)*5/9).into();

    println!("{}", cel)
}
