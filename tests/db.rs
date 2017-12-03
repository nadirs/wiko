#[cfg(test)]
extern crate wiko;


mod helpers;
use helpers::TestHelper;


#[test]
pub fn test_init_wiko() {
    TestHelper::new_wiko();
}

#[test]
pub fn test_get_post() {
    let post = TestHelper::get_post();
    println!("{:?}", post);
}

#[test]
pub fn test_create_new_post() {
    let inserted = TestHelper::insert_post();
    println!("{:?}", inserted);
}

#[test]
pub fn test_create_post_revisions() {
    let inserted = TestHelper::insert_post_revision();
    println!("{:?}", inserted);
}
