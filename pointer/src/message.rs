use std::cell::RefCell;

pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messager,
{
    pub fn new(messager: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

pub struct MockMessager {
    send_message: RefCell<Vec<String>>,
}

impl MockMessager {
    pub fn new() -> MockMessager {
        MockMessager {
            send_message: RefCell::new(vec![]),
        }
    }
}

impl Messager for MockMessager {
    fn send(&self, msg: &str) {
        self.send_message.borrow_mut().push(String::from(msg));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messager.send_message.borrow().len(), 1);
    }
}
