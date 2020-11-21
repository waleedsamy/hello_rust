pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    reviewer: u8,
}
impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            reviewer: 0,
        }
    }

    pub fn add_text(&mut self, text: &str) -> Result<(), &'static str> {
        if self.state.as_ref().unwrap().editable() {
            self.content.push_str(text);
            Ok(())
        } else {
            Err("post is not in Draft state")
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
            self.reviewer += 1;
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self).unwrap_or("")
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _: &'a Post) -> Option<&'a str> {
        None
    }
    fn editable(&self) -> bool {
        false
    }
}
#[derive(Debug)]
struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn editable(&self) -> bool {
        true
    }
}

#[derive(Debug)]
struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

#[derive(Debug)]
struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> Option<&'a str> {
        if post.reviewer >= 2 {
            Some(post.content.as_str())
        } else {
            None
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}
