use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread : {:?}", list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.height());
    println!("{:#?}", list);
}

#[derive(Debug)]
pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}
