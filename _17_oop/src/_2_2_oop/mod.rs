mod blog;

use blog::Post;

pub fn main () {
    let mut post = Post::new();
    post.add_text("A");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!(post.content(), "A");
}
