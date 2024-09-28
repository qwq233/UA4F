use log::{debug, error};

pub fn is_http_request(buf: &mut [u8]) -> bool {
    if buf[0..3] == [b'G', b'E', b'T'] {
        // GET
        true
    } else if buf[0..4] == [b'P', b'O', b'S', b'T'] {
        // POST
        true
    } else if buf[0..4] == [b'H', b'E', b'A', b'D'] {
        // HEAD
        true
    } else if buf[0..6] == [b'D', b'E', b'L', b'E', b'T', b'E'] {
        // DELETE
        true
    } else if buf[0..5] == [b'T', b'R', b'A', b'C', b'E'] {
        // TRACE
        true
    } else if buf[0..7] == [b'O', b'P', b'T', b'I', b'O', b'N', b'S'] {
        // OPTIONS
        true
    } else if buf[0..7] == [b'C', b'O', b'N', b'N', b'E', b'C', b'T'] {
        // CONNECT
        true
    } else if buf[0..3] == [b'P', b'U', b'T'] {
        // PUT
        true
    } else if buf[0..5] == [b'P', b'A', b'T', b'C', b'H'] {
        // PATCH
        true
    } else {
        false
    }
}

pub fn modify_user_agent(buf: &mut Vec<u8>, user_agent: &String) {
    let len = buf.len();
    let mut pos = 0 as usize;
    let mut start = 0 as usize;
    let mut end = 0 as usize;
    const TARGET: &[u8; 12] = b"User-Agent: ";

    while pos < len {
        if pos + TARGET.len() >= len {
            error!("User-Agent not found, end of buffer");
            return;
            
        }
        if buf[pos..pos + TARGET.len()] == *TARGET {
            start = pos;
            break;
        }
        pos += 1;
    }
    if pos >= len {
        error!("User-Agent not found, start not found");
        return;
    }

    while pos < len {
        if pos + 1 >= len {
            error!("User-Agent not found, end of buffer");
            return;
        }
        if buf[pos] == b'\r' && buf[pos + 1] == b'\n' {
            end = pos;
            break;
        }
        pos += 1;
    }
    if pos >= len {
        error!("User-Agent not found, end not found");
        return;
    }

    debug!("start: {}, end: {}", start, end);
    debug!("user_agent: {}", String::from_utf8_lossy(&buf[start..end]));

    buf.drain(start + 12..end - 1);
    let user_agent = user_agent.as_bytes();
    let mut len = user_agent.len();
    while len > 0 {
        buf.insert(start + 12, user_agent[len - 1]);
        len -= 1;
    }

    debug!(
        "new user_agent: {}",
        String::from_utf8_lossy(&buf[start..start + 12 + user_agent.len()])
    );
}
