

mod golang_generator;
mod constant;
mod python_generator;
mod nodejs_generator;
mod curl_handler;

use async_std::task;


fn main() {
    let _input1 = r#"curl --location 'https://dog.ceo/api/breeds/list/all'"#;
    let mut ch = curl_handler::CurlHandler::new();
    task::block_on(ch.update_curl(_input1, false));
    println!("{}", ch.get_status_code());
    println!("{}", ch.get_parsing_err());
    println!("{}", ch.get_response());
    println!("{}", ch.get_golang());
}
