struct Post {
    content: String,
}

struct DraftPost {
    content: String,
}

struct PendingReviewPost {
    content: String,
}

impl Post {
    #[allow(clippy::new_ret_no_self)]
    fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
