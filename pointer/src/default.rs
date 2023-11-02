pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Shows how to use a box to store an `i32` value on the heap
pub fn demo() {
    let b = Box::new(5);
    println!("b = {}", b);
}

pub struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_box() {
        let b = Box::new(5);
        assert_eq!(*b, 5);

        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_pointer() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_drop_custom_smart_pointer() {
        let _a = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _b = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }
}
