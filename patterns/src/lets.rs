#[cfg(test)]
mod tests {
    #[test]
    fn test_if_let() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    #[test]
    fn test_while_let() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        // while pattern
        while let Some(top) = stack.pop() {
            println!("{top}");
        }
    }

    #[test]
    fn test_for() {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
}
