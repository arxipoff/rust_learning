use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Угадай число!");

  let secret_number = rand::thread_rng().gen_range(1, 101);
  //println!("Твой вариант: {}", secret_number);

  loop {

    println!("Пожалуйста введите число: ");
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("Failed to read line!");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("Вы ввели число: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Слишком маленькое!"),
      Ordering::Greater => println!("Слишком боьшое!"),
      Ordering::Equal => {
        println!("Вы выиграли!");
        break;
      }
    }

  }  

}
