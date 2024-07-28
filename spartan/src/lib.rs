pub mod spypoint;
pub mod client;
pub mod sys;
pub mod cameras;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
