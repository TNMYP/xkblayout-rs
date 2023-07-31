use x11::xlib::XkbGetKeyboard;
use x11::xlib::{XOpenDisplay, XkbGetState, XkbStateRec, XGetAtomName, XCloseDisplay};
use std::ptr;
use std::ffi::CStr;
use std::mem;
use std::env;
fn main() {
    unsafe{
        let display_name = env::var("DISPLAY").expect("Display not found");
        let display = XOpenDisplay(display_name.as_ptr() as * const _);
        if display == ptr::null_mut(){
           panic!("Failed to open display");
        }     

        let mut state: XkbStateRec = mem::zeroed();
        XkbGetState(display, 256 , &mut state);
        
        let keyboard = XkbGetKeyboard(display, 127, 256); // 127 XkbAllComponentsMask
                                                          // 256 XkbUseCoreKbd

        let cname = XGetAtomName(display, (*(*keyboard).names).groups[state.group as usize]);
        let cstr = CStr::from_ptr(cname);
        let name = String::from_utf8_lossy(cstr.to_bytes()).to_string();
        println!("Current keyboard layout: {}", name); 

        XCloseDisplay(display);
    }
}
