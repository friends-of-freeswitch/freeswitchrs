extern crate libc;

macro_rules! ptr_not_null {
    ($x:expr) => (if $x.is_null() { panic!(concat!(stringify!($x), "is null.")) });
}
macro_rules! gen_wrap_impl {
    ($pt:ty, $it:ty, $itx:expr, $tt:tt) => (
        pub unsafe fn from_ptr(p: *mut $pt) -> $it {
            ptr_not_null!(p);
            $itx(&mut *p)
        }
        pub fn as_ptr(&mut self) -> *mut $pt {
            self.0 as *mut $pt
        }
        pub fn into_mut_ref(self) -> &'a mut $pt {
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
    pub unsafe fn from_ptr(p: *mut fsr::core_session) -> CoreSession {
        ptr_not_null!(p);
        CoreSession(p)
    }
    pub fn as_ptr(&self) -> *const fsr::core_session {
        self.0 
    }
    pub fn as_mut_ptr(&mut self) -> *mut fsr::core_session {
        self.0
    }
    pub unsafe fn as_ref(&self) -> &fsr::core_session {
        &*self.0
    }
    pub unsafe fn as_mut_ref(&mut self) -> &mut fsr::core_session {
        &mut *self.0
    }
}
pub struct Event(*mut fsr::event);
impl Event {
    pub unsafe fn from_ptr(p: *mut fsr::event) -> Event {
        ptr_not_null!(p);
        Event(p)
    }
    pub fn as_ptr(&self) -> *const fsr::event {
        self.0
    }
    pub fn as_mut_ptr(&mut self) -> *mut fsr::event {
        self.0
    }
    pub unsafe fn as_ref(&self) -> &fsr::event {
        &*self.0
    }
    pub unsafe fn as_mut_ref(&mut self) -> &mut fsr::event {
        &mut *self.0
    }
}

pub fn event_bind<F>(id: &str, event: fsr::event_types, subclass_name: Option<&str>, callback: F)
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
    let subclass_name = subclass_name.map_or(std::ptr::null(), |x| fsr::str_to_ptr(x));
    unsafe {
        fsr::event_bind(id,
                        event,
                        subclass_name,
                        Some(wrap_callback::<F>),
                        fp as *mut libc::c_void);
    }
}
