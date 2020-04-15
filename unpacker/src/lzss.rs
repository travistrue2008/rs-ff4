use crate::error::{Result, Error};

const THRESHOLD: usize = 2;
const MAX_LENGTH: usize = 18;
const WINDOW_SIZE: usize = 4096;

pub fn decode(src: &[u8]) -> Result<Vec::<u8>> {
    let src_len = src.len();
    let mut flags = 0u8;
    let mut f_pos = 0usize;
    let mut r_pos = WINDOW_SIZE - MAX_LENGTH;
    let mut offset = 4usize;
    let mut dest = Vec::with_capacity(src_len);
    let mut text_buf = [0u8; MAX_LENGTH + WINDOW_SIZE - 1];

    // print!("LEN: {}", src_len);
    while offset < src_len {
        if f_pos == 0 {
            flags = src[offset];
            offset += 1;

            if offset == src_len {
                break;
            }
        }

        if flags & 1 == 1 {
            let c: u8 = src[offset];

            offset += 1;
            dest.push(c);
            text_buf[r_pos] = c;
            r_pos = (r_pos + 1) % WINDOW_SIZE;
        } else {
            if offset + 1 == src_len {
                return Err(Error::InvalidDecodeLength(offset, src_len));
            }

            let mut i = src[offset] as usize;
            let mut j = src[offset + 1] as usize;
            offset += 2;

            i |= (j & 0xF0) << 4;
            j = (j & 0x0F) + THRESHOLD;

            for k in 0..(j + 1) {
                let c: u8 = text_buf[(i + k) % WINDOW_SIZE];

                dest.push(c);
                text_buf[r_pos] = c;
                r_pos = (r_pos + 1) % WINDOW_SIZE;
            }
        }

        flags >>= 1;
        f_pos = (f_pos + 1) % 8;
    }

    Ok(dest)
}
