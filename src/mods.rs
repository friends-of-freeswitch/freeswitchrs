use super::*;
use super::raw as fsr;
use std::ffi::CString;
use super::raw::module_interface_name as IntName;
use libc::c_char;
use libc::c_void;

pub enum Stream { } // Temp until wrap stream
pub type ApiFunc = fn(String, Option<&CoreSession>, Stream);
pub type ApiRawFunc = unsafe extern "C" fn(cmd: *const c_char,
                                           session: *mut fsr::core_session,
                                           stream: *mut fsr::stream_handle)
                                           -> fsr::status;
pub type AppRawFunc = unsafe extern "C" fn(session: *mut fsr::core_session,
                                           data: *const c_char);

pub struct ModInterface(*mut fsr::loadable_module_interface);
impl ModInterface {
    pub unsafe fn from_ptr(p: *mut fsr::loadable_module_interface) -> ModInterface {
        ptr_not_null!(p);
        ModInterface(&mut *p)
    }
    pub fn as_ptr(&mut self) -> *const fsr::loadable_module_interface {
        self.0
    }
    pub fn as_mut_ptr(&mut self) -> *mut fsr::loadable_module_interface {
        self.0
    }
    pub unsafe fn as_ref(&self) -> &fsr::loadable_module_interface {
        &*self.0
    }
    pub unsafe fn as_mut_ref(&self) -> &mut fsr::loadable_module_interface {
        &mut *self.0
    }

    unsafe fn create_int(&self, iname: IntName) -> *mut c_void {
        fsr::loadable_module_create_interface((*self).0, iname)
    }

    pub fn add_raw_api(&self, name: &str, desc: &str, syntax: &str, func: ApiRawFunc) {
        let name = fsr::str_to_ptr(name);
        let desc = fsr::str_to_ptr(desc);
        let syntax = fsr::str_to_ptr(syntax);
        unsafe {
            let ai = self.create_int(IntName::API_INTERFACE) as *mut fsr::api_interface;
            ptr_not_null!(ai);
            (*ai).interface_name = name;
            (*ai).desc = desc;
            (*ai).syntax = syntax;
            (*ai).function = Some(func);
        }
    }

    pub fn add_raw_application(&self, name: &str, long_desc: &str, short_desc: &str, syntax: &str,
                               func: AppRawFunc, flags: fsr::application_flag_enum) {
        let name = fsr::str_to_ptr(name);
        let long_desc = fsr::str_to_ptr(long_desc);
        let short_desc = fsr::str_to_ptr(short_desc);
        let syntax = fsr::str_to_ptr(syntax);
        unsafe {
            let ai = self.create_int(IntName::APPLICATION_INTERFACE) as *mut fsr::application_interface;
            ptr_not_null!(ai);
            (*ai).interface_name = name;
            (*ai).long_desc = long_desc;
            (*ai).short_desc = short_desc;
            (*ai).syntax = syntax;
            (*ai).flags = flags as u32;
            (*ai).application_function = Some(func);
        }
    }

    // Doing safe versions is a pain. Macros are ugly. Need to use libffi or similar
    // to dynamically create thunks that'll wrap the safe functions.
    // fn add_api(&mut self, name: &str, desc: &str, syntax: &str, func: ApiFunc) {
    //     self.add_raw_api(name, desc, syntax, TODO_ALLOC_TRAMPOLINE(func));
    // }
}

// Module Loading/Definition 

pub struct ModDefinition {
    pub name: &'static str,
    pub load: fn(&ModInterface) -> Status,
    pub shutdown: Option<fn() -> Status>,
    pub runtime: Option<fn() -> Status>,
}

pub unsafe fn wrap_mod_load(mod_def: &ModDefinition,
                            mod_int: *mut *mut fsr::loadable_module_interface,
                            mem_pool: *mut fsr::memory_pool)
                            -> fsr::status {
    // Name should be a constant [u8], but we'd need some macro or something
    // to ensure null termination. Leaking the name here shouldn't matter.
    // CString's into_raw pointer is not free()'able fwiw
    let name = CString::new(mod_def.name).unwrap().into_raw();
    *mod_int = fsr::loadable_module_create_module_interface(mem_pool, name);
    if (*mod_int).is_null() {
        return fsr::status::MEMERR;
    }
    let mi = &ModInterface::from_ptr(*mod_int);
    (mod_def.load)(mi).to_raw()
}

pub fn wrap_mod_runtime(mod_def: &ModDefinition)
                         -> fsr::status {
    if let Some(func) = mod_def.runtime {
        func().to_raw()
    } else {
        fsr::status::SUCCESS
    }
}

pub fn wrap_mod_shutdown(mod_def: &ModDefinition)
                         -> fsr::status {
    if let Some(func) = mod_def.shutdown {
        func().to_raw()
    } else {
        fsr::status::SUCCESS
    }
}

/// This macro needs to be called once in the module. It will generate the definitions
/// required to be loaded by FreeSWITCH. FS requires the exported table to have a name
/// of <filename>_module_interface. If your mod is called mod_foo, then the first param
/// to this macro must be mod_foo_module_interface.
/// The second parameter must be a static (global) ModDefinition.
#[macro_export]
macro_rules! freeswitch_export_mod {
    ($table:ident, $def:ident) => (
#[no_mangle]
pub unsafe extern "C" fn _mod_load(mod_int: *mut *mut fsr::loadable_module_interface,
                                        mem_pool: *mut fsr::memory_pool)
                                        -> fsr::status {
    if let Some(_) = $def.runtime {
        $table.runtime = Some(_mod_runtime);
    }
    if let Some(_) = $def.shutdown {
        $table.shutdown = Some(_mod_shutdown);
    }
    wrap_mod_load(&$def, mod_int, mem_pool)
}

#[no_mangle]
pub extern "C" fn _mod_runtime() -> fsr::status {
    wrap_mod_runtime(&$def)
}

#[no_mangle]
pub extern "C" fn _mod_shutdown() -> fsr::status {
    wrap_mod_shutdown(&$def)
}

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static mut $table: fsr::loadable_module_function_table =
    fsr::loadable_module_function_table {
        api_version: 5,
        load: _mod_load,
        shutdown: None,
        runtime: None,
        flags: fsr::module_flag_enum::NONE as u32,
    };
);}
