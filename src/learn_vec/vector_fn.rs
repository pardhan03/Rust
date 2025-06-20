pub fn some_vector_fn() {
    let mut num_vec: Vec<i32> = Vec::new();

    num_vec.push(11);
    num_vec.push(12);
    num_vec.push(13);
    num_vec.push(14);

    let third: &i32 = &num_vec[2];

    let fourth = num_vec.get(3).unwrap_or(&-1);

    for i in &num_vec {
        println!("This is the num_vec element with value {}", i);
    }

    let mut string_vec: Vec<String> = Vec::new();

    string_vec.push(String::from("Hello"));
    string_vec.push(String::from("World"));

    for i in &mut string_vec {
        println!("Value of the mutable Vector String {}", i);
    }

    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let cells: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(10),
        SpreadSheetCell::Text(String::from("Test")),
        SpreadSheetCell::Float(0.24),
    ];

    for i in &cells {
        println!(
            "Let's see the value of the Cells type of SpreadSheetCell: {:#?}\n",
            i
        );
    }

    match cells.get(0) {
        Some(SpreadSheetCell::Int(value )) => println!("This is an integer value"),
        Some(_) => println!("This is some other value"),
        None => println!("Didn't match to any value")
    }

    println!("Index value of num_vec {} and {}", third, fourth);
}
