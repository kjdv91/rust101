
// Topic Advanced Match
/*Print out a list of tickets and their information for an event
Tickets can be backstage Vip an Standar
BackStage and Vip tickets include the ticcket holder's name 
All tickets include the price
Notes 
Use a enum for the tickets with data asociated woth each variant
Create one of eacch ticket and place into a vector
Use a match expression while iterating the vector to print the ticket information
Usea a enum for the tickets wit data associated with eacch variant

*/


enum Ticket{
    Backstage(f64,String),
    Vip(f64,String),
    Standar(f64)



}

fn main (){
    let tickets = vec![
        Ticket::Backstage(50.0, "May".to_owned()),
        Ticket::Vip(150.0, "Black".to_owned()),
        Ticket::Standar(15.0),
    ];

    for ticket in tickets{
        match ticket {
            // more a one line can you uses brackets
            Ticket::Backstage(price, holder) => {
                println!("Backstage Holder {:?}, price: {:?}",holder, price )
        },
        Ticket::Vip(price, holder) =>{
                println!("Vip Holder {:?}, price: {:?}",holder, price )
        }

        Ticket::Standar(price) => println!("Standar ticket price {:?}", price ),

        };
    }
}