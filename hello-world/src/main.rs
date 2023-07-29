fn main() {
    firts_name();
    last_name();
    let sum = add(8,2);
    println!("The sum is {}", sum);
    let subst = sub(150,20);
    display_result(subst);
}

fn firts_name(){
    let name : &str = "Pepe";
    println!("My name is {} ",name);

}

fn last_name(){
    let lastName : &str = "Perez";
    println!("My lastName is {}", lastName);
}

// aritmetics
fn sub (a: i32, b:i32) ->i32{
    return a - b;

}

fn add (c:i16, d:i16) -> i16{
    return c + d;
}
fn display_result(result:i32){
    println!("{:?}",result);


}