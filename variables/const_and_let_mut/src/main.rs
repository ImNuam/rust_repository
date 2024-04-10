fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3; //상수는 무조건 대문자로 쓰기
    //상수는 선언된 스코프 내에서 프로그램이 동작하는 전체 시간 동안 유효(사실상 처음부터 끝까지)
    let mut x = 5; //얘는 immut(불변) 변수 
    println!("The value of x is: {x}");
    x=7;
    println!("The value of x is: {x}");
}
