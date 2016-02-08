extern crate libc;

#[macro_use]
extern crate lazy_static;

use std::sync::Arc;
use libc::c_char;
use libc::c_int;

extern "C" {
    pub fn qmlbind_application_new(argc: c_int, argv: *const *const c_char) -> *mut QMLBindApplication;
    pub fn qmlbind_application_exec(app: *mut QMLBindApplication);
    pub fn qmlbind_engine_new() -> *mut QMLBindEngine;
    pub fn qmlbind_engine_release(engine: *mut QMLBindEngine);
}

lazy_static! {
    static ref APPLICATION: *mut QMLBindApplication = unsafe { qmlbind_application_new(0 as c_int, vec![].as_ptr()) };
}

pub enum QMLBindApplication {}
unsafe impl Sync for *mut QMLBindApplication {  }

pub enum QMLBindEngine {}

struct EngineInternal {
    engine: *mut QMLBindEngine,
}

impl Drop for EngineInternal {
    fn drop(&mut self) {
        unsafe { qmlbind_engine_release(self.engine) };
    }
}

pub struct Engine {
    internal_engine: Arc<EngineInternal>,
}

impl Engine {
    pub fn new() -> Engine {
        let base_engine = unsafe { qmlbind_engine_new() };
        assert!(!base_engine.is_null());

        let internal_engine = Arc::new(EngineInternal { engine: base_engine });

        Engine { internal_engine: internal_engine }
    }

    pub fn exec(self) {
        // unsafe { qmlbind_application_exec(APPLICATION) }
    }
}
