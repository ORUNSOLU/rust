
pub mod cipher; 

#[cfg(test)]
mod test_module {
    #[test]
    fn test_bytes() {
        // checking what BYTES ENCODING looks like
        let bb = base64::encode(b"Hellow world!");
        print!("{}", bb);
    }

}