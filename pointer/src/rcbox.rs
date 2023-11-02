use std::ops::Deref;

/// MyBox<T>
pub struct RcBox<T>(T);

impl<T> RcBox<T> {
    pub fn new(x: T) -> RcBox<T> {
        RcBox(x)
    }
}

impl<T> Deref for RcBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_box() {
        let x = 5;
        let y = RcBox::new(5);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
