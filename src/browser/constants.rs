use super::structs::VerbId;
use com::sys::HRESULT;

pub const OLEIVERB_SHOW: VerbId = -1;
pub const OLEIVERB_OPEN: VerbId = -2;
pub const OLEIVERB_HIDE: VerbId = -3;
pub const OLEIVERB_UIACTIVATE: VerbId = -4;
pub const OLEIVERB_INPLACEACTIVATE: VerbId = -5;

pub const OLEOBJ_S_INVALIDVERB: HRESULT = 262528;

pub const E_NOTIMPL: HRESULT = -2147467263;
