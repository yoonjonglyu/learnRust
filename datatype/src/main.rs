fn main() {
    /*
       러스트의 정수형은 8, 16, 32, 64비트로 u(정수),i(음수,정수)가 있다.
       Rust의 정수 기본값은 i32이고 이게 가장 빠르다. 각 자료형은 무조건 정해진 크기를 차지한다.
       모든 정수형 리터럴은 57u8과 같은 타입 접미사와 1_000과 같이 시각적인 구분을 위한 _의 사용을 허용
       Decimal	98_222
       Hex	0xff
       Octal	0o77
       Binary	0b1111_0000
    */
    let a = -20; // i32
    let b = 20u32; // u32
    println!("{}, {}", &a, &b);
    /*
        러스트에서 부동소수점의 경우 32, 64 f타입이 있다.
        기본타입은 f64 그 이유는 최신의 CPU 상에서는 f64가 f32와 대략 비슷한 속도를 내면서도 더 정밀한 표현이 가능하기 때문이다.
        f32 타입은 1배수의 정밀도, f64는 2배수의 정밀도, IEEE-754 표준
    */
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{}, {}", &x, &y);
    /* 수학적 연산 
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
    */
    // 논리 연산자는 true, false만 가능 bool로 명시하거나 기본 값
    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("{},{}", &t, &f);
    /*
        스트링이 큰따옴표를 쓰는 것에 반하여 char 타입은 작은따옴표로 쓴다.
        Rust의 char타입은 Unicode Scalar를 표현하는 값이고 이는 ASCII 보다 많은 표현을 가능
    */
    let c: char = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}", &c, &z, &heart_eyed_cat);
    /*
        Rust는 두 개의 기본 타입들을 갖고 있습니다: 튜플과 배열.
        튜플은 다양한 타입의 몇 개의 숫자를 집합시켜 하나의 복합 타입
        변수 tup에는 튜플 전체가 bind, 값을 튜플에서 분리시키기 위해서는 패턴 매칭을 통해서 구조해체해야함
        또는 .를 토해서 접근가능
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (h, j, k) = tup;
    println!("{}, {}, {}, {}", &h, &j, &k, &tup.0);
    /*
        배열은 튜플과 다르게 내부 요소가 모두 같은 타입이여야한다.
        또 Rust의 배열이 몇 다른 언어들의 배열과 다른 점은 Rust에서는 배열은 고정된 길이를 갖는다
        배열이 유용할 때는 당신의 데이터를 heap보다 stack에 할당하는 것을 원하거나 항상 고정된 길이를 가질때다.
        배열과 달리 벡터 타입은 유사 집합체로 표준 라이브러리에서 제공되며 확장 혹은 축소가 가능합니다.
        다른 언어와 같이 []로 접근가능하다.
    */
    let arr1 = [1, 2, 3, 4, 5];
    println!("{}", &arr1[0]);
}
