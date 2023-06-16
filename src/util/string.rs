pub trait IsEmptyString {
    fn is_empty(&self) -> bool;
}

impl IsEmptyString for Option<String> {
    fn is_empty(&self) -> bool {
        return match self {
            Some(s) => s.is_empty(),
            _ => true,
        };
    }
}

impl IsEmptyString for Option<&str> {
    fn is_empty(&self) -> bool {
        return match self {
            Some(s) => s.is_empty(),
            _ => true,
        };
    }
}
pub fn capitalize(s: &str) -> String {

    let mut c = s.chars();

    match c.next(){
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }

}