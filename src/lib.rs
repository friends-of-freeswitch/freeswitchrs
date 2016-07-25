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
        pub unsafe fn as_ptr(&self) -> *mut $pt {
            self.0
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
pub struct Event(*mut fsr::event);
impl Event {
    gen_from_ptr!(fsr::event, Event, Event);
}

pub fn event_bind<F>(id: &str, event: fsr::event_types, subclass_name: &str, callback: F)
    where F: Fn(Event)
{
    // TODO: Can you modify events in the callback?
    unsafe extern "C" fn wrap_callback<F>(e: *mut fsr::event)
        where F: Fn(Event)
    {
        ptr_not_null!(e);
        ptr_not_null!((*e).bind_user_data);
        let f = (*e).bind_user_data as *mut F;
        let e = Event::from_ptr(e);
        (*f)(e);
    }

    let bx = std::boxed::Box::new(callback);
    let fp = std::boxed::Box::into_raw(bx);
    let id = fsr::str_to_ptr(id);
    let subclass_name = fsr::str_to_ptr(subclass_name);
    unsafe {
        fsr::event_bind(id,
                        event,
                        subclass_name,
                        Some(wrap_callback::<F>),
                        fp as *mut libc::c_void);
    }
}
