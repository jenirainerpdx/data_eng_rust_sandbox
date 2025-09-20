use futures::future::join_all;
use reqwest::Response;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = [
        "https://en.wikipedia.org/wiki/Rust_(programming_language)",
        "https://en.wikipedia.org/wiki/Python_(programming_language)",
        "https://en.wikipedia.org/wiki/Java_(programming_language)",
    ];

    let client = reqwest::Client::new();
    let fetches = urls.iter().map(|url| {
        client
            .get(*url)
            .header(USER_AGENT, "MyAppBot/1.0 jenirain@binaryarchitextures.com")
            .send()
    });
    let responses = join_all(fetches).await;

    for response in responses {
        match response {
            Ok(resp) => handle_response(resp).await,
            Err(e) => eprintln!("Request failed: {}", e),
        }
    }
    Ok(())
}

async fn handle_response(response: Response) {
    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());

    let body = response.text().await.unwrap();
    let document = Html::parse_document(&body);
    let selector = Selector::parse("h2, h3, p").unwrap();
    for element in document.select(&selector) {
        println!("{}", element.text().collect::<Vec<_>>().join(" "));
    }
}
