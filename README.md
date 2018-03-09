# memboot

Rust program to call the ASPEED_LPC_CTRL ioctl under Linux.

This uses the [ioctl macro from the nix crate](https://docs.rs/nix/0.9.0/nix/sys/ioctl/index.html)
 to generate code to call ioctls.
It is designed to be used on an ASPEED BMC system (embedded ARM).


## Error handling

This was my first Rust program, and the error handling when using nix was quite
confusing.

As you can see I used two methods; one borrowed from the [rust-spidev
example](https://github.com/rust-embedded/rust-spidev/blob/fbe6d067e7b44a11d9a97ce02b47ec06a90b1f37/src/spidevioctl.rs#L17)
that is linked from the nix documentation (which had to be reworked as it uses
nix 0.5, and is not compatible with nix 0.10), and the other open coded to try
to understand what is going on.

```rust
fn from_nix_error(err: ::nix::Error) -> io::Error {
    io::Error::from_raw_os_error(err.errno() as i32)
}

fn from_nix_result<T>(res: ::nix::Result<T>) -> io::Result<T> {
    match res {
        Ok(r) => Ok(r),
        Err(err) => Err(from_nix_error(err)),
    }
}
```

From what I understand, due to nix not providing a way to convert from
nix::Error to a std::Error, we cannot use ? to perform the error handling from
the ioctl macro. This is super confusing for a newbie, as the open coded
version is the same few lines of code that the Rust tutorials tell us to
replace with ?. Hopefully nix fixes this in the future.

```rust
cargo  build
   Compiling memboot v0.1.0 (file:///home/joel/dev/junk/rust/memboot)
error[E0277]: the trait bound `std::io::Error: std::convert::From<nix::Error>` is not satisfied
  --> src/main.rs:89:5
   |
89 |     try!(ioctl::lpc_ctrl_map(fd, &mut map));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<nix::Error>` is not implemented for `std::io::Error`
   |
   = help: the following implementations were found:
             <std::io::Error as std::convert::From<nix::errno::Errno>>
             <std::io::Error as std::convert::From<std::io::IntoInnerError<W>>>
             <std::io::Error as std::convert::From<std::ffi::NulError>>
             <std::io::Error as std::convert::From<std::io::ErrorKind>>
   = note: required by `std::convert::From::from`
   = note: this error originates in a macro outside of the current crate (in
           Nightly builds, run with -Z external-macro-backtrace for more info)
```

## Code size

When I cross compiled this application for ARM, the binary weighed in at 4.5MB.
There are a few blog posts that suggest things like changing the default
allocator to the system one (which doesn't seem to be documented?), and
stripping the file (which means you don't get backtraces when something goes
wrong), but I found it disappointing that none of this came out of the box.
