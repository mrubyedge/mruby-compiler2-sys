use std::{os::fd::AsRawFd, ptr::null_mut};

mod bindings {
    #![allow(nonstandard_style)]
    #![allow(unused)]
    #![allow(unnecessary_transmutes)]
    include!("./bindings.rs");
}
use bindings::{
    FILE, MRC_DUMP_OK, fdopen, mrc_ccontext, mrc_ccontext_free, mrc_ccontext_new, mrc_dump_irep,
    mrc_dump_irep_cfunc, mrc_irep, mrc_irep_free, mrc_load_string_cxt, mrc_codedump_all,
};

#[derive(Debug)]
pub struct MRubyCompiler2Error {
    details: String,
}

impl MRubyCompiler2Error {
    fn new(msg: &str) -> MRubyCompiler2Error {
        MRubyCompiler2Error {
            details: msg.to_string(),
        }
    }

    fn from_error<E: std::error::Error>(msg: &str, err: E) -> MRubyCompiler2Error {
        MRubyCompiler2Error {
            details: format!("{}: {}", msg, err.to_string()),
        }
    }
}

impl std::fmt::Display for MRubyCompiler2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for MRubyCompiler2Error {}

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

    pub unsafe fn compile(&mut self, code: &str) -> Result<Vec<u8>, MRubyCompiler2Error> {
        unsafe {
            let c_code = std::ffi::CString::new(code)
                .map_err(|_| MRubyCompiler2Error::new("Code includes null bytes"))?;
            let mut ptr = c_code.as_ptr() as *const u8;
            let irep =
                mrc_load_string_cxt(self.c, &mut ptr as *mut *const u8, c_code.as_bytes().len());

            if irep.is_null() {
                return Err(MRubyCompiler2Error::new("Failed to compile code"));
            }

            // Set dummy capacity, deduced from code length
            // And leak for safety rather than memory efficiency
            let bin: &'static mut [u8] = Vec::with_capacity(code.len() * 2).leak();
            let bin_ptr = bin.as_mut_ptr();
            let mut bin_size: usize = 0;

            let result = mrc_dump_irep(
                self.c,
                irep as *mut mrc_irep,
                0,
                &bin_ptr as *const *mut u8 as *mut *mut u8,
                &mut bin_size as *mut usize,
            );
            mrc_irep_free(self.c, irep as *mut mrc_irep);
            if result as u32 != MRC_DUMP_OK {
                return Err(MRubyCompiler2Error::new("Failed to dump irep binary"));
            }

            let newvec = Vec::from_raw_parts(bin_ptr, bin_size, bin_size);
            Ok(newvec)
        }
    }

    pub unsafe fn dump_bytecode(&mut self, code: &str) -> Result<(), MRubyCompiler2Error> {
        unsafe {
            let c_code = std::ffi::CString::new(code)
                .map_err(|_| MRubyCompiler2Error::new("Code includes null bytes"))?;
            let mut ptr = c_code.as_ptr() as *const u8;
            let irep =
                mrc_load_string_cxt(self.c, &mut ptr as *mut *const u8, c_code.as_bytes().len());

            if irep.is_null() {
                return Err(MRubyCompiler2Error::new("Failed to compile code"));
            }

            mrc_codedump_all(
                self.c,
                irep as *mut mrc_irep,
            );
            mrc_irep_free(self.c, irep as *mut mrc_irep);
            Ok(())
        }
    }

    pub unsafe fn compile_to_file(
        &mut self,
        code: &str,
        path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bin = unsafe { self.compile(code) }?;
        let mut out = std::fs::File::create(path)?;
        std::io::Write::write_all(&mut out, &bin)?;
        Ok(())
    }

    pub unsafe fn compile_to_c_function(
        &mut self,
        code: &str,
        initname: &str,
        path: &std::path::Path,
    ) -> Result<(), MRubyCompiler2Error> {
        let out = std::fs::File::create(path)
            .map_err(|e| MRubyCompiler2Error::from_error("Failed to create file", e))?;

        unsafe {
            let c_code = std::ffi::CString::new(code)
                .map_err(|e| MRubyCompiler2Error::from_error("Code includes null bytes", e))?;
            let mut ptr = c_code.as_ptr() as *const u8;
            let irep =
                mrc_load_string_cxt(self.c, &mut ptr as *mut *const u8, c_code.as_bytes().len());

            if irep.is_null() {
                return Err(MRubyCompiler2Error::new("Failed to compile code"));
            }
            let fd = out.as_raw_fd();
            let mode_str = std::ffi::CString::new("w").unwrap();
            let fp = fdopen(fd, mode_str.as_ptr());
            std::mem::forget(out);

            let initname = std::ffi::CString::new(initname)
                .map_err(|e| MRubyCompiler2Error::from_error("Initname includes null bytes", e))?;

            let result = mrc_dump_irep_cfunc(self.c, irep, 0, fp as *mut FILE, initname.as_ptr());
            mrc_irep_free(self.c, irep as *mut mrc_irep);
            if result as u32 != MRC_DUMP_OK {
                return Err(MRubyCompiler2Error::new("Failed to dump irep binary"));
            }
            Ok(())
        }
    }
}

impl Drop for MRubyCompiler2Context {
    fn drop(&mut self) {
        unsafe {
            mrc_ccontext_free(self.c);
        }
    }
}
