// trait State {
//     // self: Box<Self> 意味着该方法只可在持有这个类型的 Box 上被调用
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;

//     fn content<'a>(&self, _: &'a Post) -> &'a str {
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

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
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

// struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }

// impl Post {
//     fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }

//     fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }

//     fn content(&self) -> &str {
//         // 注意 as_ref 生成了一个新的 Option
//         // 这里 self 代表 &Post
//         self.state.as_ref().unwrap().content(self)
//     }

//     fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review())
//         }
//     }

//     fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve())
//         }
//     }
// }

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
