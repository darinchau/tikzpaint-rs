pub fn is_true(x: Option<bool>) -> bool {
    x.is_some() && x.unwrap()
}