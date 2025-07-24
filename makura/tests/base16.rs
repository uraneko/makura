mod encoder {
    use makura::Encode;
    use makura::BASE16;

    #[test]
    fn test0() {
        let input = "";
        let output = b"";

        assert_eq!(input.encode(BASE16), output);
    }

    #[test]
    fn test1() {
        let input = "f";
        let output = b"66";
        

        assert_eq!(input.encode(BASE16), output);
    }

    #[test]
    fn test2() {
        let input = "fo";
        let output = b"666F";
        

        assert_eq!(input.encode(BASE16), output);
    }

    #[test]
    fn test3() {
        let input = "foo";
        let output = b"666F6F";
        

        assert_eq!(input.encode(BASE16), output);
    }

    #[test]
    fn test4() {
        let input = "foob";
        let output = b"666F6F62";
        

        assert_eq!(input.encode(BASE16), output);
    }

    #[test]
    fn test5() {
        let input = "fooba";
        let output = b"666F6F6261";

        assert_eq!(input.encode(BASE16), output);
    }

    #[test]
    fn test6() {
        let input = "foobar";
        let output = b"666F6F626172";

        assert_eq!(input.encode(BASE16), output);
    }
}

mod decoder {
    use makura::BASE16;
    use makura::Bases;
    use makura::Decode;

    #[test]
    fn test0() {
        let input = "";
        let output = "";

        let enc = Bases::deduce_default(input).unwrap();

        assert_eq!(
            str::from_utf8(&output.decode(enc).unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test1() {
        let input = "f";
        let output = "66";

        assert_eq!(
            str::from_utf8(&output.decode(BASE16).unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test2() {
        let input = "fo";
        let output = "666F";

        assert_eq!(
            str::from_utf8(&output.decode(BASE16).unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test3() {
        let input = "foo";
        let output = "666F6F";

        assert_eq!(
            str::from_utf8(&output.decode(BASE16).unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test4() {
        let input = "foob";
        let output = "666F6F62";

        assert_eq!(
            str::from_utf8(&output.decode(BASE16).unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test5() {
        let input = "fooba";
        let output = "666F6F6261";

        assert_eq!(
            str::from_utf8(&output.decode(BASE16).unwrap()).unwrap(),
            input
        );
    }

    #[test]
    fn test6() {
        let input = "foobar";
        let output = "666F6F626172";

        assert_eq!(
            str::from_utf8(&output.decode(BASE16).unwrap()).unwrap(),
            input
        );
    }
}
