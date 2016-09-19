extern crate libc;

macro_rules! ptr_not_null {
    ($x:expr) => (if $x.is_null() { panic!(concat!(stringify!($x), "is null.")) });
}
// Wrapper types wrap up a *mut. This is because we cannot guarantee, in every case,
// that the pointer will be unaliased. Despite the dupe code, no macro is used, so that
// tooling (Racer) can determine the types involved and auto-complete will work.
// The constructor (from_ptr) is unsafe, as it implies the pointer is safe to deref.
// Other members on the wrapper may expose safe dereferences to the underlying struct.

pub mod raw;
pub mod mods;

use std::borrow::Cow;
use libc::c_char;

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
    // No ref access, since core_session is opaque
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
    pub fn event_id(&self) -> fsr::event_types {
        unsafe { (*self.0).event_id }
    }
    pub fn priority(&self) -> fsr::priority {
        unsafe { (*self.0).priority }
    }
    pub fn owner(&self) -> Option<Cow<str>> {
        unsafe { fsr::ptr_to_str((*self.0).owner) }
    }
    pub fn subclass_name(&self) -> Option<Cow<str>> {
        unsafe { fsr::ptr_to_str((*self.0).subclass_name) }
    }
    pub fn body(&self) -> Option<Cow<str>> {
        unsafe { fsr::ptr_to_str((*self.0).body) }
    }
    pub fn key(&self) -> u64 {
        unsafe { (*self.0).key as u64 }
    }
    pub fn flags(&self) -> isize {
        unsafe { (*self.0).flags as isize }
    }
    pub fn header<'a>(&'a self, name: &str) -> Option<Cow<'a, str>> {
        unsafe {
            let v = fsr::event_get_header_idx(self.0, name.as_ptr() as *const c_char, -1);
            fsr::ptr_to_str(v)
        }
    }
}
pub struct EventHeader(*mut fsr::event_header);
impl EventHeader {
    pub unsafe fn from_ptr(p: *mut fsr::event_header) -> EventHeader {
        ptr_not_null!(p);
        EventHeader(p)
    }
    pub fn as_ptr(&self) -> *const fsr::event_header {
        self.0
    }
    pub fn as_mut_ptr(&mut self) -> *mut fsr::event_header {
        self.0
    }
    pub unsafe fn as_ref(&self) -> &fsr::event_header {
        &*self.0
    }
    pub unsafe fn as_mut_ref(&mut self) -> &mut fsr::event_header {
        &mut *self.0
    }
    pub fn name(&self) -> Cow<str> {
        unsafe { fsr::ptr_to_str((*self.0).name).expect("event_header.name cannot be null.") }
    }
}

pub fn event_bind<F>(id: &str, event: fsr::event_types, subclass_name: Option<&str>, callback: F)
    -> u64
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
        let mut enode = 0 as *mut u64;
        fsr::event_bind_removable(id,
                                  event,
                                  subclass_name,
                                  Some(wrap_callback::<F>),
                                  fp as *mut libc::c_void,
                                  (&mut enode) as *mut _ as *mut *mut fsr::event_node);
        enode as u64
    }
}

pub fn event_unbind(id: u64) {
    let mut enode = id as *mut u64;
    unsafe {
        fsr::event_unbind((&mut enode) as *mut _ as *mut *mut fsr::event_node);
    }
}
