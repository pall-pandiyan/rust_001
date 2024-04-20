extern crate reqwest;
extern crate select;

fn main() {
    scrap_url("https://news.ycombinator.com");
}

async fn scrap_url(url: &str) {
    let mut response = reqwest::get(url).await.unwrap();
    assert!(response.status().is_success());

    select::document::Document::from_read(response)
        .unwrap()
        .find(select::predicate::Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
}
