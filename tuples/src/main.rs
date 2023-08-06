// enum Access{
//     Full,
// }


// fn numbers () -> (i32,i32,i32){
//     (1,2,3);
// }


// let ns = numbers;
// let (x,y,z) = ns;

// println!("{:?}, {:?}", x, numbers.0 );
// println!("{:?}, {:?}", y, numbers.1);
// println!("{:?}, {:?}", z, numbers.2);

// let (employee, access) = ("Thiago", Access::Full);

fn coordenates() -> (i32,i32){
    (1,8)
}


fn main() {


    let (a, b) = coordenates();

    if b > 4 {
        println!(">4");
    }else if b < 4 {
        println!("<4");
    }else{
        println!("=4");
    }
    let coord = (2,3);
    println!("{:?}, {:?}", coord.0, coord.1);


    let (x,y) = (10,20);
    println!("{:?}, {:?}", x, y);

    let (name,age) = ("Emmna",28);

    let favorites = ("Yellow",31, "Pizza", "River plate");
    let color = favorites.0;
    let fanatics = favorites.3;


}
