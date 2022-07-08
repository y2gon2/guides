/// 1. 주소록 작성하기.
/// 1.1 이름, 주소를 txt 파일에 입력, 출력, 내용 추가, 삭제, 찾기 기능을 구현
/// 실행 : cargo run --pacakage addressbook 
/// 필요 개선 사항
/// a. 모든 handler 함수는 addressBook 을 ref 또는 mut ref 로 매개변수를 받음. 
///    -> handler 에 추가 sub 함수가 있을 경우, 해당 매개변수 처리가 복잡해짐. 
///       sharded memory 를 통해 이를 해결할수 있을 듯 하지만, 아직 잘 모르겠음. 
/// b. data contol 의 가장 작은 단위는 person 의 필드로 모두 String 으로 받고 있지만, 
///    때문에 ownership 문제가 발생하거나 clone 을 남발해야 할 경우가 발생될 수 있음. 
///    -> RC, RefCell 등의 shared reference 를 통해 value 를 수정할 수 있는 스마트 포인터를 써서 
///       해결 가능할 것으로 보임.  

use addressbook::handlers;
use addressbook::data_control_models::in_memory_structs;

fn main() {
    #![allow(non_snake_case)]
    
    let fileName = "addressbook/src/statics/db/book.txt";
    let mut addressBook = in_memory_structs::AddressBook::new();
    addressBook.openBook(fileName);

    loop {
        println!("-----------------------------------------------------------------");
        println!("Please enter the number of the job you want.");
        println!("1. Add a new personal infomation.");
        println!("2. Search a personal information in the data.");
        println!("3. Check the all memorized personal information.");
        println!("4. Remove one memorized personal information.");
        println!("5. Change one memorized personal information(address).");
        println!("6. Save the update to a file.");
        println!("Please input '0' if you want to quit this program.");
        println!("-----------------------------------------------------------------");   
        println!("Please input order number. (0 - 6)");

        match handlers::inputNumber() {
            1 => handlers::addPerson(&mut addressBook),
            2 => handlers::searchPerson(&addressBook),
            3 => addressBook.showAllPerson(),
            4 => handlers::removePerson(&mut addressBook),
            5 => handlers::changeAddress(&mut addressBook),
            6 => addressBook.saveBook(fileName),
            0 => {
                println!("Bye!!");
                break;
            },
            _ => println!("Please input one vaild number (0 - 6)."),
        }
    }
}




