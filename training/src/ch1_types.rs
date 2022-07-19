/// rust type 과 관련하여.

/// 1. type 별 size 확인 
/// (VS code 기준) #[test] 아래 Run Test 를 클릭해서 실행가능.
#[test]
fn ex01() {
    use std::mem::size_of_val;
    use std::any::type_name;

    // fn type_of<T>(_: T) -> &'static str {
    //     type_name::<T>()
    // }

    let a: u8 = 1;
    let b: u8 = 100;
    let c: u32 = 100;
    println!("size a:{}, b:{}, c:{}", size_of_val(&a), size_of_val(&b), size_of_val(&c)); 
    // size a:1, b:1, c:4 
    // 그 이유는 ??

    let s1 = "abc".to_string();
    let s2 = "Returns the size of the pointed-to value in bytes.".to_string();
    println!("size s1:{}, s2:{}", size_of_val(&s1), size_of_val(&s2));    
    // size s1:24, s2:24
    // 그 이유는?? 
    // 만약 String type variable 에 법전 내용을 통째로 넣는다면 그 size 는 어떻게 될까?
    
    let arr = [0;100];
    let ve = vec![0;100];

    println!("size arr:{}, ve:{}", size_of_val(&arr), size_of_val(&ve));
    // size arr:400, ve:24
    // 그 이유는? 
}

/// 2. stack memory & heap memory 와 관련하여
/// 참고 사이트: https://velog.io/@hyo123/Stack%EA%B3%BCHeap-%EA%B7%B8%EB%A6%AC%EA%B3%A0-%EA%B8%B0%EB%B3%B8%EC%9E%90%EB%A3%8C%ED%98%95-%EB%B3%B5%ED%95%A9%EC%9E%90%EB%A3%8C%ED%98%95
/// 해당 내용은 JS 에 대한 내용이기 때문에 rust 랑 차이점이 있다. 
/// 어떤 차이점이 있을까?
/// 해당 블로그의 내용을 1번의 질문들과 연관하여 생각해 보기. 



/// 
/// 
/// 
/// --------------------------------
pub fn eof() {}

