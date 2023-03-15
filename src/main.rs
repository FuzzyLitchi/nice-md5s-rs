use rand::seq::SliceRandom;
use rand::Rng;
use std::io::Write;

#[allow(dead_code)]
const N_MD5_OF_DIGITS: usize = 32;
const N_MD5_OF_LETTERS: usize = 24;
const N_GOLD_MD5: usize = 9;
const N_NICE_MATCH: usize = 9;
const N_E_MD5: usize = 7;
const N_PI_MD5: usize = 8;

const HEX_CHARS: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

fn main() {
    let mut iter_count: u64 = 0;
    let start = std::time::Instant::now();

    let mut rng = rand::thread_rng();
    let mut preimage: [u8; 32] = [0; 32];
    for byte in &mut preimage {
        *byte = *HEX_CHARS.choose(&mut rng).unwrap();
    }

    loop {
        let digest = md5::compute(preimage);

        if check_nice_match(digest) >= N_NICE_MATCH {
            println!(
                "Perfect match with {} characters! md5({}) {:x}",
                check_nice_match(digest),
                std::str::from_utf8(&preimage).unwrap(),
                digest
            );
        }

        if check_constant_match(digest, &PI_DIGITS) >= N_PI_MD5 {
            println!(
                "πs match with {} characters! md5({}) {:x}",
                check_constant_match(digest, &PI_DIGITS),
                std::str::from_utf8(&preimage).unwrap(),
                digest
            );
        }

        if check_constant_match(digest, &E_DIGITS) >= N_E_MD5 {
            println!(
                "e match with {} characters! md5({}) {:x}",
                check_constant_match(digest, &E_DIGITS),
                std::str::from_utf8(&preimage).unwrap(),
                digest
            );
        }

        // if check_only_digits(digest) >= N_MD5_OF_DIGITS {
        //     println!(
        //         "Only digits match with {} characters! md5({}) {:x}",
        //         check_only_digits(digest),
        //         std::str::from_utf8(&preimage).unwrap(),
        //         digest
        //     );
        // }

        if check_only_letters(digest) >= N_MD5_OF_LETTERS {
            println!(
                "Only letters match with {} characters! md5({}) {:x}",
                check_only_letters(digest),
                std::str::from_utf8(&preimage).unwrap(),
                digest
            );
        }

        if check_gold(digest, &preimage) >= N_GOLD_MD5 {
            println!(
                "Only letters match with {} characters! md5({}) {:x}",
                check_gold(digest, &preimage),
                std::str::from_utf8(&preimage).unwrap(),
                digest
            );
        }

        iter_count += 1;
        if iter_count & 0xFFFFF == 0 {
            eprint!(
                "{iter_count}it [{:.2}, {:.2}it/s]\r",
                start.elapsed().as_secs_f64(),
                iter_count as f64 / start.elapsed().as_secs_f64(),
            );
            std::io::stderr().flush().unwrap();
        }

        let char = HEX_CHARS.choose(&mut rng).unwrap();
        let index = rng.gen_range(0..preimage.len());
        preimage[index] = *char;
    }
}

fn check_nice_match(digest: md5::Digest) -> usize {
    // A byte here is 2 hex digits
    let first = digest.0[0];
    if first >> 4 != first & 0xF {
        return 0;
    }

    let expected = first & 0xF;
    for (i, byte) in digest.0.iter().enumerate().skip(1) {
        if byte >> 4 != expected {
            return 2 * i;
        }
        if byte & 0xF != expected {
            return 2 * i + 1;
        }
    }

    32
}

const PI_DIGITS: [u8; 32] = [
    3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9, 5,
];
const E_DIGITS: [u8; 32] = [
    2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 5, 9, 0, 4, 5, 2, 3, 5, 3, 6, 0, 2, 8, 7, 4, 7, 1, 3, 5, 2, 6,
];

fn check_constant_match(digest: md5::Digest, constant: &[u8; 32]) -> usize {
    for (i, byte) in digest.0.iter().enumerate() {
        if byte >> 4 != constant[2 * i] {
            return 2 * i;
        }
        if byte & 0x0F != constant[2 * i + 1] {
            return 2 * i + 1;
        }
    }

    32
}

#[allow(dead_code)]
fn check_only_digits(digest: md5::Digest) -> usize {
    for (i, byte) in digest.0.iter().enumerate() {
        if byte >> 4 > 9 {
            return 2 * i;
        }
        if byte & 0x0F > 9 {
            return 2 * i + 1;
        }
    }

    32
}

fn check_only_letters(digest: md5::Digest) -> usize {
    for (i, byte) in digest.0.iter().enumerate() {
        if byte >> 4 < 0xa {
            return 2 * i;
        }
        if byte & 0x0F < 0xa {
            return 2 * i + 1;
        }
    }

    32
}

// un-fucking-readable
fn check_gold(digest: md5::Digest, preimage: &[u8; 32]) -> usize {
    for (i, byte) in digest.0.iter().enumerate() {
        if byte >> 4 != char::from(preimage[2 * i]).to_digit(16).unwrap() as u8 {
            return 2 * i;
        }
        if byte & 0x0F != char::from(preimage[2 * i + 1]).to_digit(16).unwrap() as u8 {
            return 2 * i + 1;
        }
    }

    unimplemented!()
}
