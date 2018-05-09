extern crate clap;
extern crate objc_foundation;
#[macro_use]
extern crate objc;
extern crate objc_id;

use objc::runtime::{Object, Class};
use clap::{App};
use std::io::{self, Read};
use objc_id::{Id, Owned};

use objc_foundation::{NSArray, NSDictionary, NSString, NSObject};
use objc_foundation::{INSArray, INSString, INSObject};
use std::error::Error;


// Required to bring NSPasteboard into the path of the class-resolver
#[link(name = "AppKit", kind = "framework")]
extern "C" {} // opens up the search space to C


#[cfg(target_os="macos")]
fn add_to_clipboard(data: &str) -> io::Result<String> {
    let cls = Class::get("NSPasteboard").ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Class::get(\"NSPasteboard\")"))?;
    let pasteboard: *mut Object = unsafe { msg_send![cls, generalPasteboard] };

    // NSPasteboard *pasteboard = [NSPasteboard generalPasteboard];

    let pasteboard: Id<Object> = unsafe { Id::from_ptr(pasteboard) };

    let string_array = NSArray::from_vec(vec![NSString::from_str(data)]);
    let _: usize = unsafe { msg_send![pasteboard, clearContents] };
    let success: bool = unsafe { msg_send![pasteboard, writeObjects:string_array] };

    println!("success={}", success);
    return Ok("great!".to_string());
}

fn main() {
    let matches = App::new("Rpbcopy")
        .version("0.1.0")
        .author("Bradley and Liuda")
        .about("pbcopy clone written in Rust")
        .get_matches();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("failed to read buffer");
    add_to_clipboard(&buffer).unwrap();
}


