use std::ptr::null_mut;

mod bindings {
    #![allow(nonstandard_style)]
    #![allow(unused)]
    #![allow(unnecessary_transmutes)]
    include!("./bindings.rs");
}
use bindings::{
    mrc_ccontext,
    mrc_ccontext_new,
    mrc_load_string_cxt,
    mrc_irep,
    mrc_dump_irep,
    MRC_DUMP_OK,
    mrc_irep_free,
    mrc_ccontext_free,
};

pub struct MRubyCompiler2Context {
    c: *mut mrc_ccontext,
}

impl MRubyCompiler2Context {
    pub unsafe fn new() -> Self {
        unsafe {
            let ccontext = mrc_ccontext_new(null_mut());
            MRubyCompiler2Context { c: ccontext }
        }
    }

    pub unsafe fn compile(&mut self, code: &str) -> Vec<u8> {
        unsafe {
            let c_code = std::ffi::CString::new(code).unwrap();
            let mut ptr = c_code.as_ptr() as *const u8;
            let irep = mrc_load_string_cxt(
                self.c,
                &mut ptr as *mut *const u8,
                c_code.as_bytes().len()
            );

            if irep.is_null() {
                panic!("Failed to compile code");
            }

            let mut bin: Vec<u8> = Vec::new();
            let bin_ptr = bin.as_mut_ptr();
            let mut bin_size: usize = 0;

            let result = mrc_dump_irep(
                self.c,
                irep as *mut mrc_irep,
                0,
                &bin_ptr as *const *mut u8 as *mut *mut u8,
                &mut bin_size as *mut usize,
            );
            if result as u32 != MRC_DUMP_OK {
                panic!("Failed to dump irep");
            }
            mrc_irep_free(self.c, irep as *mut mrc_irep);
            mrc_ccontext_free(self.c);

            dbg!(bin_size);
            let newvec = Vec::from_raw_parts(bin_ptr, bin_size, bin_size);
            newvec
        }
    }
}