pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0f64,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut c = AveragedCollection::new();

        c.add(100);
        assert_eq!(c.average(), 100f64);
        c.add(3000);
        assert_eq!(c.average(), 1550f64);
        c.add(50);
        assert_eq!(c.average(), 1050f64);
        c.add(500);
        assert_eq!(c.average(), 912.5);
        c.remove();
        assert_eq!(c.average(), 1050f64);
    }
}
