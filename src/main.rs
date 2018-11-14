extern crate reqwest;

use std::fmt;
use std::collections::HashMap;
use reqwest::header::CONTENT_TYPE;

struct Bestmixer {
    api_key: String,
    api_url: String,
    proxy: String,
    use_proxy: bool,
    use_tor: bool
}

impl Bestmixer {
    fn new(api_key: &str) -> Self {
        Bestmixer {
            api_key: api_key.to_string(),
            use_proxy: false,
            use_tor: false,
            proxy: String::new(),
            api_url: "https://bestmixer.io/api/ext/".to_string()
        }
    }
    fn get_api_key(&self) -> &str{
        return &self.api_key;
    }
    fn request(&self, path: &str, data: HashMap<&str, &str>) -> String {
        let mut map = data.clone();
        map.insert("api_key", &self.api_key);
        let client = reqwest::Client::new();
        let url = format!("{}{}", self.api_url, path); 
        let mut response = client.post(&url).json(&map).send();
        return response.unwrap().text().unwrap();
    }
    fn fee_info(&self) -> String {
        let data = HashMap::new();
        self.request("fee/info", data)
    }
    fn order_info(&self, order_id: &str) -> String {
        let mut data = HashMap::new();
        data.insert("order_id", order_id);
        self.request("order/info", data)
    }
    fn code_info(&self, bm_code: &str) -> String {
        let mut data = HashMap::new();
        data.insert("bm_code", bm_code);
        self.request("code/info", data)
    }
    fn create_order(&self, coin: &str, fee: f32) -> Order {
        Order::new(&self.api_key, coin, fee)
    }
    fn submit_order(&self, order: Order) -> String {
        let client = reqwest::Client::new();
        let url = format!("{}order/create", self.api_url); 
        let order = format!("{}", order);
        let mut response = client.post(&url)
            .header(CONTENT_TYPE, "application/json")
            .body(order)
            .send();
        return response.unwrap().text().unwrap();        
    }
}

struct Output {
    address: String,
    percent: u32,
    delay: u32
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               r#"{{"address": "{}", "percent": {}, "delay": {}}}"#,
               self.address, self.percent, self.delay)
        }
}

struct Order {
    api_key: String,
    coin: String,
    fee: f32,
    outputs: Vec<Output>
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"{{"api_key": "{}", "coin": "{}", "fee": {}, "output": "#,
               self.api_key, self.coin, self.fee);
        write!(f, "[");
        for o in self.outputs.iter() {
            write!(f, "{}", o);
        }
        write!(f, "]");
        write!(f, "}}")
    }
}

impl Order {
    fn new(api_key: &str, coin: &str, fee: f32) -> Self {
        Order {
            api_key: api_key.to_string(),
            coin: coin.to_string(),
            fee: fee,
            outputs: Vec::new()
        }
    }
    fn add_output(&mut self, address: &str, percent: u32, delay: u32) {
        let output = Output {
            address: address.to_string(),
            percent: percent,
            delay: delay
        };
        self.outputs.push(output);
    }
}

fn main() {
    let api = Bestmixer::new("replace_with_api_key");
    println!("API key: {}", api.get_api_key());
    println!("Fee/Info\n{}", api.fee_info());
    println!("Order/Info\n{}", api.order_info("oid"));
    println!("Code/Info\n{}", api.code_info("bm_code"));
    let mut order = api.create_order("ltc", 0.5001);
    order.add_output("LYDp9NWddbzxNfmmNj2tEjdXyDbxRuvgX4", 100, 45);
//    order.add_output("address", 50, 105);
    println!("{}", order);
    println!("{}", api.submit_order(order));
}
