extern crate rustc_serialize;

use std::io::{self, Read};
use rustc_serialize::json::Json;

fn find_nums(data: &Json) -> i64 {
    match *data {
        Json::I64(i) => { return i; }
        Json::U64(u) => { return u as i64; }
        Json::F64(_) => { panic!(); }
        Json::Array(ref a) => {
            return a.iter().fold(0i64, |acc, v| acc + find_nums(&v));
        }
        Json::Object(ref o) => {
            let mut cnt = 0i64;
            for v in o.values() {
                if let Json::String(ref s) = *v {
                    if s == "red" {
                        return 0;
                    }
                }

                cnt += find_nums(&v);
            }

            return cnt;
        }
        Json::String(_) | Json::Boolean(_) | Json::Null => { return 0; }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let data = Json::from_str(&buffer).unwrap();
    let total = find_nums(&data);
    println!("total = {}", total);
}
