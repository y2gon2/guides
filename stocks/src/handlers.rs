use std::io::{stdin, Write};
use std::fs::{self, OpenOptions};
use std::collections::HashMap;
use chrono::{Local, Date, TimeZone};
use regex::Regex;
use crate::structs::{PriceMap, AllStocks, Print};

// -- file load, save--
pub fn load_file(all_stocks: &mut AllStocks, path: &str) {
    let file_string = fs::read_to_string(path).expect("Error : failed to search and read the file.");
    //let file_str = include_str!("abc.txt");
    let file_data: Vec<&str> = file_string.split("*****\n").collect();
    dbg!(&file_data);
    
    for element in file_data {
        let mut data_iter = element.split_whitespace(); // Windows 기준
        let mut company = String::new();

        match data_iter.next(){
            Some(x) => company = x.to_owned(),
            None => break,
        }
        
        let mut prices = PriceMap::new(); 
        for _ in 0..5 {
            let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
            let date_str = data_iter.next().unwrap();

            // 년, 월, 일 각 변수에 유효한 기본값 대입 후, data 로부터 parsing
            let mut year: i32 = 1900;
            let mut month: u32 = 1;
            let mut day: u32 = 1;

            // regex 가 익숙하지 않아, 지저분....;; 
            for cap in re.captures_iter(date_str) {
                year = cap[1].parse::<i32>().expect("parsing error");
                month = cap[2].parse::<u32>().expect("parsing error");
                day = cap[3].parse::<u32>().expect("parsing error");                
            }
            let date = Local.ymd(year, month, day);

            let price = data_iter.next().unwrap();

            prices.insert(date, price.parse::<f32>().expect("parsing error"))
        }

        all_stocks.insert(company, prices)
    }

}

// HashMap 으로 하니까 저장순서가 바뀜;;;
pub fn save_file (all_stocks: &AllStocks, path: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .open(path)
        .expect("Error: Fail to access the file to save");

    for (company, prices) in all_stocks.data.iter() {
        let w_company = format!("{}\n", *company); 
        file.write(w_company.as_bytes()).expect("Error: Fail to write company name on the file");

        for (date, price) in prices.daily_prices.iter() {
            let w_daily_price = format!("{}\t{}\n", *date, *price);
            file.write(w_daily_price.as_bytes()).expect("Error: Fail to write daily price on the file");
        }
        // ***** 표시로 종목 구분
        file.write("*****\n".as_bytes()).expect("Error: Fail to write ending mark.");
    }

    println!("The file is saved!");
}

// -- input --
pub fn input_string() -> String {
    let mut string_buf = String::new();
    let _stdin = stdin().read_line(&mut string_buf).expect("input error");
    string_buf.trim().to_string()
}

pub fn input_order() -> u8{
    let err_num: u8 = 99;

    match input_string().trim().parse::<u8>() {
        Ok(order_num) => return order_num,
        Err(_error) => return err_num, 
    }
}

pub fn input_price() -> f32 {
    loop{
        match input_string().trim().parse::<f32>() {
            Ok(input_price) => return input_price,
            Err(_error) => println!("Please input valid price"),
        }
    }
}

pub fn input_date() -> Date<Local> {
    loop{
        let input_date = input_string().clone();
        let date_info: Vec<&str> = input_date.split("-").collect(); 

        let year = date_info[0].parse::<i32>();
        let month = date_info[1].parse::<u32>();
        let day = date_info[2].parse::<u32>();

        match year {
            Ok(year) => match month {
                Ok(month) => match day {
                    Ok(day) => {
                        if year > 1900 && month < 13 && day > 0 && day < 32 {
                            return Local.ymd(year, month, day);
                        } else {
                            println!("Please input valid numbers for year, month, day (ex. 2022-7-23)");
                        }
                    },
                    Err(_err) => println!("Please input valid number for day (ex. 2022-7-23)"),
                }
                Err(_err) => println!("Please input valid number for month (ex. 2022-7-23)"),
            } 
            Err(_err) => println!("Please input valid number for year (ex. 2022-7-23)"),
        }
    }
}

// -- for main menu control --
pub fn add_stock(all_stocks: &mut AllStocks) {
    let mut pricemap = PriceMap::new();

    println!("Please input a company to add on the system.");
    let company = input_string();
    println!("Please input 5days of dates and its price.");
    for i in 0..5 {
        println!("({}) Please input date.", &i + 1);
        let date = input_date();
        
        println!("({}) Please input it price.", &i + 1);
        let price = input_price();

        pricemap.insert(date, price);
    }

    all_stocks.insert(company, pricemap);
}

pub fn del_stock(all_stocks: &mut AllStocks) {
    println!("Please input a company to delete on the system.");
    let company = input_string();

    match all_stocks.remove(&company){
        None => println!("This is not exist on the system."),
        _ => println!("Deleted."),
    }
}

// HashMap 으로 하니까 출력순서가 지멋대로임;;;
pub fn show_stock(all_stocks: &AllStocks) {
    println!("Please input a company to know its infomation.");
    let company = input_string();

    match all_stocks.get(&company) {
        None => println!("This is not exist on the system."),
        Some(value) => value.info(),        // trait 을 이용한 list 출력 method 사용
    }
}

pub fn highest(all_stocks: AllStocks){
    for (company, price_map) in all_stocks.data {
        print!("{} : ", &company);

        let mut max_date = Local.ymd(1900, 1, 1);
        let mut max_price: f32 = 0.0;
        for (date, price) in price_map.daily_prices {
            if max_price <= price {     // 순서대로 저장되라는 보장은 없지만... 값이 같을 경우 최신 날짜꺼로??
                max_date = date;
                max_price = price;
            }
        }
        println!("{} : {}", max_price, max_date);
    }
}

pub fn update_stock(all_stocks: &mut AllStocks) {
    println!("Please input a company to update its infomation.");
    let company = input_string();

    match all_stocks.get_mut(&company) {
        None => println!("This is not exist on the system."),
        Some(x) => {
            // 가장 빠른 날짜의 price 제거.. vector 로 했으면 편했을듯..;;;
            let dates = x.daily_prices.keys();
            let mut min_date = Local.ymd(2100, 12,31);

            for date in dates {
                if *date < min_date {
                    min_date  = *date;
                }
            }

            (*x).remove(&min_date);

            // 새로운 date price 추가
            println!("Please input new date");
            let new_date = input_date();
            
            println!("Please input new price");
            let new_price = input_string().parse::<f32>().unwrap();

            (*x).insert(new_date, new_price);
        },
    }

}




// 볼필요 없음-----------------------------------------------
// 번외.. 잘모르겠는 기능들 test 
#[test]
fn test00() {
    let mut a = HashMap::new();
    a.insert(1,"a");
    a.insert(2, "b");
    let keys = a.keys();
    dbg!(&keys);
    print!("{:?}", a.remove(&1));
    println!("{:?}", a.remove(&1));
    
}