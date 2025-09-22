extern crate mdbook;

use mdbook::renderer::RenderContext;
use std::io;
use mdbook::book::Chapter;
use mdbook::BookItem;
fn main() {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    for item in ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            let num_words = count_words(ch);
            println!("{}: {}", ch.name, num_words);
        }
    }
}

fn count_words(ch: &Chapter) -> usize {
    ch.content.split_whitespace().count()
}
