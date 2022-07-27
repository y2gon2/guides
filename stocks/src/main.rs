/// Vector 대신 HashMap 썼더니 정렬 개판...;;;

use stocks::handlers;
use stocks::structs;
use stocks::structs::Print; // trait 사용

fn main() {
    let path = "stocks/static/stocks.txt";
    let mut all_stocks = structs::AllStocks::new();
    handlers::load_file(&mut all_stocks, path);

    loop{
        println!("
            ----------------------------
             Plese input the number.
             1 => Add a new stock
             2 => Delete a stock.
             3 => Show me stock infomation.
             4 => Show me the highest value of each company.
             5 => Update date & price.
             6 => Company list on the system
             7 => Save
             0 => Quit
            -----------------------------"
        );
        match handlers::input_order() {
            1 => handlers::add_stock(&mut all_stocks),
            2 => handlers::del_stock(&mut all_stocks),
            3 => handlers::show_stock(&all_stocks),
            4 => handlers::highest(all_stocks.clone()),  // 왜 clone 해야 할까? clone 을 하지 않을 수 있는 방법은?
            5 => handlers::update_stock(&mut all_stocks),
            6 => all_stocks.info(),                      // trait 을 이용한 list 출력 method 사용
            7 => handlers::save_file(&all_stocks, path),
            0 => {
                println!("bye!!");
                break;
            }
            _ => println!("Please input a vaild number. (0~6)"),
        }

    }
}
