use std::{
    io::{self, Result},
    mem,
    os::fd::RawFd,
    sync::Mutex,
};

use libc::{TCSANOW, cfmakeraw, tcgetattr, tcsetattr, termios as Termios};

use crate::modules::file_descriptor::tty_fd;

static TERMINAL_MODE_PRIOR_RAW_MODE: Mutex<Option<Termios>> = Mutex::new(None);

pub struct RawModeGuard;

impl RawModeGuard {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

pub fn is_raw_mode_enabled() -> bool {
    return TERMINAL_MODE_PRIOR_RAW_MODE.lock().unwrap().is_some();
}

pub fn enable_raw_mode() -> Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_RAW_MODE.lock().unwrap();
    if original_mode.is_some() {
        return Ok(());
    }

    let tty_fd = tty_fd()?.raw_fd();
    let mut ios = get_terminal_attr(tty_fd)?;
    let original_mode_ios = ios;

    raw_terminal_attr(&mut ios);
    set_terminal_attr(tty_fd, &ios)?;

    // Keep it last - set the original mode only if we were able to switch to the raw mode
    *original_mode = Some(original_mode_ios);

    Ok(())
}

pub fn disable_raw_mode() -> Result<()> {
    let mut original_mode = TERMINAL_MODE_PRIOR_RAW_MODE.lock().unwrap();
    if let Some(original_mode_ios) = original_mode.as_ref() {
        let tty_fd = tty_fd()?;
        set_terminal_attr(tty_fd.raw_fd(), original_mode_ios)?;
        // Keep it last - remove the original mode only if we were able to switch back
        *original_mode = None;
    }
    Ok(())
}

fn get_terminal_attr(fd: RawFd) -> Result<Termios> {
    unsafe {
        let mut termios = mem::zeroed();
        wrap_with_result(tcgetattr(fd, &mut termios))?;
        Ok(termios)
    }
}

fn wrap_with_result(result: i32) -> Result<()> {
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn raw_terminal_attr(termios: &mut Termios) {
    unsafe { cfmakeraw(termios) }
}

fn set_terminal_attr(fd: RawFd, termios: &Termios) -> io::Result<()> {
    wrap_with_result(unsafe { tcsetattr(fd, TCSANOW, termios) })
}
