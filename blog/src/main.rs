fn main() {}

struct Draft {
    content: String,
}

impl Draft {
    fn new() -> Draft {
        Draft {
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn request_review(self) -> PendingReview {
        PendingReview {
            content: self.content,
        }
    }
}

struct PendingReview {
    content: String,
}

impl PendingReview {
    fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

struct Post {
    content: String,
}

impl Post {
    fn content(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publishment_flow() {
        let mut post = Draft::new();
        post.add_text("I ate a salad for lunch today.");

        let post = post.request_review();

        let post = post.approve();

        assert_eq!("I ate a salad for lunch today.", post.content());
    }
}
