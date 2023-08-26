struct Test{
    score : i32,
}

fn main(){
    let my_score = vec![
        Test {score : 98},
        Test {score : 87},
        Test {score : 78}
    ];
    for test in my_score{
        println!("score = {:?}",test.score);
    }
}