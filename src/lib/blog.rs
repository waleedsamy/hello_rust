pub struct Post {}
impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
}
pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn request_review(self) -> PendingReview {
        PendingReview {
            content: self.content,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

#[derive(Debug)]
pub struct PendingReview {
    content: String,
}
impl PendingReview {
    pub fn approve(self) -> PendingSecondReview {
        PendingSecondReview {
            content: self.content,
        }
    }
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

pub struct PendingSecondReview {
    content: String,
}
impl PendingSecondReview {
    pub fn approve(self) -> PublishedPost {
        PublishedPost {
            content: self.content,
        }
    }
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

#[derive(Debug)]
pub struct PublishedPost {
    content: String,
}
impl PublishedPost {
    pub fn content(&self) -> &str {
        &self.content
    }
}
