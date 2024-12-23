use std::io;
fn main() {
    let mut far = String::new();

    io::stdin()
        .read_line(&mut far)
        .expect("Failed to read line");

    let far: u32 = match far.trim().parse() {
        Ok(num) => num,
        Err(_) => todo!()
    };

    println!("{}",fib(far.into()));
}


fn fib(n: i64) -> i64 {
    if n == 0{
        return 0
    } else if n == 1{
        return 1
    } else{
        let x = fib(n-1);
        let y = fib(n-2);
        return x+y
    }
}