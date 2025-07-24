mod encoder {
    use makura::Encode;
    use makura::{BASE64, BASE64URL};

    #[test]
    fn test0() {
        let input = "";
        let output = b"";

        assert_eq!(input.encode(BASE64URL), output);
    }

    #[test]
    fn test1() {
        let input = "🍜";
        let output = b"8J-NnA==";

        assert_eq!(input.encode(BASE64URL), output);
    }

    #[test]
    fn test2() {
        let input = "🍔";
        let output = b"8J-NlA==";

        assert_eq!(input.encode(BASE64URL), output);
    }

    #[test]
    fn test3() {
        let input = "🍜🍔?";
        let output = b"8J-NnPCfjZQ_";

        assert_eq!(input.encode(BASE64URL), output)
    }

    #[test]
    #[should_panic]
    fn fail_b64() {
        let input = "🍜";
        let output = b"8J-NnA==";

        assert_eq!(input.encode(BASE64), output);
    }
}

mod decoder {
    use makura::BASE64URL;
    use makura::Bases;
    use makura::Decode;

    // NOTE base64 and base64 url differ at these two char points 62 (+ | -), 63 (/ | _)
    // 64 url can only be tested on an encoded value that contains either - or _

    // NOTE the following 2 tests are redundant due to their logic being implicitly tested in the
    // last 2 tests
    // but that is not to say that thye are useless
    // if the test_encoding tests succeed but the test ones fail
    // it makes it clear that the failure point was not on the encoding deduction point
    // thats why ill keep them
    #[test]
    fn test_encoding0() {
        let input = "8J-NnA==";
        let base = BASE64URL;

        assert_eq!(Bases::default().deduce_encoding(input).unwrap(), base);
    }

    #[test]
    fn test_encoding1() {
        let input = "8J-NnPCfjZQ_";
        let base = BASE64URL;

        assert_eq!(Bases::default().deduce_encoding(input).unwrap(), base);
    }

    #[test]
    fn test1() {
        let input = "🍜";
        let output = "8J-NnA==";

        assert_eq!(
            str::from_utf8(&output.decode_deduce().unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test2() {
        let input = "🍜🍔?";
        let output = "8J-NnPCfjZQ_";

        assert_eq!(
            str::from_utf8(&output.decode_deduce().unwrap()).unwrap(),
            input
        )
    }
}
