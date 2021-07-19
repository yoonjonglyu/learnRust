extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자 맞추기");
    println!("당신이 생각한 숫자를 입력해주세요.");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new(); // 러스트의 모든 변수는 기본적으로 불변 mut키워드로 가변 변수화한다.

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");// io::result로 받아서 처리하는 과정에서 에러가 발생할시 에러 핸들링. 없으면 컴파일시 에러발생.
        let guess: u32 = match guess.trim().parse() { // 변수를 재선언 하지않고 shadow가 가능하다는 특징... 형변환시 대부분 이렇게 처리하나보다.
                Ok(num) => num,
                Err(_) => continue,
            }; // expect 대신 match를 이용하여서 이렇게 처리도 가능하다.
        println!("당신의 추측은: {}", guess); // {}를 이용해서 변수를 치환한다. 기존 언어랑 기본 개념은 비슷한듯
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("더 적다."),
            Ordering::Greater => println!("더 크다."),
            Ordering::Equal => {
                println!("당신이 맞췄다.");
                break;
            }
        }
    }
}
