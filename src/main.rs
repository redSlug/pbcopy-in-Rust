extern crate clap;
#[macro_use]
extern crate cfg_if;
use clap::{App};
use std::io::{self, Read};

cfg_if! {
    if #[cfg(target_os="macos")] {
        extern crate objc_foundation;
        #[macro_use]
        extern crate objc;
        extern crate objc_id;
        use objc::runtime::{Object, Class};
        use objc_id::{Id};
        use objc_foundation::{NSArray, NSString};
        use objc_foundation::{INSArray, INSString};
        // Required to bring NSPasteboard into the path of the class-resolver
        #[link(name = "AppKit", kind = "framework")]
        extern "C" {} // opens up the search space to C
    }
}

cfg_if! {
    if #[cfg(target_os="linux")] {
        extern crate x11_clipboard;
        use std::time::Duration;
        use std::marker::PhantomData;
        use x11_clipboard::Atoms;
        use x11_clipboard::Clipboard as X11Clipboard;
        use x11_clipboard::xcb::xproto::Atom;
    }
}


#[cfg(target_os="linux")]
fn add_to_clipboard(data: &str) -> io::Result<String> {

    let clipboard = X11Clipboard::new()?;
    //atoms.clipboard
    return Ok(clipboard.0.store(
        ::atom(&clipboard.0.setter.atoms),
        clipboard.0.setter.atoms.utf8_string,
        data,
    )?)

}


#[cfg(target_os="macos")]
fn add_to_clipboard(data: &str) -> io::Result<String> {
    let cls = Class::get("NSPasteboard").ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Class::get(\"NSPasteboard\")"))?;
    let clipboard: *mut Object = unsafe { msg_send![cls, generalPasteboard] };
    let clipboard: Id<Object> = unsafe { Id::from_ptr(clipboard) };
    let string_array = NSArray::from_vec(vec![NSString::from_str(data)]);
    let _: usize = unsafe { msg_send![clipboard, clearContents] };
    let success: bool = unsafe { msg_send![clipboard, writeObjects:string_array] };

    println!("success={}", success);
    return Ok("great!".to_string());
}

fn main() {
    App::new("Rpbcopy")
        .version("0.1.0")
        .author("Bradley and Liuda")
        .about("pbcopy clone written in Rust")
        .get_matches();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("failed to read buffer");
    add_to_clipboard(&buffer).unwrap();
}

