enum Result<T,E>{
    Ok(T),
    Err(E),
}

fn get_sound(name: &str) -> Result<SoundData, String>{
    if name == "name"{
        Ok(SoundData::new("alert"));
    }else{
        Err("no such sound".to_owned());
    }

    let sound = get_sound("alert");

    match sound {
        Ok(_) => println!("ok"),
        Err(e)=>{println!("error {:?}", e)}, 
        
    };

}
fn main() {
    println!("Hello, world!");
}
