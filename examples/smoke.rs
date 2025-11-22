extern crate mruby_compiler2_sys;
use mruby_compiler2_sys::MRubyCompiler2Context;

fn main() {
    unsafe {
        let mut cxt = MRubyCompiler2Context::new();
        let bin = cxt.compile("puts \"Hello, mruby-compiler2!\"");
        println!("Compiled bytecode size: {}", bin.len());

        let out = std::fs::File::create("examples/out.mrb").unwrap();
        std::io::Write::write_all(&mut &out, &bin).unwrap();
    }
}