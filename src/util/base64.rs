pub fn encode(bytes: &[u8], size: usize) -> String {
    let mut buf = String::new();
    let i = 0;
    for mut i in 0..(size / 3) {
        i = i * 3;
        let f = bytes[i];
        let s = bytes[i + 1];
        let t = bytes[i + 2];

        buf.push_str(&cvt((f & 0xfc) >> 2));
        buf.push_str(&cvt((f & 0x03) << 4 | ((s & 0xf0) >> 4)));
        buf.push_str(&cvt((s & 0x0f) << 2 | ((t & 0xc0) >> 6)));
        buf.push_str(&cvt(t & 0x3f));
    }

    let mut i = (i + 1) * 3;
    i = if size < i { 0 } else { i };
    let remain = size - i;
    if remain == 1 {
        let f = bytes[i];
        buf.push_str(&cvt((f & 0xfc) >> 2));
        buf.push_str(&cvt((f & 0x03) << 4 | 0));
        buf.push_str("==");
    } else if remain == 2 {
        let f = bytes[i];
        let s = bytes[i + 1];
        buf.push_str(&cvt((f & 0xfc) >> 2));
        buf.push_str(&cvt((f & 0x03) << 4 | ((s & 0xf0) >> 4)));
        buf.push_str(&cvt((s & 0x0f) << 2 | 0));
        buf.push_str("=");
    }
    buf
}

const BASE64_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

fn cvt(i: u8) -> String {
    BASE64_TABLE.get(i as usize).unwrap().to_string()
}

#[cfg(test)]
mod test {
    use crate::util::base64::encode;
    #[test]
    fn test_encode() {

    }

}
