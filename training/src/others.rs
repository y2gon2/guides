/// 그냥 공부하면서 rust 관련 찾은 내용들. 

/// 01. zero Cost Abstrctions
/// Zero Cost Abstractions means adding higher-level programming concepts,
/// like generics, collections and so on do not come with a run-time cost,
/// only compiler time cost (the code will be slower to compile). 
/// Any operation on zero-cost abstractions is as fast as 
/// you would write out matching functionality by hand using lower-level programming concepts 
/// (for loops, counters, ifs and so on).
/// https://stackoverflow.com/questions/69178380/what-does-zero-cost-abstraction-mean

/// 02. Smart Pointer in Rust (Reference Counting)
/// https://www.youtube.com/watch?v=M9Owp3iLigg
/// enum type 을 사용하여 (아래와 같은) linked list 을 생성해보면
/// 
///    b ->3| ---\
///            a-> 5| -> 10| -> Nil| 
///    c ->4| ___/
#[test]
fn ex02_error() {
    enum List{
        Cons(i32, Box<List>), // 다른 list 가리킴
        Nil,
    }

    // use crate::List::{Cons, Nil};

    let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    let b = List::Cons(3, Box::new(a));
    // let c = List::Cons(4, Box::new(a)); // a 에 대한 ownership move 됨 error 발생
}

#[test]
fn ex02() {
    use std::rc::Rc;

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b = List::Cons(3, Rc::clone(&a)); 
    // Rc::clone(&x) -> deep copy X, reference counter 만 증가시킴 (strong reference count??)
    let c = List::Cons(4, a.clone());
    // Deep copy 를 발생시킴. 

    println!(" a addr: {:p}", a);   //  a addr: 0x131f6347400 ?? 스마트 포인터는 heap memory 주소를 그냥 찍을 수 있는듯?
    println!("&b addr: {:p}", &b);  // &b addr: 0xcf60fee38   길이가 다르다...
    println!("&c addr: {:p}", &c);  // &c addr: 0xcf60fee48
}   

/// 03. type Send & Sync 
/// A type is Send if it is safe to send it to another thread. -> 다른 thread 로 ownership move 가능??
/// A type is Sync if it is safe to share between threads (T is Sync if and only if &T is Send). -> mutable 가능한가????

/// 04. TcpStream
/// 
#[test]
fn ex04() {
    use std::io::prelude::*;
    use std::net::TcpStream;

    exe().unwrap();

    fn exe() -> std::io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;

        stream.write(&[1])?;
        stream.read(&mut[0;128])?;
        Ok(())
    }

}


/// -------------------------------------------------------------- 
pub fn eof() {}