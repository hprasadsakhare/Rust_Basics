enum Color{
    Red,
    Yellow,
    Blue
}

fn print_color(my_color:Color){
    match my_color{
        Color::Blue => println!("Blue"),
        Color::Red => println!("Red"),
        Color::Yellow => println!("Yellow")
    }
}
fn main(){
    print_color(Color::Blue);
}