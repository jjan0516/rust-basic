// 리터럴 문자열 조합
fn main() {
    println!("{} {}", "hello", "world");
    // "{} {}" : 포맷 문자열
    // {} : 위치 지정자 (placeholder)
    // "hello" : 데이터 1
    // "world" : 데이터 2

    // 줄바꿈
    println!("첫번째줄\n두번째줄\n세번째줄");
    print!("첫번째줄\n두번째줄\n세번째줄");

    // 정수 출력
    println!("숫자 : {}", 85);
    println!("숫자 : {}", "85");
    println!("숫자 : {}", 000085);
    println!("숫자 : {}", 1_000_585);
}
