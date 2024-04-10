fn main() {
    let x = 5;
    let x = x+1;

    {
        let x= x * 2;
        println!("The value of x in the inner scope is : {x}"); //12
    } //x한테 2배한 게 여기서까지만 적용되고 사라진다.
    
    println!("The value of x is : {x}"); //6

    let spaces = "   "; //이제 여기에 스코프 끼워서 spaces 쓰고 스코프 닫으면 되는 거임
    let spaces = spaces.len(); //이건 가능

    let mut spacse = "   ";
    spaces = spaces.len(); //이건 불가능
}
