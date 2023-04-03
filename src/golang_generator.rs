use reqwest;
use crate::constant;
use std::str;
use reqwest::{Method};


pub struct GolangGenerator {
    pub content: String
}

impl GolangGenerator {
    pub fn new_from_reqwest(req: &reqwest::Request) -> GolangGenerator{
        let url = req.url().as_str();
        let method = req.method().as_str();
        let headers = req.headers();
        let mut header_str = "".to_string();
        for (key, value) in &*headers {
            let line = format!("  req.Header.Add(\"{}\",\"{}\")\n", key.as_str(), value.to_str().unwrap());
            header_str += &*line
        }
        let body = match req.method()  {
            &Method::GET => {
                GOLANG_TEMPLATE_GET.to_string()
            }
            &_ => {
                GOLANG_TEMPLATE_POST.replace(constant::PLACEHOLDER_BODY, match  req.body() {
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
                })
            }
        };

        let mut content = GOLANG_TEMPLATE.replace(constant::PLACEHOLDER_URL, url);
        content = content.replace(constant::PLACEHOLDER_METHOD, method);
        content = content.replace(constant::PLACEHOLDER_HEADER, &*header_str);
        content = content.replace(constant::PLACEHOLDER_BODY, &body);
        GolangGenerator{content}
    }
}

static GOLANG_TEMPLATE_POST: &str = r#"
  payload := strings.NewReader(`*BODY_STRING*`)

  req, err := http.NewRequest(method, url, payload)
  "#;

static GOLANG_TEMPLATE_GET: &str = r#"
  req, err := http.NewRequest(method, url, nil)
  "#;

static GOLANG_TEMPLATE: &str = r#"
package main

import (
  "fmt"
  "strings"
  "net/http"
  "io/ioutil"
)

func main() {

  url := "*URL_STRING*"
  method := "*METHOD_STRING*"

  client := &http.Client {
  }

  *BODY_STRING*

  if err != nil {
    fmt.Println(err)
    return
  }

*HEADERS_STRING*

  res, err := client.Do(req)
  if err != nil {
    fmt.Println(err)
    return
  }
  defer res.Body.Close()

  body, err := ioutil.ReadAll(res.Body)
  if err != nil {
    fmt.Println(err)
    return
  }
  fmt.Println(string(body))
}"#;