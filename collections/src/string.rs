pub fn demo() {
    println!("Here is string demo!");

    let mut _s = String::new();

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    let mut s1 = String::from("foot");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s}");

    let hello = "Здравствуйте";

    let s = &hello[0..6];
    println!("hello[0..4] is {s}");
}

pub fn chars() {
    let hello = "Здравствуйте";

    let s = &hello[0..6];
    println!("hello[0..4] is {s}");

    for c in hello.chars() {
        println!("{c}");
    }
}

pub fn types() {
    let hello = "Здравствуйте";

    for b in hello.bytes() {
        println!("{b}")
    }
}
