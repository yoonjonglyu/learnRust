fn main() {
    const MAX_POINTS: u32 = 100_000; // 러스트에서 상수는 대문자로 스네이크 케이스를 사용한다. 당연히 예약어는 선언 불가능하다.
    println!("The Max Point: {}", MAX_POINTS);
    let mut x = 5; // mut를 통해서 불변 변수를 가변 변수로 변경 가능하다.
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    let y = 5; // 한번 사용 전까지는 shadowing이 가능하다. 다만 새도잉 하는 코드를 기존 코드에서 참조하지 않으면 에러가 나온다.
    let y = y + 555; // cargo 에서는 선언한 변수가 사용 되지 않는 상황을 메모리 낭비로 보고 잡아주는 것 같다.
    println!("The value of y is {}", y);
}
