struct GroceryItem{
    stock: i32,
    price: f64,
}

fn main(){
    let cereal = GroceryItem{
        stock: 10,
        price: 2.76,
    };
    println!("stock: {:?}",cereal.stock);
    println!("stock: {:?}",cereal.price);
}