use reqwest;
use crate::constant;
use std::str;


pub struct PythonGenerator {
    pub content: String
}

impl PythonGenerator {
    pub fn new_from_reqwest(req: &reqwest::Request) -> PythonGenerator{
        let url = req.url().as_str();
        let method = req.method().as_str();
        let headers = req.headers();
        let mut header_str = "{\n".to_string();
        let key_len = headers.keys().len();
        let mut current_count = 1;
        for (key, value) in &*headers {
            let line = format!("  \"{}\":\"{}\"{}\n", key.as_str(), value.to_str().unwrap(),if current_count >= key_len { " " } else { current_count+=1 ; ", "});
            header_str += &*line
        }
        header_str += "  }\n";

        let body = match  req.body() {
            None => {"{}"}
            Some(body_u8_str) => {
                match body_u8_str.as_bytes() {
                    None => {"{}"}
                    Some(body_u8) => {
                        match str::from_utf8(body_u8) {
                            Ok(v) => v,
                            Err(_e) => "{}",
                        }
                    }
                }
            }
        };
        let mut content = PYTHON_TEMPLATE.replace(constant::PLACEHOLDER_URL, url);
        content = content.replace(constant::PLACEHOLDER_METHOD, method);
        content = content.replace(constant::PLACEHOLDER_HEADER, &*header_str);
        content = content.replace(constant::PLACEHOLDER_BODY, body);
        PythonGenerator{content}
    }
}



static PYTHON_TEMPLATE: &str = r#"import requests
import json

url = "*URL_STRING*"

payload = json.dumps(*BODY_STRING*)
headers = *HEADERS_STRING*

response = requests.request("*METHOD_STRING*", url, headers=headers, data=payload)

print(response.text)
"#;