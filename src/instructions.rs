pub mod instructions {
    pub enum Instruction {
        ClearDisplay,                    //00E0
        Return,                          //00EE
        Jump(u16),                       //1nnn
        Call(u16),                       //2nnn
        SkipIfEqual(u8, u8),             //3xkk
        SkipIfNotEqual(u8, u8),          //4xkk
        SkipIfRegistersEqual(u8, u8),    //5xy0
        SetRegister(u8, u8),             //6xkk
        Add(u8, u8),                     //7xkk
        Set(u8, u8),                     //8xy0
        Or(u8, u8),                      //8xy1
        And(u8, u8),                     //8xy2
        Xor(u8, u8),                     //8xy3
        AddRegisters(u8, u8),            //8xy4
        Subtract(u8, u8),                //8xy5
        ShiftRight(u8),                  //8xy6 (omitted y, as most modern interpreters do)
        ReverseSubtract(u8, u8),         //8xy7
        ShiftLeft(u8),                   //8xyE (omitted y)
        SkipIfRegistersNotEqual(u8, u8), //9xy0
        SetI(u16),                       //Annn
        JumpPlusV0(u16),                 //Bnnn
        RandomByte(u8, u8),              //Cxkk
        Draw(u8, u8, u8),                //Dxyn
        SkipIfKeyPressed(u8),            //Ex9E
        SkipIfKeyNotPressed(u8),         //ExA1
        SetRegisterToDelayTimer(u8),     //Fx07
        StoreInput(u8),                  //Fx0A
        SetDelayTimer(u8),               //Fx15
        SetSoundTimer(u8),               //Fx18
        AddToI(u8),                      //Fx1E
        SetIToSpriteLocation(u8),        //Fx29
        StoreBinaryCodedDecimal(u8),     //Fx33
        StoreRegisters(u8),              //Fx55
        LoadRegisters(u8),               //Fx65
    }

    pub struct InvalidInstructionError(u16);

    impl TryFrom<u16> for Instruction {
        type Error = InvalidInstructionError;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            let nibbles: (u8, u8, u8, u8) = (
                ((value >> 12) & 0xF) as u8,
                ((value >> 8) & 0xF) as u8,
                ((value >> 4) & 0xF) as u8,
                (value & 0xF) as u8,
            );

            let nnn = || value & 0x0FFF;
            let kk = || (value & 0x0FF) as u8;

            match (nibbles.0, nibbles.1, nibbles.2, nibbles.3) {
                (0x0, 0x0, 0xE, 0x0) => Ok(Instruction::ClearDisplay),
                (0x0, 0x0, 0xE, 0xE) => Ok(Instruction::Return),
                (0x1, _, _, _) => Ok(Instruction::Jump(nnn())),
                (0x2, _, _, _) => Ok(Instruction::Call(nnn())),
                (0x3, x, _, _) => Ok(Instruction::SkipIfEqual(x, kk())),
                (0x4, x, _, _) => Ok(Instruction::SkipIfNotEqual(x, kk())),
                (0x5, x, y, 0x0) => Ok(Instruction::SkipIfRegistersEqual(x, y)),
                (0x6, x, _, _) => Ok(Instruction::SetRegister(x, kk())),
                (0x7, x, _, _) => Ok(Instruction::Add(x, kk())),
                (0x8, x, y, 0x0) => Ok(Instruction::Set(x, y)),
                (0x8, x, y, 0x1) => Ok(Instruction::Or(x, y)),
                (0x8, x, y, 0x2) => Ok(Instruction::And(x, y)),
                (0x8, x, y, 0x3) => Ok(Instruction::Xor(x, y)),
                (0x8, x, y, 0x4) => Ok(Instruction::Add(x, y)),
                (0x8, x, y, 0x5) => Ok(Instruction::Subtract(x, y)),
                (0x8, x, _, 0x6) => Ok(Instruction::ShiftRight(x)),
                (0x8, x, y, 0x7) => Ok(Instruction::ReverseSubtract(x, y)),
                (0x8, x, _, 0xE) => Ok(Instruction::ShiftLeft(x)),
                (0x9, x, y, 0x0) => Ok(Instruction::SkipIfRegistersNotEqual(x, y)),
                (0xA, _, _, _) => Ok(Instruction::SetI(nnn())),
                (0xB, _, _, _) => Ok(Instruction::JumpPlusV0(nnn())),
                (0xC, x, _, _) => Ok(Instruction::RandomByte(x, kk())),
                (0xD, x, y, n) => Ok(Instruction::Draw(x, y, n)),
                (0xE, x, 0x9, 0xE) => Ok(Instruction::SkipIfKeyPressed(x)),
                (0xE, x, 0xA, 0x1) => Ok(Instruction::SkipIfKeyNotPressed(x)),
                (0xF, x, 0x0, 0x7) => Ok(Instruction::SetRegisterToDelayTimer(x)),
                (0xF, x, 0x0, 0xA) => Ok(Instruction::StoreInput(x)),
                (0xF, x, 0x1, 0x5) => Ok(Instruction::SetDelayTimer(x)),
                (0xF, x, 0x1, 0x8) => Ok(Instruction::SetSoundTimer(x)),
                (0xF, x, 0x1, 0xE) => Ok(Instruction::AddToI(x)),
                (0xF, x, 0x2, 0x9) => Ok(Instruction::SetIToSpriteLocation(x)),
                (0xF, x, 0x3, 0x3) => Ok(Instruction::StoreBinaryCodedDecimal(x)),
                (0xF, x, 0x5, 0x5) => Ok(Instruction::StoreRegisters(x)),
                (0xF, x, 0x6, 0x5) => Ok(Instruction::LoadRegisters(x)),
                _ => Err(InvalidInstructionError(value)),
            }
        }
    }
}
