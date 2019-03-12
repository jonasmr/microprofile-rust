use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;

#[link(name="microprofile")]
extern {
	fn MicroProfileInit();
	fn MicroProfileShutdown();
	pub fn MicroProfileGetToken(sGroup : *const c_char, sName : *const c_char, nColor : u32, TokenType : u32) -> u64;
	pub fn MicroProfileEnter(Token : u64);
	pub fn MicroProfileLeave();
	fn MicroProfileFlip(GpuContext : *const c_void);
	fn MicroProfileStartAutoFlip(nMs : u32);
	fn MicroProfileStopAutoFlip();
	fn MicroProfileDumpFile(pHtml : *const c_char, pCsv : *const c_char, fCpuSpike : f32, fGpuSpike : f32);
	fn MicroProfileDumpFileImmediately(pHtml : *const c_char, pCsv : *const c_char, pGpuContext : *const c_void);
	fn MicroProfileSetEnableAllGroups(enable : i32);
	fn MicroProfileOnThreadCreate(pThreadName: *const c_char);
	fn MicroProfileOnThreadExit();
}

pub fn init()
{
	unsafe
	{
		MicroProfileInit();
	}
}

pub fn shutdown()
{
	unsafe
	{
		MicroProfileShutdown();
	}
}

pub fn flip()
{
	unsafe
	{
		MicroProfileFlip(std::ptr::null());
	}
}

pub fn start_auto_flip(ms : u32)
{
	unsafe
	{
		MicroProfileStartAutoFlip(ms);
	}
}

pub fn stop_auto_flip()
{
	unsafe
	{
		MicroProfileStopAutoFlip();
	}
}

pub fn get_token(group : String, name : String, color : u32) -> u64
{
	let group = CString::new(group).unwrap();
	let name = CString::new(name).unwrap();
	unsafe
	{
		MicroProfileGetToken(group.as_ptr(), name.as_ptr(), color, 0)
	}
}

pub fn enter(token:u64)
{
	unsafe
	{
		MicroProfileEnter(token);
	}
}

pub fn leave()
{
	unsafe
	{
		MicroProfileLeave();
	}
}

pub fn dump_file(html : String, csv : String, cpu_spike : f32, gpu_spike : f32)
{
	let html_len = html.len();
	let csv_len = csv.len();
	let html = CString::new(html).unwrap();
	let csv = CString::new(csv).unwrap();

	unsafe
	{
		let html_bytes = if html_len > 0 { html.as_ptr() } else { std::ptr::null() };
		let csv_bytes = if csv_len > 0 { csv.as_ptr() } else { std::ptr::null() };
		MicroProfileDumpFile(html_bytes, csv_bytes, cpu_spike, gpu_spike);
	}
}

pub fn dump_file_immediately(html : &str, csv : &str)
{
	let html_len = html.len();
	let csv_len = csv.len();
	let html = CString::new(html).unwrap();
	let csv = CString::new(csv).unwrap();

	unsafe
	{
		let html_bytes = if html_len > 0 { html.as_ptr() } else { std::ptr::null() };
		let csv_bytes = if csv_len > 0 { csv.as_ptr() } else { std::ptr::null() };
		MicroProfileDumpFileImmediately(html_bytes, csv_bytes, std::ptr::null());
	}
}

pub fn set_enable_all_groups(enable : bool)
{
	unsafe
	{
		let enable : i32 = if enable { 1 } else { 0 };
		MicroProfileSetEnableAllGroups(enable);
	}
}

pub fn on_thread_create(thread_name : &str)
{
	let thread_name = CString::new(thread_name).unwrap();
	unsafe
	{
		MicroProfileOnThreadCreate(thread_name.as_ptr());
	}
}

pub fn on_thread_exit()
{
	unsafe
	{
		MicroProfileOnThreadExit();
	}
}

pub struct MicroProfileDroppable
{
}

impl Drop for MicroProfileDroppable
{
	fn drop(&mut self)
	{
		unsafe
		{
			MicroProfileLeave();
		}
	}
}

#[macro_export]
macro_rules! scope {
	($group_name:expr, $scope_name:expr, $color:expr) => {
		{
			static mut TOKEN : u64 = 0;
			static INIT: std::sync::Once = std::sync::Once::new();
			unsafe{
				INIT.call_once(||{
					let group = std::ffi::CString::new($group_name).unwrap();
					let scope = std::ffi::CString::new($scope_name).unwrap();
					TOKEN = $crate::MicroProfileGetToken(group.as_ptr(), scope.as_ptr(), $color, 0);;
				});
				$crate::MicroProfileEnter(TOKEN);	
			}
		}
		let _scope = $crate::MicroProfileDroppable{};
	};
	($group_name:expr, $scope_name:expr) => {
		scope!($group_name, $scope_name, 0);
	}
}


#[cfg(test)]
mod tests {
	fn run_test()
	{
		crate::scope!("foo", "fisk");
		for _ in 0..10
		{
			crate::scope!("foo", "bar", 0xff00ff00);
			crate::scope!("foo", "fest");
			for _ in 0..10
			{
				crate::scope!("foo", "baz");
				std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 /100));
			}
		}
	}
    #[test]
	fn basic() {
		crate::init();
		crate::set_enable_all_groups(true);
		crate::start_auto_flip(20);
		run_test();
		crate::dump_file_immediately("foo.html", "");
		crate::dump_file_immediately("", "foo.csv");
		crate::stop_auto_flip();
		crate::shutdown();
	}
}
