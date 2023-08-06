struct Book{
    pages: i32,
    rating: i32
}

fn display_page_count(book: &Book){
    println!("pages ={:?}", book.pages);
}

fn display_raiting(book: &Book){
    println!("Rating the book  = {:?}", book.rating);
}


/*the owner variable  book is function main 
is in charge of managing memory
*/

// Topic Ownership
/* Requirements
Print out the quantity and id number of a grocery item
- Use a struct for the glocery item
Use two i32 fields for the quantity and id number
Create a function to display the quantity
Create a function to display id number 

*/

struct GloceryItem{
    quantity: i32,
    id_number: i32

}

fn display_quantity(item:&GloceryItem){
    println!("Quantity {:?}", item.quantity);
}

fn display_idNumbers(item: &GloceryItem){
    println!("IdNumbers {:?}", item.id_number);
}


fn main (){
    let book = Book{
        pages: 10,
        rating : 7
    };
    // ownership variable book is transferred to this function become the new owner
    display_page_count(&book);
    /* the program will crash because the variable no longer exists because it was already cleared from memory */
    display_raiting(&book);
    // error value used here after move

    //fix error we borrow memory borrow data

    let item = GloceryItem{
        quantity :150,
        id_number :2
    };
    display_quantity(&item);
    display_idNumbers(&item);

}