
//Enums
enum Direction{
    Up, 
    Down,
    Left,
    Right
}

enum Colors {
    Blue,
    Yellow,
    Black
}

enum Flavor {
    Sparking,
    Sweet,
    Fruity


}


struct Drink {
    flavor: Flavor,
    fluid_oz: f64
}

fn print_drink(drink: Drink){
    match drink.flavor {
        Flavor::Sparking => println!("flavor: sparky"),
        Flavor::Sweet => println!("Flavor: sweet"),
        Flavor::Fruity => println!("Flavor: fruity"),

    }

    println!("oz {:?}", drink.fluid_oz);

}



fn my_favorite_color(my_color:Colors){

   match my_color{
    Colors::Blue => println!("Not my Blue is not favorite color"),
    Colors::Yellow => println!("Yes is Yellow my favorite color"),
    Colors::Black => println!("Not Black is not my favorite color"),

   } 
}



fn main() {

   
    let go = Direction::Left;

    match go {
        Direction::Up => println!("Go Up"),
        Direction::Left => println!("Go Left"),
        Direction::Down => println!("Go Down"),
        Direction::Right => println!("Go Right")
    };

    // exec second enum colors
    my_favorite_color(Colors::Yellow);

    // struct 

    let sweet = Drink{
        flavor : Flavor::Sweet,
        fluid_oz : 6.0
    };
    print_drink(sweet);

    let fruity = Drink{
        flavor : Flavor::Fruity,
        fluid_oz : 10.0

    };
    print_drink(fruity);

    let sparking = Drink{
        flavor : Flavor::Sparking,
        fluid_oz : 4.0

    };
    print_drink(sparking);

}
