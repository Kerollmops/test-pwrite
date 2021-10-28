//! This program reproduces an error 22 (invalid argument) that we get when using LMDB.
//! Internally LMDb was trying to append a 3.3GiB buffer at the end of a 16GiB file by using pwrite
//! and it fail returning EINVAL.
//!
//! We are able to reproduce the "bug" on a macbook pro M1.

use libc::pwrite;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

fn main() {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .open("big-file")
        .unwrap();

    let offset = 16818044928;
    let wsize = 2147483648; // limit is 2147483648-1
    let content = vec![42; wsize];
    let fd = file.as_raw_fd();

    println!("try appending {} bytes", wsize);
    let wres = unsafe { pwrite(fd, content.as_slice().as_ptr() as *const _, wsize, offset) };

    if wres != wsize as isize {
        if wres < 0 {
            let code = errno::errno();
            println!("error code is {}", code);
            return;
        }
        println!("didn't write everything in one shot");
        return;
    }

    println!("Everything worked fine");
}
