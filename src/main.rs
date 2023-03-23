use reqwest::Client;
use scraper::{Html,Selector};
#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{
    let client = Client::new();

    let res = client.get("http://books.toscrape.com/")
                                                .send().await?;
    let body = res.text().await?;

    let document = Html::parse_document(&body);

    let book_title_selector = Selector::parse("h3 > a").unwrap();

    for book_tilte in document.select(&book_title_selector){
        let title = book_tilte.text().collect::<Vec<_>>();
        println!("title: {}",title[0]);
    }

    let book_price_selector = Selector::parse(".price_color").unwrap();
    for book_price in document.select(&book_price_selector){
        let price = book_price.text().collect::<Vec<_>>();
        println!("price: {}",price[0]);
    }
    
    Ok(())

}
// test with c ffi
// use libc::{c_int,c_void,size_t};
// #[link(name = "yourlb")]
// extern {
//     fn your_func(arg1:c_int,arg2:*mut c_void)->size_t;
    
// }