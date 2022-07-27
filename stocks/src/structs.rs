#![allow(unused_imports)]

use std::collections::HashMap;
use chrono::{Date, TimeZone, Local};
use crate::handlers::input_date;

// new 를 제외한 모든 method 들은 기존 HashMap 함수를 실행을 단순히 연결해주고 있다. 
// 이걸 좀더 (구조적으로) 간단하게 또는 좀더 범용적으로 만들 수 없을까?
// -> 해당 질문이 trait 을 쓰는것과 관련된 것이였는데... 
//    다시 생각해보니 나도 trait 에 대해 잘못 이해하고 있는것들이 확인되서.. 
//    간단하게 "trait 이 이렇게 생겼다"는 형태만 보여주고 
//    자세한 설명은 동훈이한테 듣도록.. (나도 동훈이 문제 풀이 해야 함. ;; ㅋㅋ)

// (기존 method 들을 적용하려니 좋은 예가 떠오르지않아서...)
// triat 을 이용해서 print 하는 새로운 method 를 정의해서 사용해 보면.. 
// (여기선는 그냥 이렇게 생겼구나 정도만 확인하고 실제 왜 쎠야 하는지는 동훈 사부에게 전수받도록)
// 65 번 줄부터 trait 내용
 
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

// -- trait 맛보기 --
pub trait Print {
    fn info(&self);
}

// trait 을 사용하여 특정 종목의 전체 data - price 를 보고 싶다면,
impl Print for PriceMap {
    fn info(&self) {
        for (date, price) in &self.daily_prices {
            print!("date : {:}, price: {:} \n", date, price);
        }
        println!("");
    }
}

// tait 을 사용하여 현재 
impl Print for AllStocks {
    fn info(&self) {
        for (company, _) in &self.data {
            print!("{}\t", company);
        }
    }
}

// -------------------




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

