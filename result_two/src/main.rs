
/*Topic Result
Requirement:

* The Err variant should contain a String or Str that explain why 
    the structure could not be created



*/
/*Create an structure name Adult thats represents a person age 21 or older:
The structure must contain the person's name and age
Implement Debug print functionality using derive*/
#[derive(Debug)]
struct  Adult {
    name :String,
    age:u8,
}

//Implements a new function 
//The ok variant should contain the initialized structure, but only 
//if the person is aged 21 or older

impl Adult{
    fn create_adult(name: &str, age: u8) -> Result<Self, &'static str> {
        if age >= 21 {
            Ok(Self {
                name: name.to_string(),
                age,
            })
        } else {
            Err("age must be at least 21")
        }
    }
}
// * Instantiate two Adults structure:
//     One should ve aged under 21
//     Two should be 21 or over



fn main (){
    // instanc new adult or child
    let child = Adult::create_adult("Anita", 16);
    let adult = Adult::create_adult("Andres", 22);

    // * use match to print out message for each Adult
    //  For the Ok Variant, print any message you want
    //  For The Err variant, print out the error message

    match child {
        Ok(resp) => println!("{} is {}  years old", resp.name, resp.age),
        Err(e) => println!("{}", e),
    }

    match adult {
        Ok(adult) => println!("{} is {}  years old", adult.name, adult.age),
        Err(e) => println!("{}", e),
    }

}