mod common; 

#[test]
fn test_return_1() {
    common::setup();
    assert_eq!(1, bin_pack::return_1()); 
}