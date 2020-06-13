mod cases;

use cases::TEST_CASES;

const FILLER: [u8; 512] = [b'~'; 512];

#[test]
fn test_decode_to_slice() {
    for &(val, s) in TEST_CASES {
        let mut bytes = FILLER;
        assert_eq!(Ok(val.len()), bs58::decode(s).into(&mut bytes[..]));
        assert_eq!(val, &bytes[..val.len()]);
        assert_eq!(&FILLER[val.len()..], &bytes[val.len()..]);
    }
}

#[test]
#[cfg(feature = "alloc")]
fn test_decode_to_vec() {
    for &(val, s) in TEST_CASES {
        assert_eq!(val.to_vec(), bs58::decode(s).into_vec().unwrap());
    }
}

#[test]
fn test_decode_small_buffer_err() {
    let mut output = [0; 2];
    assert_eq!(
        bs58::decode("a3gV").into(&mut output),
        Err(bs58::decode::Error::BufferTooSmall)
    );
}

#[test]
fn test_decode_invalid_char() {
    let sample = "123456789abcd!efghij";
    let mut output = [0; 32];
    assert_eq!(
        bs58::decode(sample).into(&mut output).unwrap_err(),
        bs58::decode::Error::InvalidCharacter {
            character: '!',
            index: 13
        }
    );
}

#[cfg(feature = "check")]
mod check {
    use super::{cases::CHECK_TEST_CASES, FILLER};

    #[test]
    fn test_decode_without_check_to_slice() {
        for &(val, s) in CHECK_TEST_CASES {
            let mut bytes = FILLER;
            assert_eq!(
                Ok(val.len()),
                bs58::decode(s).with_check(None).into(&mut bytes[..])
            );
            assert_eq!(val, &bytes[..val.len()]);
            assert_eq!(&FILLER[val.len() + 4..], &bytes[val.len() + 4..]);
        }
    }

    #[test]
    fn test_decode_with_check_to_slice() {
        for &(val, s) in &CHECK_TEST_CASES[1..] {
            let mut bytes = FILLER;
            assert_eq!(
                Ok(val.len()),
                bs58::decode(s)
                    .with_check(Some(val[0]))
                    .into(&mut bytes[..])
            );
            assert_eq!(val, &bytes[..val.len()]);
            assert_eq!(&FILLER[val.len() + 4..], &bytes[val.len() + 4..]);
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_decode_without_check_to_vec() {
        for &(val, s) in CHECK_TEST_CASES {
            assert_eq!(
                val.to_vec(),
                bs58::decode(s).with_check(None).into_vec().unwrap()
            );
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_decode_with_check_to_vec() {
        for &(val, s) in &CHECK_TEST_CASES[1..] {
            assert_eq!(
                val.to_vec(),
                bs58::decode(s).with_check(Some(val[0])).into_vec().unwrap()
            );
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_check_ver_failed() {
        assert_eq!(
            Err(bs58::decode::Error::InvalidVersion {
                ver: 49,
                expected_ver: 1,
            }),
            bs58::decode("K5zqBMZZTzUbAZQgrt4")
                .with_check(Some(1))
                .into_vec()
        );
    }
}
