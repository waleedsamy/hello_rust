use hello::blog::Post;
fn main() {
    let mut post = Post::new();

    post.add_text("text text text");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("text text text", post.content());
}
