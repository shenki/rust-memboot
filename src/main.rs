// memboot
// Copyright 2018 Joel Stanley <joel@jms.id.au>
// SPDX-License-Identifier:	Apache-2.0

#[macro_use] extern crate nix;
//extern crate libc;
//use std::io;

pub const LPC_CTRL_MAGIC: u8 = 0xb2;
pub const LPC_CTRL_GET_SIZE: u8 = 0;
pub const LPC_CTRL_MAP: u8 = 1;

#[allow(non_camel_case_types)]
pub struct aspeed_lpc_ctrl_mapping {
    pub window_type: u8,
    pub window_id: u8,
    pub flags: u16,
    pub addr: u32,
    pub offset: u32,
    pub size: u32
}

ioctl!{
    write_buf lpc_ctrl_get_size with LPC_CTRL_MAGIC, LPC_CTRL_GET_SIZE;
    aspeed_lpc_ctrl_mapping
}

ioctl!{
    write_buf lpc_ctrl_map with LPC_CTRL_MAGIC, LPC_CTRL_MAP;
    aspeed_lpc_ctrl_mapping
}

fn main() {
    println!("Hello, world!");
}
