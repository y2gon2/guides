#![allow(non_snake_case)]

use std::io::stdin;
use crate::data_control_models::in_memory_structs::{AddressBook, Person};

pub fn inputName() -> String {
    println!("Please input your name.");
    let mut nameBuffer = String::new();
    let stdinName = stdin().read_line(&mut nameBuffer).expect("input error");
    nameBuffer[..nameBuffer.len() - 2].to_string()
}

pub fn inputAddress() -> String {
    println!("Please input an address.");
    let mut addrBuffer = String::new();
    let stdinAddr = stdin().read_line(&mut addrBuffer).expect("input error");
    addrBuffer[..addrBuffer.len() - 2].to_string()
}

pub fn inputNumber() -> u8 {
    let inputError: u8 = 99;

    let mut numBuffer = String::new();
    let stdinNum = stdin().read_line(&mut numBuffer).expect("input error");
    let orderNum = numBuffer[..numBuffer.len() - 2].parse::<u8>();

    match orderNum {
        Ok(orderNum) => return orderNum,
        Err(error) => return inputError,
    }
}

pub fn showMenu() {
    println!("-----------------------------------------------------------------"); 
    println!("Please input the order number.");
    println!(" 0 : return to main menu without update.");
    println!(" 1 : confirm to save this change and return to main menu.");
    println!(" 2 : confirm to save this change and update inpormation more.");
    println!(" 3 : something wrong about the information. Input again without saving.");
    println!("-----------------------------------------------------------------"); 
}

pub fn addPerson(addressBook : &mut AddressBook) {
    println!("1. Add a new personal infomation.");

    let mut dataInputAgain = true;
    while dataInputAgain {
        let name = inputName();
        let address = inputAddress();    
        if addressBook.is_included(&name, &address) {
            println!("The same name and address information is already in the AddressBook");
            println!("Please input the 0 if you want to back to main menu or input any others.");

            let order = inputNumber();
            match inputNumber() {
                0 => dataInputAgain = false,
                _ => {},
            }

        } else {        
            let mut orderInputAgain = true;
            while orderInputAgain {
                println!("Prease check the input information.");

                // 만약 아래 variable 선언이 while 문 앞에 있다면, 에러 발생
                // (아마도) 내부 while 문이 반복되는 상항에서, 기존의 newPerson ownership 이 move 해버린 상태에서
                // 다시 선언하지 않고 돌아기기 때문(인듯)
                // 추가로 while 문 내부로 넣었지만, name, address 를 그냥 가져오게되면, 
                // name, address ownership 이 반복문 실행 시, 1회 사용으로 버려지기 때문에 clone 사용 (해야 하는듯)
                let newPerson = Person::new(name.clone(), address.clone()); // 

                newPerson.showPerson();
                showMenu();
                match inputNumber() {
                    0 => {
                        dataInputAgain = false;
                        orderInputAgain = false;
                        },
                    1 => {
                        addressBook.pushPerson(newPerson);
                        dataInputAgain = false;
                        orderInputAgain = false;
                    },
                    2 => {
                        addressBook.pushPerson(newPerson);
                        orderInputAgain = false;
                    },
                    3 => orderInputAgain = false,
                    _ => println!("Please enter one vaild number (0 - 3)."),
                }                    
            }
        }    
    }
}


pub fn searchPerson(addressBook : &AddressBook) {
    println!("2. Search a personal information in the data.");
    let mut dataInputAgain = true;
    while dataInputAgain {
        println!("Please select the searching keyword.");
        println!("0: return to main menu, 1: Name,  2: Address");
        let order = inputNumber();

        match order {
            0 => dataInputAgain = false,
            1 => {
                let findPerson = inputName();
                addressBook.findName(findPerson)
            },
            2 => {
                let findPerson = inputAddress();
                addressBook.findAddress(findPerson)
            },
            _ => println!("Please enter one vaild number (0 - 2).")
        }
    }
}

pub fn removePerson(addressBook: &mut AddressBook) {
    println!("4. Remove one memorized personal information.");
    let mut dataInputAgain = true;
    while dataInputAgain {
        let name = inputName();
        let address = inputAddress();

        if !addressBook.is_included(&name, &address) {
            println!("This is not in the AddressBook.");
            println!("Please input the 0 if you want to back to main menu or again input any others.");

            match inputNumber() {
                0 => dataInputAgain = false,
                _ => {},
            }
        } else {
            let rmPerson = Person::new(name, address); 
            
            let mut orderInputAgain = true;
            while orderInputAgain {
                rmPerson.showPerson();
                showMenu();

                match inputNumber() {
                    0 => {
                        dataInputAgain = false;
                        orderInputAgain = false;
                    },
                    1 => {
                        addressBook.delPerson(&rmPerson);
                        dataInputAgain = false;
                        orderInputAgain = false;
                    },
                    2 => {
                        addressBook.delPerson(&rmPerson);
                        orderInputAgain = false;
                    },
                    3 => orderInputAgain = false,
                    _ => println!("Please enter one vaild number (0 - 3)."),
                }
            }
        }
    }
}

pub fn changeAddress (addressBook: &mut AddressBook) {
    println!("5. Change one memorized personal information(address).");

    let mut dataInputAgain = true;
    while dataInputAgain {
        let name = inputName();
        let oldAddress = inputAddress();

        if !addressBook.is_included(&name, &oldAddress) {
            println!("This is not in the AddressBook.");
            println!("Please input the 0 if you want to back to main menu or again input any others.");

            match inputNumber() {
                0 => dataInputAgain = false,
                _ => {},
            }
        } else {
            println!("Please input new address to save.");
            let newAddress = inputAddress();

            let mut orderInputAgain = true;
            while orderInputAgain {
                showMenu();
                match inputNumber() {
                    0 => {
                        dataInputAgain = false;
                        orderInputAgain = false;
                    },
                    1 => {
                        addressBook.updateAddress(name.clone(), oldAddress.clone(), newAddress.clone());
                        dataInputAgain = false;
                        orderInputAgain = false;
                    },
                    2 => {
                        addressBook.updateAddress(name.clone(), oldAddress.clone(), newAddress.clone());
                        orderInputAgain = false;
                    },
                    3 => orderInputAgain = false,
                    _ => println!("Please enter one vaild number (0 - 3)."),
                }
            } 
        }
    }
}
