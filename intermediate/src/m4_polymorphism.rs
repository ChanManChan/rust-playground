use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EthereumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum Address String")
        }
    }
}

impl EthereumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

// function that can accept both string reference and Address type address
fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
    let converted_address: Address = address.convert_address().unwrap();
    converted_address
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_polymorphism() {
        let address: Address = Address::from_str("0x4675C7e5BaAFBFFbca748158bEcBA61ef3b0a263").unwrap();
        
        let new_address: Address = get_ethereum_data(address);

        assert_eq!(address, new_address);

        let string_address: Address = get_ethereum_data("0x4675C7e5BaAFBFFbca748158bEcBA61ef3b0a263");

        assert_eq!(address, string_address);
    }
}