mod blog;
use blog::Post;

pub fn main () {
    // publish상태에서만 content를 확인할 수 있음
    // review상태에서 approve가 2번이어야 publish가 됨
    // draft상태에서만 add_text를 사용할 수 있음
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_visible_only_in_publish() {
        let mut post = Post::new();

        post.add_text("A");
        assert_eq!("", post.content()); // draft

        post.request_review(); // review
        assert_eq!("", post.content());

        post.approve();
        post.approve();
        assert_eq!("A", post.content()); // published
    }

    #[test]
    fn need_two_successive_approvals_from_review() {
        let mut post = Post::new(); // draft

        post.add_text("A");

        post.request_review(); // review(approve: 0)
        post.approve(); // review(approve: 1)
        assert_eq!("", post.content());

        post.reject(); // draft(approve: 0)

        post.request_review(); // review(approve: 0)
        post.approve(); // review(approve: 1)
        assert_eq!("", post.content());

        post.approve(); // publish(approve: 0)
        assert_eq!("A", post.content());
    }

    #[test]
    fn add_text_only_in_draft() {
        let mut post = Post::new();
        post.add_text("A"); //draft

        post.request_review(); 
        post.add_text("B"); //review

        post.reject();
        post.add_text("C"); //draft

        post.request_review(); 
        post.approve();
        post.approve();
        post.add_text("D"); //publish
        
        assert_eq!("AC", post.content());
    }
}