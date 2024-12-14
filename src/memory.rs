pub mod memory {
    pub enum AddressErrorKind {
        OutOfRange,
        Reserved,
    }

    pub struct InvalidAddressError {
        address: u16,
        kind: AddressErrorKind,
    }

    pub struct Memory {
        memory: [u8; 4096],
    }

    impl Memory {
        pub fn new() -> Self {
            Self { memory: [0; 4096] }
        }

        pub fn wipe(&mut self) {
            self.memory = [0; 4096]
        }

        pub fn read(&self, address: u16) -> Result<u16, InvalidAddressError> {
            match address {
                0x000..0x200 => Err(InvalidAddressError {
                    address,
                    kind: AddressErrorKind::Reserved,
                }),
                0x200..=0xFFF => Ok(address),
                _ => Err(InvalidAddressError {
                    address,
                    kind: AddressErrorKind::OutOfRange,
                }),
            }
        }

        pub fn write(&mut self, address: u16, value: u8) -> Result<(), InvalidAddressError> {
            match address {
                0x000..0x200 => Err(InvalidAddressError {
                    address,
                    kind: AddressErrorKind::Reserved,
                }),
                0x200..=0xFFF => {
                    self.memory[address as usize] = value;
                    Ok(())
                }
                _ => Err(InvalidAddressError {
                    address,
                    kind: AddressErrorKind::OutOfRange,
                }),
            }
        }
    }
}
