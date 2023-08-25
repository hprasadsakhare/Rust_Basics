struct Person{
    name: String,
    fav_color: String,
    age: i32,
}

fn print(data: &str){
    println!("{:?}",data);
}
fn main(){
    let people = vec![
        Person{
            name: String::from("George"),
            fav_color: String::from("green"),
            age: 7,
        },
        Person{
            name: String::from("anna"),
            fav_color: String::from("purple"),
            age: 8,
        },
        Person{
            name: String::from("rock"),
            fav_color: String::from("black"),
            age: 14,
        },
    ];

    for Person in people{
        if Person.age <= 10{
            print(&Person.name);
            print(&Person.fav_color);
        }
    }
}