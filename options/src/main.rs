struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,

}
// topic: Option
/*Requirements
Print out the details of a student's locker assignment
Lockers use a number and are optional for students

Notes:
Use a struct containing the student name and locker assignment
The locker assignment should use a Option<i32>
*/

struct Student{
    name : String,  //student name
    locker : Option<i32>,

}

fn main() {
    let response = Survey{
        q1: None,
        q2: Some(true),
        q3: Some("A".to_owned()),
    };

    match response.q1 {
        Some(ans) => println!("Q1 is {:?}", ans),
        None => println!("Q1: no response"),
    };

    match response.q2 {
        Some(ans) => println!("Q2 is {:?}",ans) ,
        None => println!("Q2: no response"),
    };

    match response.q3 {
        Some(ans) => println!("Q3 is {:?}", ans) ,
        None => println!("Q3: no response"),
    };

    let carlo = Student{
        name:"Carlos".to_owned(),
        locker:Some(114),
    };
    println!("student name {:?}",carlo.name);

    match carlo.locker {
        Some(num) => println!("locker num {:?}", num),
        None => println!("No locker assigned"),
    };
}
