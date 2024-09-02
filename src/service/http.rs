use wasm_bindgen_futures::spawn_local;
use std::collections::HashMap;




#[derive(Clone, Debug, Default)]
pub struct HttpService;


#[derive(Clone, Debug, Default)]
pub struct Word{
    parts_of_speech: String,
    meaning: String,
}

impl HttpService {
    pub async fn new(wordlist:&str) -> std::result::Result<HashMap<String, String>,reqwest::Error> {
        
        let client = reqwest::Client::builder().build()?;
        let url = format!("https://dict.youdao.com/result?word={}&lang=en",wordlist.to_string());
        
        let res = client
            .get(url)
            //.header("referer", "https://cn.bing.com/")
            .send()
            .await?
            .text()
            .await?;
        println!("{:#?}", res);
        Ok(HashMap::new())
    }    

    pub fn crawl(&self, word:&str) -> HashMap<String, String> {
        let mut result = HashMap::new();
        result.insert("word".to_string(), word.to_string());
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::service::http::HttpService;    

    #[tokio::test]
    async fn fetch_test() {
        let resp = HttpService::new("hello").await.unwrap();
        println!("{:?}", resp);
    }
}