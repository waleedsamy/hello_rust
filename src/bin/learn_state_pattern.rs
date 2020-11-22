use hello::blog::Post;
fn main() {
    let mut post = Post::new();
    post.add_text("text text text");
    let post = post.request_review();
    let post = post.approve();
    let post = post.approve();
    assert_eq!("text text text", post.content());
}
