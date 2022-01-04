#![windows_subsystem = "windows"]
use windows_dll::dll;

#[dll("ntdll.dll")]
extern "system" {
    #[allow(non_snake_case)]
    fn NtSetTimerResolution(DesiredTime: u32, SetResolution: u8, PreviousTime: *mut u32) -> i32;

    #[allow(non_snake_case)]
    fn NtQueryTimerResolution(MaximumTime: *mut u32, MinimumTime: *mut u32, CurrentTime: *mut u32) -> i32;
}
enum CVoid {}
#[dll("kernel32.dll")]
extern "system" {
    #[allow(non_snake_case)]
    fn GetCurrentProcess() -> *mut CVoid;

    #[allow(non_snake_case)]
    fn SetPriorityClass(
        hProcess: *mut CVoid,
        dwPriorityClass: u32,
    ) -> i32;
}

fn main() {
    let mut min = 0u32;
    let mut max = 0u32;
    let mut cur = 0u32;

    unsafe {
        // https://www.pinvoke.net/search.aspx?search=NtQueryTimerResolution&namespace=[All]
        if NtQueryTimerResolution(&mut max, &mut min, &mut cur) == 0 && cfg!(debug_assertions) {
            println!("MaximumResolution: {:#}", max);
            println!("MinimumResolution: {:#}", min);
            println!("CurrentResolution: {:#}", cur);
        }

        #[cfg(debug_assertions)]
        println!("NtSetTimerResolution: {:#}", min);

        if NtSetTimerResolution( min, 1, &mut cur ) == 0 && cfg!(debug_assertions) {
            println!("CurrentResolution: {:#}", cur);
        }

        // https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass
        SetPriorityClass(GetCurrentProcess(), 0x00100000) // PROCESS_MODE_BACKGROUND_BEGIN
    };

    std::thread::park();
}
