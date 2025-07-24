mod encoder {
    use makura::Encode;
    use makura::BASE32;

    #[test]
    fn test0() {
        let input = "";
        let output = b"";
        

        assert_eq!(input.encode(BASE32), output);
    }

    #[test]
    fn test1() {
        let input = "f";
        let output = b"MY======";
        

        assert_eq!(input.encode(BASE32), output);
    }

    #[test]
    fn test2() {
        let input = "fo";
        let output = b"MZXQ====";
        

        assert_eq!(input.encode(BASE32), output);
    }

    #[test]
    fn test3() {
        let input = "foo";
        let output = b"MZXW6===";
        

        assert_eq!(input.encode(BASE32), output);
    }

    #[test]
    fn test4() {
        let input = "foob";
        let output = b"MZXW6YQ=";
        

        assert_eq!(input.encode(BASE32), output);
    }

    #[test]
    fn test5() {
        let input = "fooba";
        let output = b"MZXW6YTB";
        

        assert_eq!(input.encode(BASE32), output);
    }

    #[test]
    fn test6() {
        let input = "foobar";
        let output = b"MZXW6YTBOI======";
        

        assert_eq!(input.encode(BASE32), output);
    }
}

mod decoder {
    use makura::BASE32;
    use makura::Decode;

    #[test]
    fn test0() {
        let input = b"";
        let output = "";
        assert_eq!(
            output.decode_deduce().unwrap(),
            input
        );
    }

    #[test]
    fn test1() {
        let input = b"f";
        let output = "MY======";
        assert_eq!(
            output.decode_deduce().unwrap(),
            input
        );
    }

    #[test]
    fn test2() {
        let input = b"fo";
        let output = "MZXQ====";
        assert_eq!(
            output.decode_deduce().unwrap(),
            input
        );
    }

    #[test]
    fn test3() {
        let input = b"foo";
        let output = "MZXW6===";
        assert_eq!(
            output.decode_deduce().unwrap(),
            input
        );
    }

    #[test]
    fn test4() {
        let input = b"foob";
        let output = "MZXW6YQ=";
        assert_eq!(
            output.decode(BASE32).unwrap(),
            input
        );
    }

    #[test]
    fn test5() {
        let input = b"fooba";
        let output = "MZXW6YTB";
        assert_eq!(
            output.decode(BASE32).unwrap(),
            input
        );
    }

    #[test]
    fn test6() {
        let input = b"foobar";
        let output = "MZXW6YTBOI======";
        assert_eq!(
            output.decode(BASE32).unwrap(),
            input
        );
    }
}
