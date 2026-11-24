use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::{Duration, Instant};

pub struct Serial {
    stream: UnixStream,
    buf: Vec<u8>,
}

impl Serial {
    pub fn connect(sock: &Path) -> Result<Self, String> {
        let stream = UnixStream::connect(sock)
            .map_err(|e| format!("connect serial {}: {e}", sock.display()))?;
        stream
            .set_read_timeout(Some(Duration::from_millis(500)))
            .ok();
        Ok(Self {
            stream,
            buf: Vec::new(),
        })
    }

    pub fn send(&mut self, s: &str) -> Result<(), String> {
        self.stream
            .write_all(s.as_bytes())
            .map_err(|e| format!("serial write: {e}"))
    }

    /// Consumes through `needle`; returns the text before it.
    pub fn expect(&mut self, needle: &str, timeout: Duration) -> Result<String, String> {
        let deadline = Instant::now() + timeout;
        loop {
            if let Some(pos) = find(&self.buf, needle.as_bytes()) {
                let before = String::from_utf8_lossy(&self.buf[..pos]).into_owned();
                self.buf.drain(..pos + needle.len());
                return Ok(before);
            }
            if Instant::now() > deadline {
                let tail = String::from_utf8_lossy(&self.buf);
                let tail: String = tail
                    .chars()
                    .rev()
                    .take(600)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();
                return Err(format!(
                    "serial: no {needle:?} within {timeout:?}; last output:\n{tail}"
                ));
            }
            let mut chunk = [0u8; 4096];
            match self.stream.read(&mut chunk) {
                Ok(0) => return Err("serial: connection closed".into()),
                Ok(n) => self.buf.extend_from_slice(&chunk[..n]),
                Err(e)
                    if matches!(
                        e.kind(),
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                    ) => {}
                Err(e) => return Err(format!("serial read: {e}")),
            }
        }
    }

    pub fn login(&mut self, user: &str, password: &str, timeout: Duration) -> Result<(), String> {
        let deadline = Instant::now() + timeout;
        loop {
            self.send("\n")?;
            match self.expect("login:", Duration::from_secs(5)) {
                Ok(_) => break,
                Err(e) if Instant::now() > deadline => return Err(e),
                Err(_) => {}
            }
        }
        self.send(&format!("{user}\n"))?;
        self.expect("assword:", Duration::from_secs(30))?;
        self.send(&format!("{password}\n"))?;
        // `login` flushes pending terminal input when it hands over to
        // the shell, so anything typed before the prompt is lost. Wait
        // for a root prompt (ends in "# "), then prove the shell works.
        self.expect("# ", Duration::from_secs(120))?;
        self.run("true", Duration::from_secs(30)).map(|_| ())
    }

    pub fn run(&mut self, script: &str, timeout: Duration) -> Result<(i64, String), String> {
        const MARK: &str = "E2E-SERIAL-DONE";
        let (a, b) = MARK.split_at(5);
        self.buf.clear();
        // `stty -echo` keeps the console from echoing the (possibly long)
        // command back; the split marker covers the case where it still does.
        self.send(&format!(
            "stty -echo 2>/dev/null; {{ {script}\n}}; echo \"{a}\"\"{b}:$?\"\n"
        ))?;
        let out = self.expect(&format!("{MARK}:"), timeout)?;
        let code = self.expect("\n", Duration::from_secs(5))?;
        let code = code
            .trim()
            .trim_end_matches(|c: char| !c.is_ascii_digit())
            .parse()
            .unwrap_or(-1);
        Ok((code, out))
    }
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}
