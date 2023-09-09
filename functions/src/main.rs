


fn nothing (){
    println!("No pasa nada");
}


fn function_return() -> u64{
    return 20_u64;
}

fn fun_params (p:u64) -> u64 {
    return p * 2;
}
#[allow(dead_code)]
fn funt_with_param_str(v:String) -> String {
    let new_string:String = String::from(v.as_str());
    return  new_string.to_uppercase();
} 

fn main() {
    nothing();
    
    let val_ret: u64 = function_return();
    println!("El valor del retorno es : {}", val_ret);

    let val_sol : u64 = 15_u64;
    let val_ret_para =  fun_params(val_sol);
    println!("El valor solicitado es -> {}", val_sol);
    println!("El valor del retorno {}", val_ret_para);

    
}
