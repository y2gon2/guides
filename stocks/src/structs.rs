#![allow(unused_imports)]

use std::collections::HashMap;
use chrono::{Date, TimeZone, Local};
use crate::handlers::input_date;

// new 를 제외한 모든 method 들은 기존 HashMap 함수를 실행을 단순히 연결해주고 있다. 
// 이걸 좀더 (구조적으로) 간단하게 또는 좀더 범용적으로 만들 수 없을까?

#[derive(Debug, Clone)]
pub struct PriceMap {
    pub daily_prices: HashMap<Date<Local>, f32>,
}

impl PriceMap {
    pub fn new() -> Self {
        Self { daily_prices: HashMap::new() }
    }

    pub fn insert(&mut self, date: Date<Local>, price: f32) {
        self.daily_prices.insert(date, price);
    }

    pub fn remove(&mut self, date: &Date<Local>) {
        self.daily_prices.remove(date);
    }
}

#[derive(Clone)]
pub struct AllStocks{
    pub data: HashMap<String, PriceMap>, // key : 종목, value : PriceMap
} 

impl AllStocks{
    pub fn new() -> Self{
        Self { data: HashMap::new() }
    }

    pub fn insert(&mut self, company: String, pricemap: PriceMap) {
        self.data.insert(company, pricemap);        
    }

    pub fn remove(&mut self, company: &String) -> Option<PriceMap> {
        return self.data.remove(company)
    }

    pub fn get(&self, company: &String) -> Option<&PriceMap> {
        return self.data.get(company);
    }
  
    pub fn get_mut(&mut self, company: &String) -> Option<&mut PriceMap> {
        return self.data.get_mut(company)
    }
}




// 볼필요 없음-----------------------------------------------
// 번외.. 잘모르겠는 기능들 test 
#[test]
fn ex() {
    let now1 = Local.ymd(1900, 2, 8);
    let now2 = Local.ymd(2000, 12, 12);
    println!("{:?}", &now1 );
    // let (is_common_era, year) = now.year_ce();
    println!("{:?}", &now1.max(now2) );

    // https://morioh.com/p/5e37d90da1f9

    #[derive(Debug)]
    pub struct A {
        b: HashMap<String, String>, 
    }
    impl A{
        fn new() -> Self{
            Self{b: HashMap::new()}
        }
    }
    let mut a = A::new();
    a.b.insert("1".to_string(), "10".to_string());
    a.b.insert("2".to_string(), "20".to_string());
    a.b.insert("3".to_string(), "30".to_string());
    println!("{:?}", &a);

    a.b.remove("1");
    println!("{:?}", &a);

    let q = "2".to_string();

    println!("{}", a.b.get(&q).unwrap());

}


// regex
#[test]
fn reg() {
    use regex::Regex;

    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2014-07-05+sfsdf";
    for cap in re.captures_iter(text) {
        println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
    }

}

// HashMap - Clone & Copy
#[test]
fn clone_copy() {
    let mut a = HashMap::new();
    a.insert(1, "a");
    let mut b = a.clone();

    a.insert(2, "b");
    b.insert(3, "c");
    b.remove(&1);
    println!("{:?}", a);
    println!("{:?}", b);
}

