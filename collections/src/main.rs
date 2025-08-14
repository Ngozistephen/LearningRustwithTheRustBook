use std ::collections:: HashMap;

fn main() {

    // let blue = String::from ("Blue");
    // let yellow = String::from ("yellow");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 2);
    scores.insert(String::from("yellow"), 15);


    println!("{:?}", scores);

    for (key, value) in &scores{
        println!("{key}: {value}");
    }
}
