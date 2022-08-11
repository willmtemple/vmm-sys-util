// Copyright (C) 2019 Alibaba Cloud Computing. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause

//! Safe wrapper over [`Linux AIO`](http://man7.org/linux/man-pages/man7/aio.7.html).

#![allow(non_camel_case_types)]

/* automatically generated by rust-bindgen from file linux/include/uapi/linux/aio_abi.h
 * of commit 69973b8 and then manually edited */

use std::io::{Error, Result};
use std::os::raw::{c_int, c_long, c_uint, c_ulong};
use std::ptr::null_mut;

type __s16 = ::std::os::raw::c_short;
type __u16 = ::std::os::raw::c_ushort;
type __u32 = ::std::os::raw::c_uint;
type __s64 = ::std::os::raw::c_longlong;
type __u64 = ::std::os::raw::c_ulonglong;

/// Read from a file descriptor at a given offset.
pub const IOCB_CMD_PREAD: u32 = 0;
/// Write to a file descriptor at a given offset.
pub const IOCB_CMD_PWRITE: u32 = 1;
/// Synchronize a file's in-core metadata and data to storage device.
pub const IOCB_CMD_FSYNC: u32 = 2;
/// Synchronize a file's in-core data to storage device.
pub const IOCB_CMD_FDSYNC: u32 = 3;
/// Noop, this defined by never used by linux kernel.
pub const IOCB_CMD_NOOP: u32 = 6;
/// Read from a file descriptor at a given offset into multiple buffers.
pub const IOCB_CMD_PREADV: u32 = 7;
/// Write to a file descriptor at a given offset from multiple buffers.
pub const IOCB_CMD_PWRITEV: u32 = 8;

/// Valid flags for the "aio_flags" member of the "struct iocb".
/// Set if the "aio_resfd" member of the "struct iocb" is valid.
pub const IOCB_FLAG_RESFD: u32 = 1;

/// Maximum number of concurrent requests.
pub const MAX_REQUESTS: usize = 0x10000;

/// Wrapper over the [`iocb`](https://elixir.bootlin.com/linux/v4.9/source/include/uapi/linux/aio_abi.h#L79) structure.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IoControlBlock {
    pub aio_data: __u64,
    pub aio_key: __u32,
    pub aio_reserved1: __u32,
    pub aio_lio_opcode: __u16,
    pub aio_reqprio: __s16,
    pub aio_fildes: __u32,
    pub aio_buf: __u64,
    pub aio_nbytes: __u64,
    pub aio_offset: __s64,
    pub aio_reserved2: __u64,
    pub aio_flags: __u32,
    pub aio_resfd: __u32,
}

/// Wrapper over the [`io_event`](https://elixir.bootlin.com/linux/v4.9/source/include/uapi/linux/aio_abi.h#L58) structure.
#[allow(missing_docs)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct IoEvent {
    pub data: __u64,
    pub obj: __u64,
    pub res: __s64,
    pub res2: __s64,
}

/// Newtype for [`aio_context_t`](https://elixir.bootlin.com/linux/v4.9/source/include/uapi/linux/aio_abi.h#L33).
#[repr(transparent)]
#[derive(Debug)]
pub struct IoContext(::std::os::raw::c_ulong);

impl IoContext {
    /// Create a new aio context instance.
    ///
    /// Refer to Linux [`io_setup`](http://man7.org/linux/man-pages/man2/io_setup.2.html).
    ///
    /// # Arguments
    /// * `nr_events`: maximum number of concurrently processing IO operations.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(nr_events: c_uint) -> Result<Self> {
        if nr_events as usize > MAX_REQUESTS {
            return Err(Error::from_raw_os_error(libc::EINVAL));
        }

        let mut ctx = IoContext(0);
        // Safe because we have checked the result.
        let rc =
            unsafe { libc::syscall(libc::SYS_io_setup, nr_events, &mut ctx as *mut Self) as c_int };
        if rc < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(ctx)
        }
    }

    /// Submit asynchronous I/O blocks for processing.
    ///
    /// Refer to Linux [`io_submit`](http://man7.org/linux/man-pages/man2/io_submit.2.html).
    ///
    /// # Arguments
    /// * `iocbs`: array of AIO control blocks, which will be submitted to the context.
    ///
    /// # Examples
    /// ```
    /// extern crate vmm_sys_util;
    /// use vmm_sys_util::aio::*;
    /// # use std::fs::File;
    /// # use std::os::unix::io::AsRawFd;
    ///
    /// let file = File::open("/dev/zero").unwrap();
    /// let ctx = IoContext::new(128).unwrap();
    /// let mut buf: [u8; 4096] = unsafe { std::mem::uninitialized() };
    /// let iocbs = [&mut IoControlBlock {
    ///     aio_fildes: file.as_raw_fd() as u32,
    ///     aio_lio_opcode: IOCB_CMD_PREAD as u16,
    ///     aio_buf: buf.as_mut_ptr() as u64,
    ///     aio_nbytes: buf.len() as u64,
    ///     ..Default::default()
    /// }];
    /// assert_eq!(ctx.submit(&iocbs[..]).unwrap(), 1);
    /// ```
    pub fn submit(&self, iocbs: &[&mut IoControlBlock]) -> Result<usize> {
        let rc = unsafe {
            // It's safe because parameters are valid and we have checked the result.
            libc::syscall(
                libc::SYS_io_submit,
                self.0,
                iocbs.len() as c_ulong,
                iocbs.as_ptr(),
            ) as c_int
        };
        if rc < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(rc as usize)
        }
    }

    /// Cancel an outstanding asynchronous I/O operation.
    ///
    /// Refer to Linux [`io_cancel`](http://man7.org/linux/man-pages/man2/io_cancel.2.html).
    /// Note: according to current Linux kernel implementation(v4.19), libc::SYS_io_cancel always
    /// return failure, thus rendering it useless.
    ///
    /// # Arguments
    /// * `iocb`: The iocb for the operation to be canceled.
    /// * `result`: If the operation is successfully canceled, the event will be copied into the
    ///             memory pointed to by result without being placed into the completion queue.
    pub fn cancel(&self, iocb: &IoControlBlock, result: &mut IoEvent) -> Result<()> {
        let rc = unsafe {
            // It's safe because parameters are valid and we have checked the result.
            libc::syscall(
                libc::SYS_io_cancel,
                self.0,
                iocb as *const IoControlBlock,
                result as *mut IoEvent,
            ) as c_int
        };
        if rc < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    /// Read asynchronous I/O events from the completion queue.
    ///
    /// Refer to Linux [`io_getevents`](http://man7.org/linux/man-pages/man2/io_getevents.2.html).
    ///
    /// # Arguments
    /// * `min_nr`: read at least min_nr events.
    /// * `events`: array to receive the io operation results.
    /// * `timeout`: optional amount of time to wait for events.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate vmm_sys_util;
    /// use vmm_sys_util::aio::*;
    /// # use std::fs::File;
    /// # use std::os::unix::io::AsRawFd;
    ///
    /// let file = File::open("/dev/zero").unwrap();
    /// let ctx = IoContext::new(128).unwrap();
    /// let mut buf: [u8; 4096] = unsafe { std::mem::uninitialized() };
    /// let iocbs = [
    ///     &mut IoControlBlock {
    ///         aio_fildes: file.as_raw_fd() as u32,
    ///         aio_lio_opcode: IOCB_CMD_PREAD as u16,
    ///         aio_buf: buf.as_mut_ptr() as u64,
    ///         aio_nbytes: buf.len() as u64,
    ///         ..Default::default()
    ///     },
    ///     &mut IoControlBlock {
    ///         aio_fildes: file.as_raw_fd() as u32,
    ///         aio_lio_opcode: IOCB_CMD_PREAD as u16,
    ///         aio_buf: buf.as_mut_ptr() as u64,
    ///         aio_nbytes: buf.len() as u64,
    ///         ..Default::default()
    ///     },
    /// ];
    ///
    /// let mut rc = ctx.submit(&iocbs[..]).unwrap();
    /// let mut events = [unsafe { std::mem::uninitialized::<IoEvent>() }];
    /// rc = ctx.get_events(1, &mut events, None).unwrap();
    /// assert_eq!(rc, 1);
    /// assert!(events[0].res > 0);
    /// rc = ctx.get_events(1, &mut events, None).unwrap();
    /// assert_eq!(rc, 1);
    /// assert!(events[0].res > 0);
    /// ```
    pub fn get_events(
        &self,
        min_nr: c_long,
        events: &mut [IoEvent],
        timeout: Option<&mut libc::timespec>,
    ) -> Result<usize> {
        let to = match timeout {
            Some(val) => val as *mut libc::timespec,
            None => null_mut() as *mut libc::timespec,
        };

        // It's safe because parameters are valid and we have checked the result.
        let rc = unsafe {
            libc::syscall(
                libc::SYS_io_getevents,
                self.0,
                min_nr,
                events.len() as c_long,
                events.as_mut_ptr(),
                to,
            ) as c_int
        };
        if rc < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(rc as usize)
        }
    }
}

impl Drop for IoContext {
    fn drop(&mut self) {
        if self.0 != 0 {
            // It's safe because the context is created by us.
            let _ = unsafe { libc::syscall(libc::SYS_io_destroy, self.0) as c_int };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::os::unix::io::AsRawFd;

    #[test]
    fn test_new_context() {
        let _ = IoContext::new(0).unwrap_err();
    }

    #[test]
    fn test_cancel_request() {
        let file = File::open("/dev/zero").unwrap();

        let ctx = IoContext::new(128).unwrap();
        let mut buf: [u8; 16384] = [0u8; 16384];
        let iocbs = [&mut IoControlBlock {
            aio_fildes: file.as_raw_fd() as u32,
            aio_lio_opcode: IOCB_CMD_PREAD as u16,
            aio_buf: buf.as_mut_ptr() as u64,
            aio_nbytes: buf.len() as u64,
            ..Default::default()
        }];

        let mut rc = ctx.submit(&iocbs).unwrap();
        assert_eq!(rc, 1);

        let mut result = Default::default();
        let err = ctx
            .cancel(iocbs[0], &mut result)
            .unwrap_err()
            .raw_os_error()
            .unwrap();
        assert_eq!(err, libc::EINVAL);

        let mut events = [IoEvent::default()];
        rc = ctx.get_events(1, &mut events, None).unwrap();
        assert_eq!(rc, 1);
        assert!(events[0].res > 0);
    }

    #[test]
    fn test_read_zero() {
        let file = File::open("/dev/zero").unwrap();

        let ctx = IoContext::new(128).unwrap();
        let mut buf: [u8; 4096] = [0u8; 4096];
        let iocbs = [
            &mut IoControlBlock {
                aio_fildes: file.as_raw_fd() as u32,
                aio_lio_opcode: IOCB_CMD_PREAD as u16,
                aio_buf: buf.as_mut_ptr() as u64,
                aio_nbytes: buf.len() as u64,
                ..Default::default()
            },
            &mut IoControlBlock {
                aio_fildes: file.as_raw_fd() as u32,
                aio_lio_opcode: IOCB_CMD_PREAD as u16,
                aio_buf: buf.as_mut_ptr() as u64,
                aio_nbytes: buf.len() as u64,
                ..Default::default()
            },
        ];

        let mut rc = ctx.submit(&iocbs[..]).unwrap();
        assert_eq!(rc, 2);

        let mut events = [IoEvent::default()];
        rc = ctx.get_events(1, &mut events, None).unwrap();
        assert_eq!(rc, 1);
        assert!(events[0].res > 0);

        rc = ctx.get_events(1, &mut events, None).unwrap();
        assert_eq!(rc, 1);
        assert!(events[0].res > 0);
    }

    #[test]
    fn bindgen_test_layout_io_event() {
        assert_eq!(
            ::std::mem::size_of::<IoEvent>(),
            32usize,
            concat!("Size of: ", stringify!(IoEvent))
        );
        assert_eq!(
            ::std::mem::align_of::<IoEvent>(),
            8usize,
            concat!("Alignment of", stringify!(IoEvent))
        );
    }

    #[test]
    fn bindgen_test_layout_iocb() {
        assert_eq!(
            ::std::mem::size_of::<IoControlBlock>(),
            64usize,
            concat!("Size of:", stringify!(IoControlBlock))
        );
        assert_eq!(
            ::std::mem::align_of::<IoControlBlock>(),
            8usize,
            concat!("Alignment of", stringify!(IoControlBlock))
        );
    }
}
