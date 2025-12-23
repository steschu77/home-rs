// ----------------------------------------------------------------------------
pub fn next_code_point<'a, I: Iterator<Item = &'a u8>>(bytes: &mut I) -> Option<u32> {
    // Note: This function assumes valid UTF-8 input.
    let x = *bytes.next()? as u32;
    if x < 0x80 {
        Some(x)
    } else if x < 0xE0 {
        let y = (*bytes.next()? & 0x3F) as u32;
        Some((x & 0x1F) << 6 | y)
    } else if x < 0xF0 {
        let y = (*bytes.next()? & 0x3F) as u32;
        let z = (*bytes.next()? & 0x3F) as u32;
        Some((x & 0x0F) << 12 | y << 6 | z)
    } else if x < 0xF8 {
        let y = (*bytes.next()? & 0x3F) as u32;
        let z = (*bytes.next()? & 0x3F) as u32;
        let w = (*bytes.next()? & 0x3F) as u32;
        Some((x & 0x07) << 18 | y << 12 | z << 6 | w)
    } else {
        None
    }
}
