extern crate libc;

macro_rules! ptr_not_null {
    ($x:expr) => (if $x.is_null() { panic!(concat!(stringify!($x), "is null.")) });
}
macro_rules! gen_from_ptr {
    ($pt:ty, $it:ty, $itx:expr) => (
        pub unsafe fn from_ptr(p: *mut $pt) -> $it {
            ptr_not_null!(p);
            $itx(p)
        }
    )
}

pub mod raw;
pub mod mods;

use raw as fsr;

pub type Status = Result<(), fsr::status>;
pub trait StatusImpl {
    fn to_raw(&self) -> fsr::status;
}
impl StatusImpl for Status {
    fn to_raw(&self) -> fsr::status {
        self.err().unwrap_or(fsr::status::SUCCESS)
    }
}

pub struct CoreSession(*mut fsr::core_session);
impl CoreSession {
    gen_from_ptr!(fsr::core_session, CoreSession, CoreSession);
    
}
