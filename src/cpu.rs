pub mod cpu {
    use crate::instructions::instructions::Instruction;

    struct Chip8CPU {
        v: [u8; 16],
        i: u16,
        dt: u8,
        st: u8,
        pc: u16,
        sp: u8, //docs are vague about this one. Might have to be u16
        stack: [u16; 16],
    }

    impl Chip8CPU {
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
            // TODO: Implement the clear_display functionality
        }

        pub fn return_from_subroutine(&mut self) {
            // TODO: Implement the return_from_subroutine functionality
        }

        pub fn jump(&mut self, addr: u16) {
            // TODO: Implement the jump functionality
        }

        pub fn call(&mut self, addr: u16) {
            // TODO: Implement the call functionality
        }

        pub fn skip_if_equal(&mut self, vx: u8, byte: u8) {
            // TODO: Implement the skip_if_equal functionality
        }

        pub fn skip_if_not_equal(&mut self, vx: u8, byte: u8) {
            // TODO: Implement the skip_if_not_equal functionality
        }

        pub fn skip_if_registers_equal(&mut self, vx: u8, vy: u8) {
            // TODO: Implement the skip_if_registers_equal functionality
        }

        pub fn set_register(&mut self, vx: u8, byte: u8) {
            // TODO: Implement the set_register functionality
        }

        pub fn add(&mut self, vx: u8, byte: u8) {
            // TODO: Implement the add functionality
        }

        pub fn copy_register(&mut self, vx: u8, vy: u8) {
            // TODO: Implement the copy_register functionality
        }

        pub fn or(&mut self, vx: u8, vy: u8) {
            // TODO: Implement the or functionality
        }

        pub fn and(&mut self, vx: u8, vy: u8) {
            // TODO: Implement the and functionality
        }

        pub fn xor(&mut self, vx: u8, vy: u8) {
            // TODO: Implement the xor functionality
        }

        pub fn add_registers(&mut self, vx: u8, vy: u8) {
            // TODO: Implement the add_registers functionality
        }

        pub fn subtract(&mut self, vx: u8, vy: u8) {
            // TODO: Implement the subtract functionality
        }

        pub fn shift_right(&mut self, vx: u8) {
            // TODO: Implement the shift_right functionality
        }

        pub fn reverse_subtract(&mut self, vx: u8, vy: u8) {
            // TODO: Implement the reverse_subtract functionality
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
