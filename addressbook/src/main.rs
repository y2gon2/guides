// 1. 주소록 작성하기.
// 1.1 이름, 주소를 txt 파일에 입력, 출력, 내용 추가, 삭제, 찾기 기능을 구현

#![allow(non_snake_case)]

use std::fs::{self, OpenOptions};
use std::io::{stdin, Write};
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct Person {
    name: RefCell<&'static str>,
    address: RefCell<&'static str>,
}

impl Person {
    pub fn new(inputName: &'static str, inputAddress: &'static str) -> Self {
        Person { 
            name: RefCell::new(inputName), 
            address: RefCell::new(inputAddress),
        }
    }
}

#[derive(Debug)]
pub struct AddressBook {
    addressList: Vec<Person>,
}

impl AddressBook {
    pub fn new() -> Self {
        AddressBook { addressList: Vec::<Person>::new() }
    }

    pub fn openBook(&mut self, fileName: &str) {
        let fileData = fs::read_to_string(fileName)
            .expect("Searching and reading the file error");
        let mut data = fileData.as_str();

        while data.chars().count() > 0 {
            let nameSlice = data.find('\t').unwrap();
            let name = &data[..nameSlice];
            
            let addSlice = data.find('\n').unwrap();
            let address = &data[nameSlice + 1..addSlice];

            &mut self.addressList.push(Person::new(name, address));

            data = &data[addSlice + 1 ..];
        }
    } 

    pub fn saveBook(&self, fileName: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(fileName)
            .expect("saving the data error");
        
        for data in &self.addressList {
            let writing = format!("{}\t{}\n", data.name.borrow(), data.address.borrow());
            file.write(writing.as_bytes()).expect("writing error");
        }

        println!("The file is updated!");
    }

    pub fn showAllPerson(&self) {
        for person in &self.addressList {
            println!("{} : {}", person.name.borrow(), person.address.borrow());
        }
    }

    pub fn addPerson(&mut self, person: Person) {
        &mut self.addressList.push(person);
    }

    pub fn delPerson(&mut self, person: &Person) {
        &mut self.addressList.retain(|x| x != person);
        println!("[{} : {}] is deleted", person.name.borrow(), person.address.borrow());
    }

    pub fn updateAddress(&mut self, personFrom: &Person, personTo: &Person) {

    }

    pub fn findPerson(&self, findPerson: &str) {
        let mut exitence: u8 = 0;
        for person in &self.addressList {
            if person.name == RefCell::new(findPerson) {
                println!(
                    "[name: {}, address: {}] is in the addressbook", 
                    &person.name.borrow(), 
                    &person.address.borrow()
                );
            } 
            exitence += 1;
        }
        if exitence == 0 {
            println!("{} is not in the addressbook", findPerson);
        } else {
            println!("Find : {}", &exitence);
        }
    }


}

pub fn inputPerson() -> Person {
    println!("Please input your name.");
    let mut nameBuffer = String::new();
    let stdinName = stdin().read_line(&mut nameBuffer).expect("input error");
    
    println!("Please input an address.");
    let mut addBuffer = String::new();
    let stdinAdd = stdin().read_line(&mut addBuffer).expect("input error");
    
    let inputName = nameBuffer[..nameBuffer.len() - 2].to_string();
    let inputAddress = addBuffer[..addBuffer.len() - 2].to_string();
    
    Person::new(inputName, inputAddress)
}


fn main() {
    println!("Hello, world!");
}
