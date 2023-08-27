struct LineItem{
    name: String,
    count : i32,
}

fn print_name (name :&str){
    println!("name : {:?}", name);

}

fn print(data : &str){
    println!("{:?}", data);
}


// exercise 
/*
- Use a struct for a persons, age, name and favorite color
- The color and name should be store as a String
- Create and store at the least 3 people in a vector
- Iterate throught the vector using a for in a lopp
- Use an if expression to determine wich person info should be printed
- Yhe name and colors should be printed using a function

*/

struct Person{
    name: String,
    color: String,
    age : i32,
}



fn main() {
    let receipt = vec![
        LineItem{
            name : "cereal".to_owned(),
            count : 1,
        },

        LineItem{
            name : String::from("fruit"),
            count : 3,
        }
    ];


    for item in receipt{
        print_name(&item.name);
        println!("Name {:?},  count{:?} ",item.name, item.count );
    }


    //Exercise

    let people = vec![
        Person{
            name: String::from("Pepe"),
            color: String::from("green"),
            age: 25,
        },

        Person{
            name: String::from("Juan"),
            color: String::from("black"),
            age:  22
        },
        Person{
            name: String::from("Juanito"),
            color: String::from("red"),
            age: 10,

        },
    ];


    for person in people{
        if person.age  <=10 {
            print(&person.name);
            print(&person.color);

        }
    }
}
