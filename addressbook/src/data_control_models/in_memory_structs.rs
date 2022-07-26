#![allow(non_snake_case)]

use std::fs::{self, OpenOptions};
use std::io::Write;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Person {
    name: String,
    address: String,
}

impl Person {
    pub fn new(inputName: String, inputAddress: String) -> Person {
        Self { 
            name: inputName, 
            address: inputAddress,
        }
    }

    pub fn showPerson(&self) {
        println!("name : {},  address : {}]", &self.name, &self.address);
    }
}


#[derive(Debug)]
pub struct AddressBook {
    db: Vec<Person>
}

impl AddressBook {
    pub fn new() -> Self {
        AddressBook { db: Vec::<Person>::new() }
    }

    pub fn openBook (&mut self, fileName: &str) {
        let mut fileData: String = fs::read_to_string(fileName)
            .expect("Searching and reading the file error");
    
        while fileData.chars().count() > 0 {
            let nameSlice = fileData.find('\t').unwrap();
            let name = fileData[..nameSlice].to_string();
            
            let addSlice = fileData.find('\n').unwrap();
            let address = fileData[nameSlice + 1..addSlice].to_string();

            &mut self.db.push(Person::new(name, address));

            fileData = fileData[addSlice + 1 ..].to_string();
        }
    } 

    pub fn saveBook(&self, fileName: &str) {
        println!("6. Save the update to a file.");
        let mut file = OpenOptions::new()
            .write(true)
            .open(fileName)
            .expect("saving the data error");
        
        for data in &self.db {
            let writing = format!("{}\t{}\n", data.name, data.address);
            file.write(writing.as_bytes()).expect("writing error");
        }
        println!("The file is updated!");
    }

    pub fn showAllPerson(&self) {
        println!("3. Check the all memorized personal information.");
        for person in &self.db {
            println!("{} : {}", &person.name, &person.address);
        }
    }

    pub fn pushPerson(&mut self, person: Person) {
        &mut self.db.push(person.clone());
        println!("[{} : {}] is added", &person.name, &person.address);
    }

    pub fn delPerson(&mut self, person: &Person) {
        &mut self.db.retain(|x| x != person);
        println!("[{} : {}] is deleted", person.name, person.address);
    }

    pub fn updateAddress(&mut self, name: String, oldAddress: String, newAddress: String) {
        for person in &mut self.db {
            if person.name == name && person.address == oldAddress {
                person.address = newAddress;
                println!("[{} : {}] is updated", person.name, person.address);
                break;
            }
        }
    }

    pub fn findName(&self, name: String) {
        let mut existence: u8 = 0;
        for person in &self.db {
            if person.name == name {
                println!(
                    "[name: {}, address: {}]", 
                    &person.name, 
                    &person.address
                );
                existence += 1;
            } 
        }
        if existence == 0 {
            println!("{} is not in the addressbook", &name);
        } else {
            println!("Find : {}", existence);
        }
    }

    pub fn findAddress(&self, address: String) {
        let mut existence: u8 = 0;
        for person in &self.db {
            if person.address == address {
                println!(
                    "[name: {}, address: {}]", 
                    &person.name, 
                    &person.address
                );
                existence += 1;
            }
        }
        if existence == 0 {
            println!("{} is not in to addressbook", &address);
        } else {
            println!("Find : {}", existence);
        } 
    }

    pub fn is_included(&self, name: &String, address: &String) -> bool {
        let mut existence = false;
        for person in &self.db {
            if &person.name == name && &person.address == address {
                existence = true;
                return existence
            } 
        }
        existence
    }
}
