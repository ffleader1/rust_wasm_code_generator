use reqwest;
use crate::constant;
use std::str;


pub struct RustGenerator {
    pub content: String,
}

impl RustGenerator {
    pub fn new_from_reqwest(req: &reqwest::Request) -> RustGenerator {
        let url = req.url().as_str();
        let mut content = RUST_TEMPLATE.replace(constant::PLACEHOLDER_URL, url);
        let method = req.method().as_str().to_lowercase();
        content = content.replace(constant::PLACEHOLDER_METHOD, &method);
        let headers = req.headers();
        let header_section = if headers.is_empty() {
            "".to_string()
        } else {
            let mut all_headers = "".to_string();
            for (key, value) in &*headers {
                let line = format!("\n\t\t\theaders.insert(\"{}\", \"{}\".parse().unwrap());", key.as_str(), value.to_str().unwrap());
                all_headers += &*line
            }
            RUST_TEMPLATE_HEADER.replace(constant::PLACEHOLDER_HEADER, &*all_headers)
        };
        content = content.replace(constant::PLACEHOLDER_HEADER,&header_section);
        let rust_body = match  req.body() {
            None => {"".to_string()}
            Some(body_u8_str) => {
                match body_u8_str.as_bytes() {
                    None => {"".to_string()}
                    Some(body_u8) => {
                        match str::from_utf8(body_u8) {
                            Ok(v) => {
                                format!(".body(r##\"{}\"##)", v)
                            },
                            Err(_e) => "".to_string(),
                        }
                    }
                }
            }
        };
        content = content.replace(constant::PLACEHOLDER_BODY, &rust_body);

        RustGenerator{content}
    }
}

static RUST_TEMPLATE_HEADER: &str = r#".headers({
            let mut headers = http::header::HeaderMap::new();*HEADERS_STRING*
            headers})"#;

static RUST_TEMPLATE: &str = r#"use reqwest;

#[async_std::main]
async fn main()  {

    let client = reqwest::Client::new();
    let res = client.*METHOD_STRING*("*URL_STRING*")
        *HEADERS_STRING*
        *BODY_STRING*
        .send()
        .await.unwrap();
    println!("{}", res.text().await.unwrap())
}

"#;