pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    #[should_panic]
    fn it_not_works(){
        let result = add(1, 1);
        assert_eq!(3, result);
    }
}
