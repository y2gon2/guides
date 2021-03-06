// -----------------------------------------------------------------------



// -----------------------------------------------------------------------
// 기존 주소 변경 적용
// #![allow(non_snake_case)]

// use std::fs::{self, OpenOptions};
// use std::io::{stdin, Write};
// //use std::cell::RefCell;

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct Person {
//     name: String,
//     address: String,
// }

// impl Person {
//     pub fn new(inputName: String, inputAddress: String) -> Self {
//         Person { 
//             name: inputName, 
//             address: inputAddress,
//         }
//     }

//     pub fn showPerson(&self) {
//         println!("name : {},  address : {}]", &self.name, &self.address);
//     }
// }

// #[derive(Debug)]
// pub struct AddressBook {
//     db: Vec<Person>,
// }

// impl AddressBook {
//     pub fn new() -> Self {
//         AddressBook { db: Vec::<Person>::new() }
//     }

//     pub fn openBook (&mut self, fileName: &str) {
//         let mut fileData: String = fs::read_to_string(fileName)
//             .expect("Searching and reading the file error");
    
//         while fileData.chars().count() > 0 {
//             let nameSlice = fileData.find('\t').unwrap();
//             let name = fileData[..nameSlice].to_string();
            
//             let addSlice = fileData.find('\n').unwrap();
//             let address = fileData[nameSlice + 1..addSlice].to_string();

//             &mut self.db.push(Person::new(name, address));

//             fileData = fileData[addSlice + 1 ..].to_string();
//         }
//     } 

//     pub fn saveBook(&self, fileName: &str) {
//         println!("6. Save the update to a file.");
//         let mut file = OpenOptions::new()
//             .write(true)
//             .open(fileName)
//             .expect("saving the data error");
        
//         for data in &self.db {
//             let writing = format!("{}\t{}\n", data.name, data.address);
//             file.write(writing.as_bytes()).expect("writing error");
//         }

//         println!("The file is updated!");
//     }

//     pub fn showAllPerson(&self) {
//         println!("3. Check the all memorized personal information.");
//         for person in &self.db {
//             println!("{} : {}", &person.name, &person.address);
//         }
//     }

//     pub fn pushPerson(&mut self, person: Person) {
//         &mut self.db.push(person.clone());
//         println!("[{} : {}] is added", &person.name, &person.address);
//     }

//     pub fn delPerson(&mut self, person: &Person) {
//         &mut self.db.retain(|x| x != person);
//         println!("[{} : {}] is deleted", person.name, person.address);
//     }

//     pub fn updateAddress(&mut self, name: String, oldAddress: String, newAddress: String) {
//         for person in &mut self.db {
//             if person.name == name && person.address == oldAddress {
//                 person.address = newAddress;
//                 println!("[{} : {}] is updated", person.name, person.address);
//                 break;
//             }
//         }
//     }

//     pub fn findName(&self, name: String) {
//         let mut existence: u8 = 0;
//         for person in &self.db {
//             if person.name == name {
//                 println!(
//                     "[name: {}, address: {}]", 
//                     &person.name, 
//                     &person.address
//                 );
//                 existence += 1;
//             } 
//         }
//         if existence == 0 {
//             println!("{} is not in the addressbook", &name);
//         } else {
//             println!("Find : {}", existence);
//         }
        
//     }

//     pub fn findAddress(&self, address: String) {
//         let mut existence: u8 = 0;
//         for person in &self.db {
//             if person.address == address {
//                 println!(
//                     "[name: {}, address: {}]", 
//                     &person.name, 
//                     &person.address
//                 );
//                 existence += 1;
//             }
//         }
//         if existence == 0 {
//             println!("{} is not in to addressbook", &address);
//         } else {
//             println!("Find : {}", existence);
//         }
        
//     }

//     pub fn is_included(&self, name: &String, address: &String) -> bool {
//         let mut existence = false;
//         for person in &self.db {
//             if &person.name == name && &person.address == address {
//                 existence = true;
//                 return existence
//             } 
//         }
//         existence
//     }


// }

// pub fn inputName() -> String {
//     println!("Please input your name.");
//     let mut nameBuffer = String::new();
//     let stdinName = stdin().read_line(&mut nameBuffer).expect("input error");
//     nameBuffer[..nameBuffer.len() - 2].to_string()
// }

// pub fn inputAddress() -> String {
//     println!("Please input an address.");
//     let mut addrBuffer = String::new();
//     let stdinAddr = stdin().read_line(&mut addrBuffer).expect("input error");
//     addrBuffer[..addrBuffer.len() - 2].to_string()
// }

// pub fn inputNumber() -> u8 {
//     let inputError: u8 = 99;

//     let mut numBuffer = String::new();
//     let stdinNum = stdin().read_line(&mut numBuffer).expect("input error");
//     let orderNum = numBuffer[..numBuffer.len() - 2].parse::<u8>();

//     match orderNum {
//         Ok(orderNum) => return orderNum,
//         Err(error) => return inputError,
//     }
// }

// pub fn showMenu() {
//     println!("-----------------------------------------------------------------"); 
//     println!("Please input the order number.");
//     println!(" 0 : return to main menu without update.");
//     println!(" 1 : confirm to save this change and return to main menu.");
//     println!(" 2 : confirm to save this change and update inpormation more.");
//     println!(" 3 : something wrong about the information. Input again without saving.");
//     println!("-----------------------------------------------------------------"); 
// }

// pub fn addPerson(addressBook : &mut AddressBook) {
//     println!("1. Add a new personal infomation.");

//     let mut dataInputAgain = true;
//     while dataInputAgain {
//         let name = inputName();
//         let address = inputAddress();    
//         if addressBook.is_included(&name, &address) {
//             println!("The same name and address information is already in the AddressBook");
//             println!("Please input the 0 if you want to back to main menu or input any others.");

//             let order = inputNumber();
//             match inputNumber() {
//                 0 => dataInputAgain = false,
//                 _ => {},
//             }

//         } else {        
//             let mut orderInputAgain = true;
//             while orderInputAgain {
//                 println!("Prease check the input information.");

//                 // 만약 아래 variable 선언이 while 문 앞에 있다면, 에러 발생
//                 // (아마도) 내부 while 문이 반복되는 상항에서, 기존의 newPerson ownership 이 move 해버린 상태에서
//                 // 다시 선언하지 않고 돌아기기 때문(인듯)
//                 // 추가로 while 문 내부로 넣었지만, name, address 를 그냥 가져오게되면, 
//                 // name, address ownership 이 반복문 실행 시, 1회 사용으로 버려지기 때문에 clone 사용 (해야 하는듯)
//                 let newPerson = Person::new(name.clone(), address.clone()); // 

//                 newPerson.showPerson();
//                 showMenu();
//                 match inputNumber() {
//                     0 => {
//                         dataInputAgain = false;
//                         orderInputAgain = false;
//                         },
//                     1 => {
//                         addressBook.pushPerson(newPerson);
//                         dataInputAgain = false;
//                         orderInputAgain = false;
//                     },
//                     2 => {
//                         addressBook.pushPerson(newPerson);
//                         orderInputAgain = false;
//                     },
//                     3 => orderInputAgain = false,
//                     _ => println!("Please enter one vaild number (0 - 3)."),
//                 }                    
//             }
//         }    
//     }
// }


// pub fn searchPerson(addressBook : &AddressBook) {
//     println!("2. Search a personal information in the data.");
//     let mut dataInputAgain = true;
//     while dataInputAgain {
//         println!("Please select the searching keyword.");
//         println!("0: return to main menu, 1: Name,  2: Address");
//         let order = inputNumber();

//         match order {
//             0 => dataInputAgain = false,
//             1 => {
//                 let findPerson = inputName();
//                 addressBook.findName(findPerson)
//             },
//             2 => {
//                 let findPerson = inputAddress();
//                 addressBook.findAddress(findPerson)
//             },
//             _ => println!("Please enter one vaild number (0 - 2).")
//         }
//     }
// }

// pub fn removePerson(addressBook: &mut AddressBook) {
//     println!("4. Remove one memorized personal information.");
//     let mut dataInputAgain = true;
//     while dataInputAgain {
//         let name = inputName();
//         let address = inputAddress();

//         if !addressBook.is_included(&name, &address) {
//             println!("This is not in the AddressBook.");
//             println!("Please input the 0 if you want to back to main menu or again input any others.");

//             match inputNumber() {
//                 0 => dataInputAgain = false,
//                 _ => {},
//             }
//         } else {
//             let rmPerson = Person::new(name, address); 
            
//             let mut orderInputAgain = true;
//             while orderInputAgain {
//                 rmPerson.showPerson();
//                 showMenu();

//                 match inputNumber() {
//                     0 => {
//                         dataInputAgain = false;
//                         orderInputAgain = false;
//                     },
//                     1 => {
//                         addressBook.delPerson(&rmPerson);
//                         dataInputAgain = false;
//                         orderInputAgain = false;
//                     },
//                     2 => {
//                         addressBook.delPerson(&rmPerson);
//                         orderInputAgain = false;
//                     },
//                     3 => orderInputAgain = false,
//                     _ => println!("Please enter one vaild number (0 - 3)."),
//                 }
//             }
//         }
//     }
// }

// pub fn changeAddress (addressBook: &mut AddressBook) {
//     println!("5. Change one memorized personal information(address).");

//     let mut dataInputAgain = true;
//     while dataInputAgain {
//         let name = inputName();
//         let oldAddress = inputAddress();

//         if !addressBook.is_included(&name, &oldAddress) {
//             println!("This is not in the AddressBook.");
//             println!("Please input the 0 if you want to back to main menu or again input any others.");

//             match inputNumber() {
//                 0 => dataInputAgain = false,
//                 _ => {},
//             }
//         } else {
//             println!("Please input new address to save.");
//             let newAddress = inputAddress();

//             let mut orderInputAgain = true;
//             while orderInputAgain {
//                 showMenu();
//                 match inputNumber() {
//                     0 => {
//                         dataInputAgain = false;
//                         orderInputAgain = false;
//                     },
//                     1 => {
//                         addressBook.updateAddress(name.clone(), oldAddress.clone(), newAddress.clone());
//                         dataInputAgain = false;
//                         orderInputAgain = false;
//                     },
//                     2 => {
//                         addressBook.updateAddress(name.clone(), oldAddress.clone(), newAddress.clone());
//                         orderInputAgain = false;
//                     },
//                     3 => orderInputAgain = false,
//                     _ => println!("Please enter one vaild number (0 - 3)."),
//                 }
//             } 
//         }
//     }
// }

// fn main() {
//     let fileName = "addressbook/src/book.txt";
//     let mut addressBook = AddressBook::new();
//     addressBook.openBook(fileName);

//     loop {
//         println!("-----------------------------------------------------------------");
//         println!("Please enter the number of the job you want.");
//         println!("1. Add a new personal infomation.");
//         println!("2. Search a personal information in the data.");
//         println!("3. Check the all memorized personal information.");
//         println!("4. Remove one memorized personal information.");
//         println!("5. Change one memorized personal information(address).");
//         println!("6. Save the update to a file.");
//         println!("Please input '0' if you want to quit this program.");
//         println!("-----------------------------------------------------------------");   
//         println!("Please input order number. (0 - 6)");

//         match inputNumber() {
//             1 => addPerson(&mut addressBook),
//             2 => searchPerson(&addressBook),
//             3 => addressBook.showAllPerson(),
//             4 => removePerson(&mut addressBook),
//             5 => changeAddress(&mut addressBook),
//             6 => addressBook.saveBook(fileName),
//             0 => {
//                 println!("Bye!!");
//                 break;
//             },
//             _ => println!("Please input one vaild number (0 - 6)."),
//         }
//     }
// }

// -----------------------------------------------------------------------
// refactoring 이전 및 기존 주소 변경 미적용

// #![allow(non_snake_case)]

// use std::fs::{self, OpenOptions};
// use std::io::{stdin, Write};
// //use std::cell::RefCell;

// #[derive(Debug, PartialEq, Eq)]
// pub struct Person {
//     name: String,
//     address: String,
// }

// impl Person {
//     pub fn new(inputName: String, inputAddress: String) -> Self {
//         Person { 
//             name: inputName, 
//             address: inputAddress,
//         }
//     }

//     pub fn showPerson(&self) {
//         println!("name : {},  address : {}]", &self.name, &self.address);
//     }
// }

// #[derive(Debug)]
// pub struct AddressBook {
//     db: Vec<Person>,
// }

// impl AddressBook {
//     pub fn new() -> Self {
//         AddressBook { db: Vec::<Person>::new() }
//     }

//     pub fn openBook (&mut self, fileName: &str) {
//         let mut fileData: String = fs::read_to_string(fileName)
//             .expect("Searching and reading the file error");
    
//         while fileData.chars().count() > 0 {
//             let nameSlice = fileData.find('\t').unwrap();
//             let name = fileData[..nameSlice].to_string();
            
//             let addSlice = fileData.find('\n').unwrap();
//             let address = fileData[nameSlice + 1..addSlice].to_string();

//             &mut self.db.push(Person::new(name, address));

//             fileData = fileData[addSlice + 1 ..].to_string();
//         }
//     } 

//     pub fn saveBook(&self, fileName: &str) {
//         let mut file = OpenOptions::new()
//             .write(true)
//             .open(fileName)
//             .expect("saving the data error");
        
//         for data in &self.db {
//             let writing = format!("{}\t{}\n", data.name, data.address);
//             file.write(writing.as_bytes()).expect("writing error");
//         }

//         println!("The file is updated!");
//     }

//     pub fn showAllPerson(&self) {
//         for person in &self.db {
//             println!("{} : {}", person.name, person.address);
//         }
//     }

//     pub fn pushPerson(&mut self, person: Person) {
//         &mut self.db.push(person);
//     }

//     pub fn delPerson(&mut self, person: &Person) {
//         &mut self.db.retain(|x| x != person);
//         println!("[{} : {}] is deleted", person.name, person.address);
//     }

//     pub fn updateAddress(&mut self, personFrom: &Person, personTo: &Person) {

//     }

//     pub fn findName(&self, name: String) {
//         let mut existence: u8 = 0;
//         for person in &self.db {
//             if person.name == name {
//                 println!(
//                     "[name: {}, address: {}]", 
//                     &person.name, 
//                     &person.address
//                 );
//                 existence += 1;
//             } 
//         }
//         if existence == 0 {
//             println!("{} is not in the addressbook", &name);
//         } else {
//             println!("Find : {}", existence);
//         }
        
//     }

//     pub fn findAddress(&self, address: String) {
//         let mut existence: u8 = 0;
//         for person in &self.db {
//             if person.address == address {
//                 println!(
//                     "[name: {}, address: {}]", 
//                     &person.name, 
//                     &person.address
//                 );
//                 existence += 1;
//             }
//         }
//         if existence == 0 {
//             println!("{} is not in to addressbook", &address);
//         } else {
//             println!("Find : {}", existence);
//         }
        
//     }

//     pub fn is_included(&self, name: &String, address: &String) -> bool {
//         let mut existence = false;
//         for person in &self.db {
//             if &person.name == name && &person.address == address {
//                 existence = true;
//                 return existence
//             } 
//         }
//         existence
//     }


// }

// pub fn inputName() -> String {
//     println!("Please input your name.");
//     let mut nameBuffer = String::new();
//     let stdinName = stdin().read_line(&mut nameBuffer).expect("input error");
//     nameBuffer[..nameBuffer.len() - 2].to_string()
// }

// pub fn inputAddress() -> String {
//     println!("Please input an address.");
//     let mut addrBuffer = String::new();
//     let stdinAddr = stdin().read_line(&mut addrBuffer).expect("input error");
//     addrBuffer[..addrBuffer.len() - 2].to_string()
// }

// pub fn inputNumber() -> u8 {
//     let inputError: u8 = 99;

//     let mut numBuffer = String::new();
//     let stdinNum = stdin().read_line(&mut numBuffer).expect("input error");
//     let orderNum = numBuffer[..numBuffer.len() - 2].parse::<u8>();

//     match orderNum {
//         Ok(orderNum) => return orderNum,
//         Err(error) => return inputError,
//     }
// }

// pub fn showMenu() {
//     println!("-----------------------------------------------------------------"); 
//     println!("Please input the order number.");
//     println!(" 0 : return to main menu without update.");
//     println!(" 1 : confirm to save this change and return to main menu.");
//     println!(" 2 : confirm to save this change and update inpormation more.");
//     println!(" 3 : something wrong about the information. Input again without saving.");
//     println!("-----------------------------------------------------------------"); 
// }

// pub fn addPerson(addressBook : &mut AddressBook) {
//     println!("1. Add a new personal infomation.");

//     let mut dataInputAgain = true;
//     while dataInputAgain {
//         let name = inputName();
//         let address = inputAddress();    
//         if addressBook.is_included(&name, &address) {
//             println!("The same name and address information is already in the AddressBook");
//             println!("Please input the 0 if you want to back to main menu or input any others.");

//             let order = inputNumber();
//             match inputNumber() {
//                 0 => dataInputAgain = false,
//                 _ => dataInputAgain = true,
//             }

//         } else {        
//             let mut orderInputAgain = true;
//             while orderInputAgain {
//                 println!("Prease check the input information.");

//                 // 만약 아래 variable 선언이 while 문 앞에 있다면, 에러 발생
//                 // (아마도) 내부 while 문이 반복되는 상항에서, 기존의 newPerson ownership 이 move 해버린 상태에서
//                 // 다시 선언하지 않고 돌아기기 때문(인듯)
//                 // 추가로 while 문 내부로 넣었지만, name, address 를 그냥 가져오게되면, 
//                 // name, address ownership 이 반복문 실행 시, 1회 사용으로 버려지기 때문에 clone 사용 (해야 하는듯)
//                 let newPerson = Person::new(name.clone(), address.clone()); // 

//                 newPerson.showPerson();
//                 showMenu();
//                 match inputNumber() {
//                     0 => {
//                         dataInputAgain = false;
//                         orderInputAgain = false;
//                         },
//                     1 => {
//                         addressBook.pushPerson(newPerson);
//                         dataInputAgain = false;
//                         orderInputAgain = false;
//                     },
//                     2 => {
//                         addressBook.pushPerson(newPerson);
//                         orderInputAgain = false;
//                     },
//                     3 => orderInputAgain = false,
//                     _ => println!("Please enter one vaild number (0 - 3)."),
//                 }                    
//             }
//         }    
//     }
// }


// pub fn searchPerson(addressBook : &AddressBook) {
//     let mut dataInputAgain = true;
//     while dataInputAgain {
//         println!("Please select the searching keyword.");
//         println!("0: return to main menu, 1: Name,  2: Address");
//         let order = inputNumber();

//         match order {
//             0 => dataInputAgain = false,
//             1 => {
//                 let findPerson = inputName();
//                 addressBook.findName(findPerson)
//             },
//             2 => {
//                 let findPerson = inputAddress();
//                 addressBook.findAddress(findPerson)
//             },
//             _ => println!("Please enter one vaild number (0 - 2).")
//         }
//     }
// }

// pub fn removePerson(addressBook: &mut AddressBook) {
//     let mut DataInputAgain = true;
//     while DataInputAgain {
//         let name = inputName();
//         let address = inputAddress();

//         if !addressBook.is_included(&name, &address) {
//             println!("This is not in the AddressBook.");
//             println!("Please input the 0 if you want to back to main menu or input any others.");

//             match inputNumber() {
//                 0 => DataInputAgain = false,
//                 _ => DataInputAgain = true,
//             }
//         } else {
//             let rmPerson = Person::new(name, address); 
            
//             let mut orderInputAgain = true;
//             while orderInputAgain {
//                 rmPerson.showPerson();
//                 showMenu();

//                 match inputNumber() {
//                     0 => {
//                         DataInputAgain = false;
//                         orderInputAgain = false;
//                     },
//                     1 => {
//                         addressBook.delPerson(&rmPerson);
//                         DataInputAgain = false;
//                         orderInputAgain = false;
//                     },
//                     2 => {
//                         addressBook.delPerson(&rmPerson);
//                         orderInputAgain = false;
//                     },
//                     3 => orderInputAgain = false,
//                     _ => println!("Please enter one vaild number (0 - 3)."),
//                 }
//             }
//         }
//     }
// }

// fn main() {
//     let fileName = "addressbook/src/book.txt";
//     let mut addressBook = AddressBook::new();
//     addressBook.openBook(fileName);

//     loop {
//         println!("-----------------------------------------------------------------");
//         println!("Please enter the number of the job you want.");
//         println!("1. Add a new personal infomation.");
//         println!("2. Search a personal information in the data.");
//         println!("3. Check the all memorized personal information.");
//         println!("4. Remove one memorized personal information.");
//         println!("5. Update one memorized personal information.");
//         println!("6. Save the added address record to a file.");
//         println!("Please enter '0' if you want to quit this program.");
//         println!("-----------------------------------------------------------------");   
//         println!("Please input order number. (0 - 6)");

//         let orderNum = inputNumber();
//         match orderNum {
//             1 => addPerson(&mut addressBook),
//             2 => searchPerson(&addressBook),
//             3 => addressBook.showAllPerson(),
//             4 => removePerson(&mut addressBook),
//             5 => println!("To do"),
//             6 => addressBook.saveBook(fileName),
//             0 => {
//                 println!("Bye!!");
//                 break;
//             },
//             _ => println!("Please enter one vaild number (0 - 6)."),
//         }

//     }
// }



// -----------------------------------------------------------------------
// Recell 및 &str value 적용

// #![allow(non_snake_case)]

// use std::fs::{self, OpenOptions};
// use std::io::{stdin, Write};

// #[derive(Debug, PartialEq, Eq)]
// pub struct Person<'a> {
//     name: RefCell<&'a str>,
//     address: RefCell<&'a str>,
// }

// impl <'a> Person<'a> {
//     pub fn new(inputName: &'a str, inputAddress: &'a str) -> Self {
//         Person { 
//             name: RefCell::new(inputName), 
//             address: RefCell::new(inputAddress),
//         }
//     }
// }

// #[derive(Debug)]
// pub struct AddressBook<'a> {
//     addressList: Vec<Person<'a>>,
// }

// impl <'a> AddressBook<'a> {
//     pub fn new() -> Self {
//         AddressBook { addressList: Vec::<Person>::new() }
//     }

//     pub fn openBook (&mut self, fileName: &str) {
//         let fileRead: String = fs::read_to_string(fileName)
//             .expect("Searching and reading the file error");
//         let mut fileData: &'a str = fileRead.as_str();

//         while fileData.chars().count() > 0 {
//             let nameSlice = fileData.find('\t').unwrap();
//             let name: &'a str = &fileData[..nameSlice];
            
//             let addSlice = fileData.find('\n').unwrap();
//             let address: &'a str = &fileData[nameSlice + 1..addSlice];

//             &mut self.addressList.push(Person::new(name, address));

//             fileData = &fileData[addSlice + 1 ..];
//         }
//     } 

//     pub fn saveBook(&self, fileName: &str) {
//         let mut file = OpenOptions::new()
//             .write(true)
//             .open(fileName)
//             .expect("saving the data error");
        
//         for data in &self.addressList {
//             let writing = format!("{}\t{}\n", data.name.borrow(), data.address.borrow());
//             file.write(writing.as_bytes()).expect("writing error");
//         }

//         println!("The file is updated!");
//     }

//     pub fn showAllPerson(&self) {
//         for person in &self.addressList {
//             println!("{} : {}", person.name.borrow(), person.address.borrow());
//         }
//     }

//     pub fn addPerson(&mut self, person: Person<'a>) {
//         &mut self.addressList.push(person);
//     }

//     pub fn delPerson(&mut self, person: &Person<'a>) {
//         &mut self.addressList.retain(|x| x != person);
//         println!("[{} : {}] is deleted", person.name.borrow(), person.address.borrow());
//     }

//     pub fn updateAddress(&mut self, personFrom: &Person, personTo: &Person) {

//     }

//     pub fn findPerson(&self, findPerson: &'a str) {
//         let mut exitence: u8 = 0;
//         for person in &self.addressList {
//             if person.name == RefCell::new(findPerson) {
//                 println!(
//                     "[name: {}, address: {}] is in the addressbook", 
//                     &person.name.borrow(), 
//                     &person.address.borrow()
//                 );
//             } 
//             exitence += 1;
//         }
//         if exitence == 0 {
//             println!("{} is not in the addressbook", findPerson);
//         } else {
//             println!("Find : {}", &exitence);
//         }
//     }


// }

// pub fn inputPerson() {
//     println!("Please input your name.");
//     let mut nameBuffer = String::new();
//     let stdinName = stdin().read_line(&mut nameBuffer).expect("input error");
    
//     println!("Please input an address.");
//     let mut addBuffer = String::new();
//     let stdinAdd = stdin().read_line(&mut addBuffer).expect("input error");
    
//     let inputName = &nameBuffer[..nameBuffer.len() - 2];
//     let inputAddress = &addBuffer[..addBuffer.len() - 2];
    
//     Person::new(inputName, inputAddress);
// }


// fn main() {
//     println!("Hello, world!");
// }
