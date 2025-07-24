mod encoder {
    use makura::Encode;
    use makura::BASE32HEX;

    #[test]
    fn test0() {
        let input = "f";
        let output = b"CO======";
        

        assert_eq!(input.encode(BASE32HEX), output);
    }

    #[test]
    fn test1() {
        let input = "fo";
        let output = b"CPNG====";
        

        assert_eq!(input.encode(BASE32HEX), output);
    }

    #[test]
    fn test2() {
        let input = "foo";
        let output = b"CPNMU===";
        

        assert_eq!(input.encode(BASE32HEX), output);
    }

    #[test]
    fn test3() {
        let input = "foob";
        let output = b"CPNMUOG=";
        

        assert_eq!(input.encode(BASE32HEX), output);
    }

    #[test]
    fn test4() {
        let input = "fooba";
        let output = b"CPNMUOJ1";
        

        assert_eq!(input.encode(BASE32HEX), output);
    }

    #[test]
    fn test5() {
        let input = "foobar";
        let output = b"CPNMUOJ1E8======";
        

        assert_eq!(input.encode(BASE32HEX), output);
    }
}

mod decoder {
    use makura::BASE32HEX;
    use makura::Decode;

    #[test]
    fn test0() {
        let input = b"f";
        let output = "CO======";

        assert_eq!(
            output.decode_deduce().unwrap(),
            input
        );
    }

    #[test]
    fn test1() {
        let input = b"fo";
        let output = "CPNG====";

        assert_eq!(
            output.decode_deduce().unwrap(),
            input
        );
    }

    #[test]
    fn test2() {
        let input = b"foo";
        let output = "CPNMU===";

        assert_eq!(
            output.decode(BASE32HEX).unwrap(),
            input
        );
    }

    #[test]
    fn test3() {
        let input = b"foob";
        let output = "CPNMUOG=";

        assert_eq!(
            output.decode_deduce().unwrap(),
            input
        );
    }

    #[test]
    fn test4() {
        let input = b"fooba";
        let output = "CPNMUOJ1";

        assert_eq!(
            output.decode(BASE32HEX).unwrap(),
            input
        );
    }

    #[test]
    // #[should_panic]
    // no longer fails with the new deduce_encoding method of Bases
    fn test4_fail() {
        let input = b"fooba";
        let output = "CPNMUOJ1";
        // println!("{:?}", Decoder::deduce_encoding(output));
        // -> BASE45 // wrong

        assert_eq!(
            output.decode(BASE32HEX).unwrap(),
            input
        );
    }

    #[test]
    fn test5() {
        let input = b"foobar";
        let output = "CPNMUOJ1E8======";

        assert_eq!(
            output.decode_deduce().unwrap(),
            input
        );
    }
}
