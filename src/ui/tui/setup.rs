use std::{
    fs::{File, OpenOptions},
    io,
};

#[cfg(unix)]
use os::fd::{AsRawFd, FromRawFd};

use anyhow::Result; 
use ratatui::{backend::CrosstermBackend, Terminal};

use super::tuiguard::TuiGuard;

pub struct Tui {
    #[cfg(unix)]
    pub terminal: Terminal<CrosstermBackend<File>>,
    #[cfg(windows)]
    pub terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub _guard: TuiGuard,
}

pub fn setup_tty() -> Result<Tui> {
    let _guard = TuiGuard::new()?;

   #[cfg(unix)]
   {
        let ui_fd = unsafe { libc::dup(libc::STDOUT_FILENO) };
        if ui_fd < 0 {
            return Err(io::Error::last_os_error().into());
        }

        let null = OpenOptions::new().write(true).open("/dev/null")?;

        unsafe {
            if libc::dup2(null.as_raw_fd(), libc::STDOUT_FILENO) < 0 {
                return Err(io::Error::last_os_error().into());
            }

            if libc::dup2(null.as_raw_fd(), libc::STDERR_FILENO) < 0 {
                return Err(io::Error::last_os_error().into());
            }
        }

        let ui_out = unsafe { File::from_raw_fd(ui_fd) };
        let backend = CrosstermBackend::new(ui_out);
        let terminal = Terminal::new(backend)?;

        Ok(Tui { terminal, _guard })
   }

   #[cfg(windows)]
   {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;

        Ok(Tui { terminal, _guard })
   }
}