
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approve_count: u32,
}
impl Post {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            content: "".to_owned(),
            approve_count: 0
        }
    }
    pub fn add_text(&mut self, text: &str){
        if self.state.as_ref().unwrap().is_draft() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject(&mut self.approve_count))
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve(&mut self.approve_count))
        }
    }
}

trait State {
    fn is_draft(&self) -> bool { false }
    fn is_review(&self) -> bool { false }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>, approve_count: &mut u32) -> Box<dyn State>;
    fn reject(self: Box<Self>, approve_count: &mut u32) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}


struct Draft;
impl State for Draft {
    fn is_draft(&self) -> bool { true }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview)
    }
    fn approve(self: Box<Self>, approve_count: &mut u32) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>, approve_count: &mut u32) -> Box<dyn State> {
        self
    }
}

struct PendingReview;
impl State for PendingReview {
    fn is_review(&self) -> bool { true }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>, approve_count: &mut u32) -> Box<dyn State> {
        if *approve_count >= 1 {
            *approve_count = 0;
            Box::new(Published)
        } else {
            *approve_count += 1;
            self
        }
    }
    fn reject(self: Box<Self>, approve_count: &mut u32) -> Box<dyn State> {
        *approve_count = 0;
        Box::new(Draft)
    }
}

struct Published;
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>, approve_count: &mut u32) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>, approve_count: &mut u32) -> Box<dyn State> {
        self
    }
}

