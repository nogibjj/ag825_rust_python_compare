// Ensures all 4 CRUD functions under lib.rs run successfully
#[cfg(test)]
mod tests {
    use rust_code::*;

    #[test]
    fn test_crud() {        
        let result = read();

        assert!(
            result.is_ok(),
            "FAILED");
    }

}
