#[derive(Debug)]

enum MenuChoice{
    MainMenu,
    Start,
    Quit,
}


fn get_choice(input : &str) -> Result<MenuChoice, String>{
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()),
    }
}


fn print_choice(choice: &MenuChoice){
    println!("choice: {:?}", choice);


}



fn pick_choice(input: &str) -> Result<(), String>{
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}



fn main (){
    let choice = get_choice("mainmenu");
    match choice {
        Ok(inner_choise) => print_choice(&inner_choise),
        Err(e) => println!("error = {:?}", e),
        
    }
    
    
}


