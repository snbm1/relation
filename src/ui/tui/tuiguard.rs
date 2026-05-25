use std::io;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

#[cfg(unix)]
use std::os::fd::RawFd;

pub struct TuiGuard {
    #[cfg(unix)]
    pub saved_stdout: RawFd,
    #[cfg(unix)]
    pub saved_stderr: RawFd,
    pub active: bool,
}

impl TuiGuard {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        #[cfg(unix)] 
        {
            let saved_stdout = unsafe { libc::dup(libc::STDOUT_FILENO) };
            if saved_stdout < 0 {
                let _ = disable_raw_mode();
                let _ = execute!(io::stdout(), LeaveAlternateScreen);
                return Err(io::Error::last_os_error());
            }

            let saved_stderr = unsafe { libc::dup(libc::STDERR_FILENO) };
            if saved_stderr < 0 {
                unsafe {
                    libc::close(saved_stdout);
                }
                let _ = disable_raw_mode();
                let _ = execute!(io::stdout(), LeaveAlternateScreen);
                return Err(io::Error::last_os_error());
            }

            Ok(Self {
                saved_stdout,
                saved_stderr,
                active: true,
            })
        }
        #[cfg(windows)]
        {
            Ok(Self {
                active: true,
            })
        }
        
    }

    pub fn restore(&mut self) {
        if !self.active {
            return;
        }
        #[cfg(unix)] 
        {
            unsafe {
            libc::dup2(self.saved_stdout, libc::STDOUT_FILENO);
            libc::dup2(self.saved_stderr, libc::STDERR_FILENO);
            }

            let _ = disable_raw_mode();
            let _ = execute!(io::stdout(), LeaveAlternateScreen);

            
            unsafe {
                libc::close(self.saved_stdout);
                libc::close(self.saved_stderr);
            }
        }
        #[cfg(windows)]
        {
            let _ = disable_raw_mode();
            let _ = execute!(io::stdout(), LeaveAlternateScreen);
        }
        

        self.active = false;
    }
}

impl Drop for TuiGuard {
    fn drop(&mut self) {
        self.restore();
    }
}
