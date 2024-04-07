pub fn defang_i_paddr(address: String) -> String {
    address.replace(".", "[.]")
}

#[cfg(test)]
mod tests {
    use crate::strings::defanging_an_ip_address::defang_i_paddr;

    #[test]
    fn test_defang_i_paddr() {
        assert_eq!(
            defang_i_paddr("1.1.1.1".to_string()),
            "1[.]1[.]1[.]1".to_string()
        );
    }
}
