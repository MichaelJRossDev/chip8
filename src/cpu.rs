pub mod cpu {
    use crate::{instructions::instructions::Instruction, memory::memory::Memory};

    struct Chip8CPU {
        v: [u8; 16],
        memory: Memory,
        i: u16,
        dt: u8,
        st: u8,
        pc: u16,
        sp: u8, //docs are vague about this one. Might have to be u16
        stack: [u16; 16],
        display: [[bool; 32]; 64],
    }

    impl Chip8CPU {
        fn get(&self, register: u8) -> u8 {
            self.v[register as usize]
        }

        fn get_mutable(&mut self, register: u8) -> &mut u8 {
            &mut self.v[register as usize]
        }

        fn execute(&mut self, instruction: Instruction) {
            match instruction {
                Instruction::ClearDisplay => self.clear_display(),
                Instruction::Return => self.return_from_subroutine(),
                Instruction::Jump(addr) => self.jump(addr),
                Instruction::Call(addr) => self.call(addr),
                Instruction::SkipIfEqual(vx, byte) => self.skip_if_equal(vx, byte),
                Instruction::SkipIfNotEqual(vx, byte) => self.skip_if_not_equal(vx, byte),
                Instruction::SkipIfRegistersEqual(vx, vy) => self.skip_if_registers_equal(vx, vy),
                Instruction::Set(vx, byte) => self.set_register(vx, byte),
                Instruction::Add(vx, byte) => self.add(vx, byte),
                Instruction::CopyRegister(vx, vy) => self.copy_register(vx, vy),
                Instruction::Or(vx, vy) => self.or(vx, vy),
                Instruction::And(vx, vy) => self.and(vx, vy),
                Instruction::Xor(vx, vy) => self.xor(vx, vy),
                Instruction::AddRegisters(vx, vy) => self.add_registers(vx, vy),
                Instruction::Subtract(vx, vy) => self.subtract(vx, vy),
                Instruction::ShiftRight(vx) => self.shift_right(vx),
                Instruction::ReverseSubtract(vx, vy) => self.reverse_subtract(vx, vy),
                Instruction::ShiftLeft(vx) => self.shift_left(vx),
                Instruction::SkipIfRegistersNotEqual(vx, vy) => {
                    self.skip_if_registers_not_equal(vx, vy)
                }
                Instruction::SetI(addr) => self.set_i(addr),
                Instruction::JumpPlusV0(addr) => self.jump_plus_v0(addr),
                Instruction::RandomByte(vx, byte) => self.random_byte(vx, byte),
                Instruction::Draw(vx, vy, nibble) => self.draw(vx, vy, nibble),
                Instruction::SkipIfKeyPressed(vx) => self.skip_if_key_pressed(vx),
                Instruction::SkipIfKeyNotPressed(vx) => self.skip_if_key_not_pressed(vx),
                Instruction::StoreDelayTimer(vx) => self.store_delay_timer(vx),
                Instruction::StoreInput(vx) => self.store_input(vx),
                Instruction::SetDelayTimer(vx) => self.set_delay_timer(vx),
                Instruction::SetSoundTimer(vx) => self.set_sound_timer(vx),
                Instruction::AddToI(vx) => self.add_to_i(vx),
                Instruction::SetIToSpriteLocation(vx) => self.set_i_to_sprite_location(vx),
                Instruction::StoreBinaryCodedDecimal(vx) => self.store_binary_coded_decimal(vx),
                Instruction::StoreRegisters(vx) => self.store_registers_in_memory(vx),
                Instruction::LoadRegisters(vx) => self.load_registers_from_memory(vx),
            }
        }

        pub fn clear_display(&mut self) {
            self.display = [[false; 32]; 64];
        }

        pub fn return_from_subroutine(&mut self) {
            //! The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
            self.pc = self.stack[self.sp as usize];
            self.sp -= 1;
        }

        pub fn jump(&mut self, addr: u16) {
            //! The interpreter sets the program counter to addr.
            self.pc = addr;
        }

        pub fn call(&mut self, addr: u16) {
            //! The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to addr.
            self.sp += 1;
            self.stack[self.sp as usize] = addr;
        }

        pub fn skip_if_equal(&mut self, vx: u8, byte: u8) {
            //! The interpreter compares register Vx to byte, and if they are equal, increments the program counter by 2.
            if self.get(vx) == byte {
                self.pc += 2;
            }
        }

        pub fn skip_if_not_equal(&mut self, vx: u8, byte: u8) {
            //! The interpreter compares register Vx to byte, and if they are not equal, increments the program counter by 2.
            if self.get(vx) != byte {
                self.pc += 2;
            }
        }

        pub fn skip_if_registers_equal(&mut self, vx: u8, vy: u8) {
            //! The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
            if self.get(vx) == self.get(vy) {
                self.pc += 2;
            }
        }

        pub fn set_register(&mut self, vx: u8, byte: u8) {
            //! The interpreter puts the value byte into register Vx.
            *self.get_mutable(vx) = byte;
        }

        pub fn add(&mut self, vx: u8, byte: u8) {
            //! Adds the value byte to the value of register Vx, then stores the result in Vx.
            //! If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
            //! Only the lowest 8 bits of the result are kept, and stored in Vx.
            let result = self.get(vx) as u16 + byte as u16;
            *self.get_mutable(vx) = (result & 0xFF) as u8;
            *self.get_mutable(0xF) = (result > 0xFF) as u8;
        }

        pub fn copy_register(&mut self, vx: u8, vy: u8) {
            //! Stores the value of register Vy in register Vx.
            *self.get_mutable(vx) = self.get(vy);
        }

        pub fn or(&mut self, vx: u8, vy: u8) {
            //! Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
            *self.get_mutable(vx) |= self.get(vy);
        }

        pub fn and(&mut self, vx: u8, vy: u8) {
            //! Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
            *self.get_mutable(vx) &= self.get(vy);
        }

        pub fn xor(&mut self, vx: u8, vy: u8) {
            //! Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
            *self.get_mutable(vx) ^= self.get(vy);
        }

        pub fn add_registers(&mut self, vx: u8, vy: u8) {
            //! The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
            //! Only the lowest 8 bits of the result are kept, and stored in Vx.
            let result = self.get(vx) as u16 + self.get(vy) as u16;
            *self.get_mutable(vx) = (result & 0xFF) as u8;
            *self.get_mutable(0xF) = (result > 0xFF) as u8;
        }

        pub fn subtract(&mut self, vx: u8, vy: u8) {
            //! If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
            let x = self.get(vx);
            let y = self.get(vy);

            *self.get_mutable(0xF) = (x > y) as u8;
            *self.get_mutable(vx) -= y;
        }

        pub fn shift_right(&mut self, vx: u8) {
            //! If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
            *self.get_mutable(0xF) = self.get(vx) % 2;
            *self.get_mutable(vx) >>= 1;
        }

        pub fn reverse_subtract(&mut self, vx: u8, vy: u8) {
            //! If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
            let x = self.get(vx);
            let y = self.get(vy);

            *self.get_mutable(0xF) = (y > x) as u8;
            *self.get_mutable(vx) = y - x;
        }

        pub fn shift_left(&mut self, vx: u8) {
            // TODO: Implement the shift_left functionality
        }

        pub fn skip_if_registers_not_equal(&mut self, vx: u8, vy: u8) {
            // TODO: Implement the skip_if_registers_not_equal functionality
        }

        pub fn set_i(&mut self, addr: u16) {
            // TODO: Implement the set_i functionality
        }

        pub fn jump_plus_v0(&mut self, addr: u16) {
            // TODO: Implement the jump_plus_v0 functionality
        }

        pub fn random_byte(&mut self, vx: u8, byte: u8) {
            // TODO: Implement the random_byte functionality
        }

        pub fn draw(&mut self, vx: u8, vy: u8, nibble: u8) {
            // TODO: Implement the draw functionality
        }

        pub fn skip_if_key_pressed(&mut self, vx: u8) {
            // TODO: Implement the skip_if_key_pressed functionality
        }

        pub fn skip_if_key_not_pressed(&mut self, vx: u8) {
            // TODO: Implement the skip_if_key_not_pressed functionality
        }

        pub fn store_delay_timer(&mut self, vx: u8) {
            // TODO: Implement the store_delay_timer functionality
        }

        pub fn store_input(&mut self, vx: u8) {
            // TODO: Implement the store_input functionality
        }

        pub fn set_delay_timer(&mut self, vx: u8) {
            // TODO: Implement the set_delay_timer functionality
        }

        pub fn set_sound_timer(&mut self, vx: u8) {
            // TODO: Implement the set_sound_timer functionality
        }

        pub fn add_to_i(&mut self, vx: u8) {
            // TODO: Implement the add_to_i functionality
        }

        pub fn set_i_to_sprite_location(&mut self, vx: u8) {
            // TODO: Implement the set_i_to_sprite_location functionality
        }

        pub fn store_binary_coded_decimal(&mut self, vx: u8) {
            // TODO: Implement the store_binary_coded_decimal functionality
        }

        pub fn store_registers_in_memory(&mut self, vx: u8) {
            // TODO: Implement the store_registers_in_memory functionality
        }

        pub fn load_registers_from_memory(&mut self, vx: u8) {
            // TODO: Implement the load_registers_from_memory functionality
        }
    }
}
