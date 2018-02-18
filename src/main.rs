// memboot
// Copyright 2018 Joel Stanley <joel@jms.id.au>
// SPDX-License-Identifier:	Apache-2.0

#![allow(dead_code)]

#[macro_use] extern crate nix;
//extern crate libc;
use std::io;
use std::io::ErrorKind;
use std::os::unix::prelude::*;
use nix::Error::Sys;
use std::fs::OpenOptions;

// From https://github.com/rust-embedded/rust-spidev/blob/master/src/spidevioctl.rs
fn from_nix_error(err: ::nix::Error) -> io::Error {
    match err {
        Sys(err_no) => io::Error::from(err_no),
        _ => io::Error::new(ErrorKind::InvalidData, err)
    }
}

fn from_nix_result<T>(res: ::nix::Result<T>) -> io::Result<T> {
    match res {
        Ok(r) => Ok(r),
        Err(err) => Err(from_nix_error(err)),
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct aspeed_lpc_mapping {
    pub window_type: u8,
    pub window_id: u8,
    pub flags: u16,
    pub addr: u32,
    pub offset: u32,
    pub size: u32
}

pub const LPC_WINDOW_FLASH: u8 = 1;
pub const LPC_WINDOW_MEMORY: u8 = 2;

impl aspeed_lpc_mapping {
    pub fn new() -> aspeed_lpc_mapping {
        aspeed_lpc_mapping {
            window_type: 0,
            window_id: 0,
            flags: 0,
            addr: 0,
            offset: 0,
            size: 0
        }
    }
}

mod ioctl {
    use super::aspeed_lpc_mapping;

    pub const LPC_CTRL_MAGIC: u8 = 0xb2;
    pub const LPC_CTRL_GET_SIZE: u8 = 0;
    pub const LPC_CTRL_MAP: u8 = 1;

    ioctl!{
        write_ptr lpc_ctrl_get_size with LPC_CTRL_MAGIC, LPC_CTRL_GET_SIZE;
        aspeed_lpc_mapping
    }

    ioctl!{
        write_ptr lpc_ctrl_map with LPC_CTRL_MAGIC, LPC_CTRL_MAP;
        aspeed_lpc_mapping
    }
}

pub fn get_window_size(fd: RawFd) -> u32 {
    let mut map = aspeed_lpc_mapping::new();
    map.window_type = LPC_WINDOW_FLASH;
    match unsafe { ioctl::lpc_ctrl_get_size(fd, &mut map) } {
        Ok(val) => val,
        Err(err) => {
            panic!("Failed to run ioctl::lpc_ctrl_get_size: {:?}", err)
        },
    };
    return map.size;
}

pub fn set_flash_mapping(fd: RawFd, flash_size: u32) -> io::Result<()> {
    let mut map = aspeed_lpc_mapping::new();
    map.window_type = LPC_WINDOW_FLASH;
    map.addr = 0x0FFFFFFF & !flash_size;
    map.size = flash_size;

    try!(from_nix_result(unsafe {
        ioctl::lpc_ctrl_map(fd, &mut map)
    }));
    Ok(())
}

pub fn set_mem_mapping(fd: RawFd, mem_region: u32, mem_size: u32) -> io::Result<()> {
    let mut map = aspeed_lpc_mapping::new();
    map.window_type = LPC_WINDOW_MEMORY;
    map.addr = mem_region;
    map.size = mem_size;

    try!(from_nix_result(unsafe {
        ioctl::lpc_ctrl_map(fd, &mut map)
    }));
    Ok(())
}

fn main() {
    //let fd = nix:fcntl::open("/dev/aspeed-lpc-ctrl", OFlag::O_RDRW, nix::sys::stat::Mode::Empty);
    let path = "/dev/aspeed-lpc-ctrl";


    let device = OpenOptions::new()
                      .read(true)
                      .write(true)
                      .create(false)
                      .open(path)
                      .expect("Failed to open device");

    set_mem_mapping(device.as_raw_fd(), 0x10000, 0x1000);
}
