use my_lang::scanner::scan as scan;


#[test]
fn test_scanner() {
    let token_list = scan("./test.ben");
    for t in token_list.into_iter() {
        println!("{}",t);
    }
}
