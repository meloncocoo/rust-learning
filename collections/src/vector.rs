pub fn demo() {
    let mut _v: Vec<i32> = Vec::new();

    _v.push(5);
    _v.push(6);
    _v.push(7);
    _v.push(8);

    println!("The vector is {:?}", _v);

    let mut v = vec![1, 2, 3, 4, 5];

    println!("The vector is {:?}", v);

    let third: &i32 = &v[2];
    println!("THe third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }

    //

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for v in &row {
        println!("{:?}", v);
    }
}
