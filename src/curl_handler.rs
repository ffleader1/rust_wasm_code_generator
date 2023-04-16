use reqwest;
use curl_parser;
use serde_json::json;
use serde_json::{Result, Value};
use crate::golang_generator::GolangGenerator;
use crate::python_generator::PythonGenerator;
use crate::nodejs_generator::NodeJSGenerator;
use crate::rust_generator::RustGenerator;
use crate::constant::*;
use reqwest::StatusCode;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CurlHandler {
    input_curl: String,
    parsing_err: String,
    response: String,
    status: u32,
    golang_code: String,
    python_code: String,
    nodejs_code: String,
    rust_code: String,
}
/*
    status code:
    20: good
    1000 + HTML response code: in case the response code is not 2000
    30: parse input curl failed

 */
#[wasm_bindgen]
impl CurlHandler {
    pub fn new() -> CurlHandler{
        CurlHandler{
            input_curl: "".to_string(),
            parsing_err: "".to_string(),
            status: 0,
            response: "".to_string(),
            golang_code: "".to_string(),
            python_code: "".to_string(),
            nodejs_code: "".to_string(),
            rust_code: "".to_string(),
        }
    }
    pub async fn update_curl(&mut self, curl: &str, ignore_request: bool) {
        self.input_curl = curl.to_string().clone();
        match  curl_parser::ParsedRequest::load(curl, Some(json!({}))) {
            Ok(parsed) =>{
                self.parsing_err = "".to_string();
                let reqb: reqwest::RequestBuilder = parsed.into();
                let req = reqb.build().unwrap();
                let go_gen = GolangGenerator::new_from_reqwest(&req);
                let python_gen = PythonGenerator::new_from_reqwest(&req);
                let nodejs_gen = NodeJSGenerator::new_from_reqwest(&req);
                let rust_gen = RustGenerator::new_from_reqwest(&req);
                self.golang_code = go_gen.content;
                self.python_code = python_gen.content;
                self.nodejs_code = nodejs_gen.content;
                self.rust_code = rust_gen.content;
                if ignore_request {
                    return;
                }
                let client = reqwest::Client::new();
                let res =client.execute(req).await;
                match res {
                    Ok(r) => {

                        if r.status() != StatusCode::OK {
                            let status = r.status().as_u16() as u32;
                            self.status = HANDLER_STATUS_REQUEST_NOT_OK + status
                        }else{
                            self.status = HANDLER_STATUS_OK;
                        }

                        let r_get_text =r.text().await;
                        match r_get_text {
                            Ok(text) => {
                                let rv: Result<Value> = serde_json::from_str(&*text);
                                match rv{
                                    Ok(v) => {
                                        let pretty_str = serde_json::to_string_pretty(&v);
                                        match pretty_str {
                                            Ok(str) => {
                                                let formatted_text = format!("{}", str);
                                                self.response = formatted_text;
                                            }
                                            Err(_) => {
                                                self.response = text;
                                            }
                                        }

                                    }
                                    Err(_) => {
                                        self.response = text;
                                    }
                                }

                            }
                            Err(err_get_text) =>{
                                self.status = HANDLER_STATUS_ERROR_UNCLASSIFIED;
                                self.parsing_err = format!("{}", err_get_text);
                            }
                        }
                    }
                    Err(err_res) => {
                        self.status = HANDLER_STATUS_ERROR_UNCLASSIFIED;
                        self.parsing_err = format!("{}", err_res);}
                }
            }
            Err(e) => {
                self.status =  HANDLER_STATUS_PARSE_FAILED;
                self.parsing_err = format!("{}", e);
                // println!("{}", self.parsing_err);
                self.input_curl = "".to_string();
                self.response = "".to_string();
                self.golang_code = "".to_string();
                self.python_code = "".to_string();
                self.nodejs_code = "".to_string();
                self.rust_code = "".to_string();
            }
        };

    }
    pub fn get_status_code(&self) -> u32 {
        self.status.clone()
    }
    pub fn get_parsing_err(&self) -> String{
        self.parsing_err.clone()
    }
    pub fn get_response(&self) -> String {
        self.response.clone()
    }
    pub fn get_input_curl(&self) -> String{
        self.input_curl.clone()
    }
    pub fn get_golang(&self) -> String {
        self.golang_code.clone()
    }
    pub fn get_python(&self) -> String {
        self.python_code.clone()
    }
    pub fn get_nodejs(&self) -> String {
        self.nodejs_code.clone()
    }
    pub fn get_rust(&self) -> String {self.rust_code.clone()}
}