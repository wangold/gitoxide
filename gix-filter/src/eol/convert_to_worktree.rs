use bstr::{ByteSlice, ByteVec};

use crate::{
    clear_and_set_capacity,
    eol::{AttributesDigest, Configuration, Mode, Stats},
};

/// Convert all `\n` in `src` to `crlf` if `digest` and `config` indicate it, returning `true` if `buf` holds the result, or `false`
/// if no change was made after all.
pub fn convert_to_worktree(src: &[u8], digest: AttributesDigest, buf: &mut Vec<u8>, config: Configuration) -> bool {
    if src.is_empty() || digest.to_eol(config) != Some(Mode::CrLf) {
        return false;
    }
    let stats = Stats::from_bytes(src);
    if !stats.will_convert_lf_to_crlf(digest, config) {
        return false;
    }

    clear_and_set_capacity(buf, src.len() + stats.lone_lf);

    let mut ofs = 0;
    while let Some(pos) = src[ofs..].find_byteset(b"\r\n") {
        match src[ofs + pos] {
            b'\r' => {
                if src.get(ofs + pos + 1) == Some(&b'\n') {
                    buf.push_str(&src[ofs..][..pos + 2]);
                    ofs += pos + 2;
                } else {
                    buf.push_str(&src[ofs..][..pos + 1]);
                    ofs += pos + 1;
                }
            }
            b'\n' => {
                buf.push_str(&src[ofs..][..pos]);
                buf.push_str(b"\r\n");
                ofs += pos + 1;
            }
            _ => unreachable!("would only find one of two possible values"),
        }
    }
    buf.push_str(&src[ofs..]);
    true
}
