//! SHA-2 (256 Bit)

struct BufState {
    data: Vec<u8>,
    len: usize,
    total_len: usize,
    single: bool,
    total: bool,
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hash: [u8; 32] = [0; 32];

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut chunk: [u8; 64] = [0; 64];

    let mut state: BufState = BufState {
        data: (*data).to_owned(),
        len: data.len(),
        total_len: data.len(),
        single: false,
        total: false,
    };

    while calc_chunk(&mut chunk, &mut state) {
        let mut ah: [u32; 8] = h;
        let mut w: [u32; 16] = [0; 16];
        for i in 0..4 {
            for j in 0..16 {
                if i == 0 {
                    w[j] = ((chunk[j * 4] as u32) << 24)
                        | ((chunk[j * 4 + 1] as u32) << 16)
                        | ((chunk[j * 4 + 2] as u32) << 8)
                        | (chunk[j * 4 + 3] as u32);
                } else {
                    let s0 = (w[(j + 1) & 0xf].rotate_right(7) ^ w[(j + 1) & 0xf].rotate_right(18))
                        ^ (w[(j + 1) & 0xf] >> 3);
                    let s1 = w[(j + 14) & 0xf].rotate_right(17)
                        ^ w[(j + 14) & 0xf].rotate_right(19)
                        ^ (w[(j + 14) & 0xf] >> 10);
                    w[j] = w[j]
                        .wrapping_add(s0)
                        .wrapping_add(w[(j + 9) & 0xf])
                        .wrapping_add(s1);
                }

                let s1: u32 =
                    ah[4].rotate_right(6) ^ ah[4].rotate_right(11) ^ ah[4].rotate_right(25);
                let ch: u32 = (ah[4] & ah[5]) ^ (!ah[4] & ah[6]);
                let temp1: u32 = ah[7]
                    .wrapping_add(s1)
                    .wrapping_add(ch)
                    .wrapping_add(k[i << 4 | j])
                    .wrapping_add(w[j]);
                let s0: u32 =
                    ah[0].rotate_right(2) ^ ah[0].rotate_right(13) ^ ah[0].rotate_right(22);
                let maj: u32 = (ah[0] & ah[1]) ^ (ah[0] & ah[2]) ^ (ah[1] & ah[2]);
                let temp2: u32 = s0.wrapping_add(maj);

                ah[7] = ah[6];
                ah[6] = ah[5];
                ah[5] = ah[4];
                ah[4] = ah[3].wrapping_add(temp1);
                ah[3] = ah[2];
                ah[2] = ah[1];
                ah[1] = ah[0];
                ah[0] = temp1.wrapping_add(temp2);
            }
        }

        for i in 0..8 {
            h[i] = h[i].wrapping_add(ah[i]);
        }
        chunk = [0; 64];
    }

    for i in 0..8 {
        hash[i * 4] = (h[i] >> 24) as u8;
        hash[i * 4 + 1] = (h[i] >> 16) as u8;
        hash[i * 4 + 2] = (h[i] >> 8) as u8;
        hash[i * 4 + 3] = h[i] as u8;
    }

    hash
}

fn calc_chunk(chunk: &mut [u8; 64], state: &mut BufState) -> bool {
    if state.total {
        return false;
    }

    if state.len >= 64 {
        for x in chunk {
            *x = state.data[0];
            state.data.remove(0);
        }
        state.len -= 64;
        return true;
    }

    let remaining: usize = state.data.len();
    let space: usize = 64 - remaining;
    for x in chunk.iter_mut().take(state.data.len()) {
        *x = state.data[0];
        state.data.remove(0);
    }

    if !state.single {
        chunk[remaining] = 0x80;
        state.single = true;
    }

    if space >= 8 {
        let mut len = state.total_len;
        chunk[63] = (len << 3) as u8;
        len >>= 5;
        for i in 1..8 {
            chunk[(63 - i)] = len as u8;
            len >>= 8;
        }
        state.total = true;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            sha256(&Vec::new()),
            [
                0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
                0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
                0x78, 0x52, 0xb8, 0x55
            ]
        );
    }

    #[test]
    fn ascii() {
        assert_eq!(
            sha256(&b"The quick brown fox jumps over the lazy dog".to_vec()),
            [
                0xD7, 0xA8, 0xFB, 0xB3, 0x07, 0xD7, 0x80, 0x94, 0x69, 0xCA, 0x9A, 0xBC, 0xB0, 0x08,
                0x2E, 0x4F, 0x8D, 0x56, 0x51, 0xE4, 0x6D, 0x3C, 0xDB, 0x76, 0x2D, 0x02, 0xD0, 0xBF,
                0x37, 0xC9, 0xE5, 0x92
            ]
        )
    }

    #[test]
    fn ascii_avalanche() {
        assert_eq!(
            sha256(&b"The quick brown fox jumps over the lazy dog.".to_vec()),
            [
                0xEF, 0x53, 0x7F, 0x25, 0xC8, 0x95, 0xBF, 0xA7, 0x82, 0x52, 0x65, 0x29, 0xA9, 0xB6,
                0x3D, 0x97, 0xAA, 0x63, 0x15, 0x64, 0xD5, 0xD7, 0x89, 0xC2, 0xB7, 0x65, 0x44, 0x8C,
                0x86, 0x35, 0xFB, 0x6C
            ]
        )
    }
}
