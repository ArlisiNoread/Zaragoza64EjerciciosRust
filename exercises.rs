pub mod Similarity;
mod C1e1;
mod C1e2;
mod C1e3;
mod C1e4;
mod C1e5;


pub fn factory(code: &str) -> Option<Box<dyn Similarity::Similarity>> {
    let val: &str = &(code.to_lowercase())[..];
    match val {
        "c1e1" => Option::Some(Box::new(C1e1::C1e1 {})),
        "c1e2" => Option::Some(Box::new(C1e2::C1e2 {})),
        "c1e3" => Option::Some(Box::new(C1e3::C1e3 {})),
        "c1e4" => Option::Some(Box::new(C1e4::C1e4 {})),
        "c1e5" => Option::Some(Box::new(C1e5::C1e5 {})),
        _ => Option::None
    }
}