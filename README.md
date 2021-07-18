# Rust 공부

공부하자!

## cargo

1. cargo로 프로젝트 만들기.
```cargo
cargo new [project name] --bin
```
2. cargo로 프로젝트 빌드하기.
```cargo
cargo build // 실행파일 생성
cargo check // 실행파일은 생성하지 않으나 컴파일 가능 여부 체크
cargo build -- release // 릴리즈 빌드 최적화까지 포함한 옵션
```
3. cargo로 빌드한 프로젝트 실행하기.
```cargo
cargo run
```