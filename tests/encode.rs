mod cases;

use cases::TEST_CASES;

const FILLER: [u8; 512] = [b'~'; 512];

#[test]
fn test_encode_to_slice() {
    for &(val, s) in TEST_CASES {
        let mut bytes = FILLER;
        assert_eq!(Ok(s.len()), bs58::encode(val).into(&mut bytes[..]));
        assert_eq!(s.as_bytes(), &bytes[..s.len()]);
        assert_eq!(&FILLER[s.len()..], &bytes[s.len()..]);
    }
}

#[test]
fn test_encode_to_str() {
    for &(val, s) in TEST_CASES {
        let mut bytes = FILLER;
        if !s.is_empty() {
            bytes[(s.len() - 1)..=s.len()].copy_from_slice("Ę".as_bytes());
        }
        let string = core::str::from_utf8_mut(&mut bytes[..]).unwrap();
        assert_eq!(Ok(s.len()), bs58::encode(val).into(string));
        assert_eq!(s.as_bytes(), &bytes[..s.len()]);
        if !s.is_empty() {
            assert_eq!(0, bytes[s.len()]);
        }
        assert_eq!(&FILLER[(s.len() + 1)..], &bytes[(s.len() + 1)..]);
    }
}

#[test]
#[cfg(feature = "alloc")]
fn test_encode_to_vec() {
    for &(val, s) in TEST_CASES {
        assert_eq!(s.as_bytes(), &*bs58::encode(val).into_vec());
    }
}

#[test]
#[cfg(feature = "alloc")]
fn test_encode_to_string() {
    for &(val, s) in TEST_CASES {
        assert_eq!(s, bs58::encode(val).into_string());
    }
}

#[cfg(feature = "check")]
mod check {
    use super::{cases::CHECK_TEST_CASES, FILLER};

    #[test]
    fn test_encode_check_to_slice() {
        for &(val, s) in CHECK_TEST_CASES {
            let mut bytes = FILLER;
            assert_eq!(
                Ok(s.len()),
                bs58::encode(val).with_check().into(&mut bytes[..])
            );
            assert_eq!(s.as_bytes(), &bytes[..s.len()]);
            assert_eq!(&FILLER[s.len()..], &bytes[s.len()..]);

            if !val.is_empty() {
                assert_eq!(
                    Ok(s.len()),
                    bs58::encode(&val[1..])
                        .with_check_version(val[0])
                        .into(&mut bytes[..])
                );
                assert_eq!(s.as_bytes(), &bytes[..s.len()]);
                assert_eq!(&FILLER[s.len()..], &bytes[s.len()..]);
            }
        }
    }

    #[test]
    fn test_encode_check_to_str() {
        for &(val, s) in CHECK_TEST_CASES {
            let mut bytes = FILLER;
            if !s.is_empty() {
                bytes[(s.len() - 1)..=s.len()].copy_from_slice("Ę".as_bytes());
            }
            let string = core::str::from_utf8_mut(&mut bytes[..]).unwrap();
            assert_eq!(Ok(s.len()), bs58::encode(val).with_check().into(string));
            assert_eq!(s.as_bytes(), &bytes[..s.len()]);
            if !s.is_empty() {
                assert_eq!(0, bytes[s.len()]);
            }
            assert_eq!(&FILLER[(s.len() + 1)..], &bytes[(s.len() + 1)..]);
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_encode_check_to_vec() {
        for &(val, s) in CHECK_TEST_CASES {
            assert_eq!(s.as_bytes(), &*bs58::encode(val).with_check().into_vec());
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_encode_check_to_string() {
        for &(val, s) in CHECK_TEST_CASES {
            assert_eq!(s, bs58::encode(val).with_check().into_string());
        }
    }
}
