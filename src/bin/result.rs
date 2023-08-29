#[derive (Debug)]

struct Adult{
    age: u8,
    name: String,
}
impl Adult{
    fn new(age: u8, name: &str) -> Result<Self , &str>{
        if age >= 21{
            Ok(Self{
                age,
                name: name.to_string()
            })
        }else{
            Err("age must be at least 21")
        }
    }
}


fn main(){

    let child = Adult::new(15,"Aniket");
    let adult = Adult::new(21,"sanjay");
match child{
    Ok(child) => println!("{} is {} year old", child.name, child.age),
    Err(e) => println!("{e}"),
}

match adult{
    Ok(a) => println!("{} is {} year old", a.name, a.age),
    Err(e) => println!("{e}"),
}
}