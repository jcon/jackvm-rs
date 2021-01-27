pub mod compiler;
pub mod vm;
mod jack_os;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
