
// Move to types?
pub type VerbId = i32;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DataViewAspect {
    Content = 0x01,
    Thumbnail = 0x02,
    Icon = 0x04,
    DocPrin = 0x08,
}

unsafe impl com::AbiTransferable for DataViewAspect {
    type Abi = Self;
    const VAR_TYPE: com::TypeDescVarType = com::TypeDescVarType::Ui4;

    fn get_abi(&self) -> Self::Abi {
        *self
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}


#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VariantBool {
    True = 0xFFFF,
    False = 0x0000,
}

impl From<bool> for VariantBool {
    fn from(var: bool) -> Self {
        match var {
            true => VariantBool::True,
            false => VariantBool::False,
        }
    }
}

impl From<VariantBool> for bool {
    fn from(var: VariantBool) -> Self {
        match var as u16 {
            0xFFFF => true,
            0x0000 => false,
            _ => panic!("unknown value of VariantBool encounterd")
        }
    }
}

unsafe impl com::AbiTransferable for VariantBool {
    type Abi = Self;
    const VAR_TYPE: com::TypeDescVarType = com::TypeDescVarType::Ui2;

    fn get_abi(&self) -> Self::Abi {
        *self
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}
