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
	fn MicroProfileTick() -> u64;
	fn MicroProfileTicksPerSecondCpu() -> i64;


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

pub fn tick() -> u64
{
	unsafe
	{
		MicroProfileTick()
	}	
}

pub fn ticks_per_second_cpu() -> u64
{
	unsafe
	{
		MicroProfileTicksPerSecondCpu() as u64
	}
}

pub fn ticks_to_seconds(ticks : u64) -> f32
{
	let per_second = ticks_per_second_cpu();
	let seconds : f32 = ticks as f32 / per_second as f32;
	seconds
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

#[cfg(not(feature = "disabled"))]
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
		$crate::scope!($group_name, $scope_name, 0);
	}
}

#[cfg(feature = "disabled")]
#[macro_export] 
macro_rules! scope {
	($group_name:expr, $scope_name:expr, $color:expr) => {
	};
	($group_name:expr, $scope_name:expr) => {
	}
}



#[cfg(not(feature = "disabled"))]
#[macro_export] 
macro_rules! init { () => { $crate::init(); } }
#[cfg(feature = "disabled")]
#[macro_export]
macro_rules! init { () => { } }

#[cfg(not(feature = "disabled"))]
#[macro_export]
macro_rules! flip { () => { $crate::flip(); } }
#[cfg(feature = "disabled")]
#[macro_export]
macro_rules! flip { () => { } }

#[cfg(not(feature = "disabled"))]
#[macro_export]
macro_rules! shutdown { () => { $crate::shutdown(); } }
#[cfg(feature = "disabled")]
#[macro_export]
macro_rules! shutdown { () => { } }

#[cfg(not(feature = "disabled"))]
#[macro_export]
macro_rules! start_auto_flip { ($delay_in_ms:expr) => { $crate::start_auto_flip($delay_in_ms); } }
#[cfg(feature = "disabled")]
#[macro_export]
macro_rules! start_auto_flip { ($delay_in_ms:expr) => { } }

#[cfg(not(feature = "disabled"))]
#[macro_export]
macro_rules! stop_auto_flip { () => { $crate::stop_auto_flip(); } }
#[cfg(feature = "disabled")]
#[macro_export]
macro_rules! stop_auto_flip { () => { } }

#[cfg(not(feature = "disabled"))]
#[macro_export]
macro_rules! on_thread_exit { () => { $crate::on_thread_exit(); } }
#[cfg(feature = "disabled")]
#[macro_export]
macro_rules! on_thread_exit { () => { } }


#[cfg(not(feature = "disabled"))]
#[macro_export]
macro_rules! on_thread_create { ($name:expr) => { $crate::on_thread_create($name); } }
#[cfg(feature = "disabled")]
#[macro_export]
macro_rules! on_thread_create { ($name:expr) => { } }


#[cfg(not(feature = "disabled"))]
#[macro_export]
macro_rules! set_enable_all_groups { ($enabled:expr) => { $crate::set_enable_all_groups($enabled); } }
#[cfg(feature = "disabled")]
#[macro_export]
macro_rules! set_enable_all_groups { ($enabled:expr) => { } }


#[cfg(not(feature = "disabled"))]
#[macro_export]
macro_rules! dump_file { ($html:expr, $csv:expr, $cpu_spike:expr, $gpu_spike:expr) => { $crate::dump_file($html, $csv, $cpu_spike, $gpu_spike); } }
#[cfg(feature = "disabled")]
#[macro_export]
macro_rules! dump_file { ($html:expr, $csv:expr, $cpu_spike:expr, $gpu_spike:expr) => { } }


#[cfg(not(feature = "disabled"))]
#[macro_export]
macro_rules! dump_file_immediately { ($html:expr, $csv:expr) => { $crate::dump_file_immediately($html, $csv); } }
#[cfg(feature = "disabled")]
#[macro_export]
macro_rules! dump_file_immediately { ($html:expr, $csv:expr) => { } }


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
		crate::init!();
		crate::set_enable_all_groups!(true);
		crate::start_auto_flip!(20);
		run_test();
		crate::dump_file_immediately!("foo.html", "");
		crate::dump_file_immediately!("", "foo.csv");
		crate::stop_auto_flip!();
		crate::shutdown!();
	}
}
