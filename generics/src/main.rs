use generics::{point::Point, string::longest, NewsArticle, Summary, Tweet};

// use crate::point::Point;

pub mod point;

fn main() {
    println!("Hello, world!");

    let number_list: Vec<i32> = vec![34, 50, 32, 25, 100, 65];
    let result: &i32 = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list: Vec<i32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result: &i32 = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    let result: &char = largest(&char_list);
    println!("The largest char is {}", result);

    let point = Point::new(3.2 as f32, 5.5 as f32);
    let result = point.distance_from_origin();
    println!(
        "Distance of point({}, {}) is {}",
        point.x(),
        point.y(),
        result
    );

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn largest<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
