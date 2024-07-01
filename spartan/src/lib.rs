mod spypoint;
mod client;


pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
