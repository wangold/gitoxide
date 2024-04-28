use std::ops::Range;

use bstr::{ByteSlice, ByteVec};

use crate::clear_and_set_capacity;

/// Undo identifiers like `$Id:<hexsha>$` to `$Id$` in `src` and write to `buf`. Newlines between dollars are ignored.
/// Return `true` if `buf` was written or `false` if `src` was left unaltered (as there was nothing to do).
pub fn undo(src: &[u8], buf: &mut Vec<u8>) -> bool {
    fn find_range(input: &[u8]) -> Option<Range<usize>> {
        let mut ofs = 0;
        loop {
            let mut cursor = input.get(ofs..)?;
            let start = cursor.find(b"$Id:")?;
            cursor = cursor.get((start + 4)..)?;
            let maybe_end = cursor.find_byteset(b"$\n")?;
            if cursor[maybe_end] == b'\n' {
                ofs += start + 4 + maybe_end + 1;
                continue;
            } else {
                return Some((ofs + start)..(ofs + start + 4 + maybe_end + 1));
            }
        }
    }

    let mut ofs = 0;
    let mut initialized = false;
    while let Some(range) = find_range(&src[ofs..]) {
        if !initialized {
            clear_and_set_capacity(buf, src.len());
            initialized = true;
        }
        buf.push_str(&src[ofs..][..range.start]);
        buf.push_str(b"$Id$");
        ofs += range.end;
    }
    if initialized {
        buf.push_str(&src[ofs..]);
    }
    initialized
}

/// Substitute all occurrences of `$Id$` with `$Id: <hexsha-of-input>$` if present in `src` and write all changes to `buf`,
/// with `object_hash` being used accordingly. Return `true` if `buf` was written to or `false` if no change was made
/// (as there was nothing to do).
///
/// ### Deviation
///
/// `Git` also tries to cleanup 'stray' substituted `$Id: <hex>$`, but we don't do that, sticking exactly to what ought to be done.
/// The respective code is up to 16 years old and one might assume that `git` by now handles checking and checkout filters correctly.
pub fn apply(src: &[u8], object_hash: gix_hash::Kind, buf: &mut Vec<u8>) -> bool {
    const HASH_LEN: usize = ": ".len() + gix_hash::Kind::longest().len_in_hex();
    let mut id = None;
    let mut ofs = 0;
    while let Some(pos) = src[ofs..].find(b"$Id$") {
        let id = match id {
            None => {
                let new_id = gix_object::compute_hash(object_hash, gix_object::Kind::Blob, src);
                id = new_id.into();
                clear_and_set_capacity(buf, src.len() + HASH_LEN); // pre-allocate for one ID
                new_id
            }
            Some(id) => id.to_owned(),
        };

        buf.push_str(&src[ofs..][..pos + 3]);
        buf.push_str(b": ");
        id.write_hex_to(&mut *buf).expect("writes to memory always work");
        buf.push(b'$');

        ofs += pos + 4;
    }
    if id.is_some() {
        buf.push_str(&src[ofs..]);
    }
    id.is_some()
}
