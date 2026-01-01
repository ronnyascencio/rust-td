pub fn say_hello () {
    println!("hello from uno")
}


pub fn shot_name(name: &str, department: &str, version: i32) -> String{
    
    let sn: String = format!("{}_{}_v{:03}", name, department, version);
    let name_upper: String = name.to_uppercase();

    let new_name: String = sn.replace(name, &name_upper);

    new_name
    

    
}