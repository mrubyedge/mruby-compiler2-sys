unsafe extern "C" {
    pub fn mrc_ccontext_new(mrb: *mut ::std::os::raw::c_void) -> *mut mrc_ccontext;
}
unsafe extern "C" {
    pub fn mrc_ccontext_filename(
        c: *mut mrc_ccontext,
        s: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn mrc_ccontext_free(c: *mut mrc_ccontext);
}
unsafe extern "C" {
    pub fn mrc_irep_free(c: *mut mrc_ccontext, irep: *mut mrc_irep);
}
unsafe extern "C" {
    pub fn mrc_load_string_cxt(
        c: *mut mrc_ccontext,
        source: *mut *const u8,
        length: usize,
    ) -> *mut mrc_irep;
}
unsafe extern "C" {
    pub fn mrc_dump_irep(
        c: *mut mrc_ccontext,
        irep: *const mrc_irep,
        flags: u8,
        bin: *mut *mut u8,
        bin_size: *mut usize,
    ) -> ::std::os::raw::c_int;
}
