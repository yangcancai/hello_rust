#[cfg(test)]
mod tests {
    use crate::execute;
    #[test]
    fn it_works() {
        execute();
    }
}
fn splite_string(str: &str) -> &str{
    for (i, c) in str.as_bytes().iter().enumerate(){
        if *c == b' '{
            return &str[..i];
        }    
    }
    &str[..]
}
pub fn execute(){
    let hello = String::from("hello world");
    let first = splite_string(&hello);
    assert_eq!(first, "hello");
    assert_eq!(hello, "hello world");

}

