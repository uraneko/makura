mod encoder {
    use makura::Encode;
    use makura::BASE64;

    #[test]
    fn test0() {
        let input = "";
        let output = b"";

        assert_eq!(input.encode(BASE64), output);
    }

    #[test]
    fn test1() {
        let input = "f";
        let output = b"Zg==";

        assert_eq!(input.encode(BASE64), output);
    }

    #[test]
    fn test2() {
        let input = "fo";
        let output = b"Zm8=";

        assert_eq!(input.encode(BASE64), output);
    }

    #[test]
    fn test3() {
        let input = "foo";
        let output = b"Zm9v";

        assert_eq!(input.encode(BASE64), output);
    }

    #[test]
    fn test4() {
        let input = "foob";
        let output = b"Zm9vYg==";

        assert_eq!(input.encode(BASE64), output);
    }

    #[test]
    fn test5() {
        let input = "fooba";
        let output = b"Zm9vYmE=";

        assert_eq!(input.encode(BASE64), output);
    }

    #[test]
    fn test7() {
        let input = "foobar";
        let output = b"Zm9vYmFy";

        assert_eq!(input.encode(BASE64), output);
    }
}

mod decoder {
    use makura::BASE64;
    use makura::Decode;

    #[test]
    fn test0() {
        let input = "";
        let output = "";

        assert_eq!(
            str::from_utf8(&output.decode_deduce().unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test1() {
        let input = "f";
        let output = "Zg==";

        assert_eq!(
            str::from_utf8(&output.decode_deduce().unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test2() {
        let input = "fo";
        let output = "Zm8=";

        assert_eq!(
            str::from_utf8(&output.decode_deduce().unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test3() {
        let input = "foo";
        let output = "Zm9v";

        assert_eq!(
            str::from_utf8(&output.decode_deduce().unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test4() {
        let input = "foob";
        let output = "Zm9vYg==";

        assert_eq!(
            str::from_utf8(&output.decode_deduce().unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test5() {
        let input = "fooba";
        let output = "Zm9vYmE=";

        assert_eq!(
            str::from_utf8(&output.decode_deduce().unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test6() {
        let input = "foobar";
        let output = "Zm9vYmFy";

        assert_eq!(
            str::from_utf8(&output.decode_deduce().unwrap()).unwrap(),
            input
        );
    }
}
