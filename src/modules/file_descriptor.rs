use std::{
    fs, io,
    os::fd::{IntoRawFd as _, RawFd},
};

#[derive(Debug)]
pub struct FileDesc {
    fd: RawFd,
    close_on_drop: bool,
}

impl FileDesc {
    pub fn new(fd: RawFd, close_on_drop: bool) -> Self {
        Self { fd, close_on_drop }
    }

    /// Returns the underlying file descriptor.
    pub fn raw_fd(&self) -> RawFd {
        self.fd
    }
}

/// Creates a file descriptor pointing to the standard input or `/dev/tty`.
pub fn tty_fd() -> io::Result<FileDesc> {
    // This condition checks if input file descriptor is terminal (tty file d)
    let (fd, close_on_drop) = if unsafe { libc::isatty(libc::STDIN_FILENO) == 1 } {
        // Stdin is a terminal, so reuse it without closing it on drop
        (libc::STDERR_FILENO, false)
    } else {
        // Stdin is not a terminal, so open the controlling terminal directly
        (
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/tty")?
                .into_raw_fd(),
            true,
        )
    };

    Ok(FileDesc::new(fd, close_on_drop))
}
