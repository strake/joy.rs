//! This is a library to get joystick input.
//!
//! Example usage:
//!
//! ```
//! let mut js_dev = joy::Device::open("/dev/input/js0\0".as_bytes()).unwrap();
//! loop {
//!     for ev in &mut js_dev {
//!         use joy::Event::*;
//!         match ev {
//!             Axis(n, x) => println!("axis {} moved to {}", n, x),
//!             Button(n, true) => println!("button {} pressed", n),
//!             Button(n, false) => println!("button {} released", n),
//!         }
//!     }
//! }
//! ```

#![no_std]

extern crate libc;
#[macro_use]
extern crate syscall;

pub use native::Device;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    Button(u8, bool),
    Axis(u8, i16),
}

#[cfg(target_os = "linux")] mod native {
    use core::mem;
    use libc::{ O_NONBLOCK, O_RDONLY, EINVAL };

    const EVENT_BUTTON: u8 = 0x01;
    const EVENT_AXIS  : u8 = 0x02;
    const EVENT_INIT  : u8 = 0x80;

    #[repr(C)]
    struct Event {
        time: u32,
        value: i16,
        typ: u8,
        number: u8,
    }

    impl From<Event> for Option<super::Event> {
        #[inline] fn from(ev: Event) -> Self {
            match ev.typ & !EVENT_INIT {
                EVENT_BUTTON => Some(super::Event::Button(ev.number, ev.value != 0)),
                EVENT_AXIS   => Some(super::Event::Axis(ev.number, ev.value)),
                _            => None,
            }
        }
    }

    /// Joystick device
    ///
    /// As an `Iterator`, this returns `Some` until reading input would block, then returns None.
    pub struct Device(usize);

    impl Device {
        /// Opens the device at the given null-terminated path.
        #[inline] pub fn open(path: &[u8]) -> Result<Self, usize> {
            if Some(&0) != path.last() { return Err(EINVAL as usize) };
            let fd = unsafe { syscall!(OPEN, &path[0] as *const u8, O_NONBLOCK, O_RDONLY) } as isize;
            if fd < 0 { Err(-fd as usize) } else { Ok(Device(fd as usize)) }
        }
    }

    impl Iterator for Device {
        type Item = super::Event;
        #[inline] fn next(&mut self) -> Option<super::Event> {
            unsafe {
                let mut ev: Event = mem::uninitialized();
                if (syscall!(READ, self.0, &mut ev as *mut Event, mem::size_of::<Event>()) as isize) < 0 { None } else { From::from(ev) }
            }
        }
    }
}
