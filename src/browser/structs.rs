
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

    fn get_abi(&self) -> Self::Abi {
        *self
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self as *mut Self::Abi
    }
}
