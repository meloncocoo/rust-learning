#[cfg(test)]
mod tests {
    #[test]
    fn test_match_var() {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x);
    }

    #[test]
    fn test_match_more() {
        let x = 2;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("there"),
            _ => println!("anything"),
        }
    }

    #[test]
    fn test_match_range() {
        let x = 5;

        match x {
            1..=5 => println!("one through five"), // eq: 1 | 2 | 3 | 4 | 5
            _ => println!("something else"),
        }
    }

    #[test]
    fn test_match_char_range() {
        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }
}
