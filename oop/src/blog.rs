pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    // pub fn request_review(&mut self) {
    //     if let Some(s) = self.state.take() {
    //         self.state = Some(s.request_review())
    //     }
    // }

    // pub fn approve(&mut self) {
    //     if let Some(s) = self.state.take() {
    //         self.state = Some(s.approve())
    //     }
    // }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, _post: &'a Post) -> &'a str {
//         ""
//     }
// }

// struct Draft {}

// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }

// struct PendingReview {}

// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Post {
//         Post {
//             content: self.content,
//         }
//     }
// }

// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
