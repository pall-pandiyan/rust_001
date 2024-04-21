extern crate reqwest;
extern crate select;

use select::document::Document;

fn main() {
    scrap_url("https://news.ycombinator.com");
}

fn scrap_url(url: &str) {
    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(select::predicate::Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
}
