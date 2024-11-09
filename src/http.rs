use log::{debug, error};

pub fn is_http_request(buf: &mut [u8]) -> bool {
    match buf {
        [b'G', b'E', b'T', ..] => true,
        [b'P', b'O', b'S', b'T', ..] => true,
        [b'H', b'E', b'A', b'D', ..] => true,
        [b'D', b'E', b'L', b'E', b'T', b'E', ..] => true,
        [b'T', b'R', b'A', b'C', b'E', ..] => true,
        [b'O', b'P', b'T', b'I', b'O', b'N', b'S', ..] => true,
        [b'C', b'O', b'N', b'N', b'E', b'C', b'T', ..] => true,
        [b'P', b'U', b'T', ..] => true,
        [b'P', b'A', b'T', b'C', b'H', ..] => true,
        _ => false,
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

    if check_is_in_whitelist(&buf[start + 12..end]) {
        return;
    }

    buf.splice(start + 12..end - 1, user_agent.bytes());

    debug!(
        "new user_agent: {}",
        String::from_utf8_lossy(&buf[start..start + 12 + user_agent.len()])
    );
}

fn check_is_in_whitelist(buf: &[u8]) -> bool {
    const WHITELIST: &[&str] = &["MicroMessenger Client", "bilibili"];
    let buf = std::str::from_utf8(buf).unwrap_or("").to_lowercase();
    WHITELIST.iter().any(|&item| buf.contains(item))
}
