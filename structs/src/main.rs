struct Person{
    name: String,
    age: i32,
    weight: f64,
    friend: String,
}



impl Person {
    fn declare_friend(&self, descriptor: String){
        println!("{} is {} friends with {}", self.name,descriptor, self.friend);

    }
} 


fn main() {
    let bob: Person = Person{
        name : String::from("Bob"),
        age : 35,
        weight : 150.5,
        friend : String::from("Luna"),
    };

    bob.declare_friend(String::from("good"));

}
