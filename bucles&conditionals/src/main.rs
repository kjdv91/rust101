fn main() {
    
    //Flow control if else
    let my_bool:bool  = true;

    if my_bool == true{
        println!("Hello");
    }else{
        println!("GoodBye");
    }

    //flow control if else if else

    let num: i32 = 7;

    if num > 7 {
        println!("Greater than 7");
    } else if num < 7 {
        println!("Less than 7");
    }else{
        println!("equal to 7");
    }

    //Match

    let some_int : i32 = 7;
    match some_int{
        1 => println!("Match one"),
        2 => println!("Match two"),
        3 => println!("Yes, match number"),
        4 => println!("Match four"),
        // else 
        _ => println!("Nice to meet you"),

    }

    // match string
    let my_name = "Bob";
    match my_name {
        "Pepe" => println!("My name is Pepe"),
        "Lupe" => println!("My name is Lupe"),
        "Bob" => println!("Yes win"),
        _ => println!("Other number"),

    }

    //Decision making with match

    //loops
    let mut n :i32 = 1;
    loop {
        println!("El valor es {:?}", n);
        if n == 4{
            break;  // break to exit the loop
        }
        n = n +1;
        
        
    }

    // while 

    let mut numb2 : i16 = 1;

    while numb2 <12{
        println!("While {:?}", numb2);
        numb2 = numb2 +1;

    }

}
