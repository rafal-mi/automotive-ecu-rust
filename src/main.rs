
use std::ffi::CString;
use std::io;
use std::mem;
use std::os::fd::RawFd;

const PF_CAN: i32 = 29;
const AF_CAN: u16 = 29;
const CAN_RAW: i32 = 1;

#[repr(C)]
struct CanFrame {
    can_id: u32,
    len: u8,
    pad: u8,
    res0: u8,
    len8_dlc: u8,
    data: [u8; 8],
}

#[repr(C)]
struct SockAddrCan {
    can_family: u16,
    can_ifindex: i32,
    can_addr: [u8; 8],
}

fn main() -> io::Result<()> {
    let name = CString::new("vcan0").unwrap();

    let ifindex = unsafe {
        libc::if_nametoindex(name.as_ptr())
    };

    if ifindex == 0 {
        return Err(io::Error::last_os_error());
    }

    let fd: RawFd = unsafe {
        libc::socket(PF_CAN, libc::SOCK_RAW, CAN_RAW)
    };

    if fd < 0 {
        return Err(io::Error::last_os_error());
    }

    let result = send_frame(fd, ifindex);

    unsafe {
        libc::close(fd);
    }

    result
}

fn send_frame(fd: RawFd, ifindex: u32) -> io::Result<()> {
    let addr = SockAddrCan {
        can_family: AF_CAN,
        can_ifindex: ifindex as i32,
        can_addr: [0; 8],
    };

    let rc = unsafe {
        libc::bind(
            fd,
            &addr as *const _ as *const libc::sockaddr,
            mem::size_of::<SockAddrCan>() as libc::socklen_t,
        )
    };

    if rc < 0 {
        return Err(io::Error::last_os_error());
    }

    let frame = CanFrame {
        can_id: 0x123,
        len: 4,
        pad: 0,
        res0: 0,
        len8_dlc: 0,
        data: [0x11, 0x22, 0x33, 0x44, 0, 0, 0, 0],
    };

    let written = unsafe {
        libc::write(
            fd,
            &frame as *const _ as *const libc::c_void,
            mem::size_of::<CanFrame>(),
        )
    };

    if written < 0 {
        return Err(io::Error::last_os_error());
    }

    if written as usize != mem::size_of::<CanFrame>() {
        return Err(io::Error::new(
            io::ErrorKind::WriteZero,
            "Incomplete CAN frame write",
        ));
    }

    println!("Sent CAN frame 0x123");
    Ok(())
}
