// example generics 

// enum Mouse{
//     LeftClick,
//     RightClick,
//     MiddleClick,
//     Scroll(i32),
//     Move(i32, i32),

// }

// enum PromoDiscount{
//     NewUser,
//     Holliday(String),
// }

enum Discount{
    Percent(f64),
    Flat(i32),
    //Promo(PromoDiscount),
    //Custom(String),

}

// let numbers: Vec<i32> = vec![1,2,4];
// let letters : Vec<char> = vec!['a','b','c'];
// let clicks  : Vec<Mouse> =  vec![
//     Mouse::LeftClick
// ];


//Advanced Match
struct Ticket{
    event: String,
    price : i32,

}
fn main() {
    let n : i32 = 4;

    match n  {
        4 => println!("four"),
        _ => println!("Number: {:?}",n )
    };
    let flat = Discount::Flat(2);

    match flat {
        Discount::Flat(2) => println!("Flat 2"),
        Discount::Flat(amount) => println!("flat discount {:?}", amount),
        _ => ()
    };

    let concert = Ticket{
        event: "concert".to_owned(),
        price : 300,
    };

    match concert {
        Ticket{price: 300, event} => println!("event @ 300 {:?}", event),
        Ticket{price, ..} => println!("price = {:?}", price),

    };
    
}
