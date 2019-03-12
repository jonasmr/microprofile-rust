extern crate cc;
fn main(){
	if cfg!(windows)
	{
		cc::Build::new()
			.file("src/microprofile/microprofile.cpp")
			.cpp(true)
			.define("MICROPROFILE_GPU_TIMERS", "0")
			.define("MICROPROFILE_DYNAMIC_INSTRUMENT", "1")
			.include("src/microprofile/distorm/include")
			.compile("microprofile");

		cc::Build::new()
			.file("src/microprofile/distorm/src/mnemonics.c")
			.file("src/microprofile/distorm/src/textdefs.c")
			.file("src/microprofile/distorm/src/prefix.c")
			.file("src/microprofile/distorm/src/operands.c")
			.file("src/microprofile/distorm/src/insts.c")
			.file("src/microprofile/distorm/src/instructions.c")
			.file("src/microprofile/distorm/src/distorm.c")
			.file("src/microprofile/distorm/src/decoder.c")
			.cpp(false)
			.opt_level(2)
			.define("DISTORM_STATIC", "1")
			.define("SUPPORT_64BIT_OFFSET", "1")
			.compile("distorm");
	}
	else
	{
		cc::Build::new()
			.file("src/microprofile/microprofile.cpp")
			.file("src/microprofile/patch_osx.s")
			.cpp(true)
			.define("MICROPROFILE_GPU_TIMERS", "0")
			.define("MICROPROFILE_DYNAMIC_INSTRUMENT", "1")
			.flag("-std=c++11")
			.flag("-Wno-unused-parameter")
			.include("src/microprofile/distorm/include")
			.compile("microprofile");

		cc::Build::new()
			.file("src/microprofile/distorm/src/mnemonics.c")
			.file("src/microprofile/distorm/src/textdefs.c")
			.file("src/microprofile/distorm/src/prefix.c")
			.file("src/microprofile/distorm/src/operands.c")
			.file("src/microprofile/distorm/src/insts.c")
			.file("src/microprofile/distorm/src/instructions.c")
			.file("src/microprofile/distorm/src/distorm.c")
			.file("src/microprofile/distorm/src/decoder.c")
			.cpp(false)
			.opt_level(2)
			.define("DISTORM_STATIC", "1")
			.define("SUPPORT_64BIT_OFFSET", "1")
			.compile("distorm");
	}	
}