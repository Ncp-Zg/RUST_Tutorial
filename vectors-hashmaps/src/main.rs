fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);

    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
    }
    println!("{:?}",v2);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
        
    }


    //String

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}",s1,s2,s3);

    println!("{}",s);

    for c in "Hello!".chars() {
        println!("{}",c);
    }
    
    for c in "Hello!".bytes() {
        println!("{}",c);
    }
    

    //HashMap

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(10) => println!("Blue team has {} points!", 10),
        _ => println!("There is no such team")
        
    }
    // println!("{:?}",score);

    let teams = vec![String::from("Blue"),String::from("Red")];
    let initial_scores = vec![10,50];

    let mut scores: HashMap<_,_> =teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}",scores);

    let field_name=String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}" , map);

}

