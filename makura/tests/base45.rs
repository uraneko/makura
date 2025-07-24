mod encoder {
    use makura::Encode;
    use makura::BASE45;

    #[test]
    fn test0() {
        let input = "AB";
        let output = b"BB8";
        

        assert_eq!(input.encode(BASE45), output);
    }

    #[test]
    fn test1() {
        let input = "Hello!!";
        let output = b"%69 VD92EX0";

        

        assert_eq!(input.encode(BASE45), output);
    }

    #[test]
    fn test2() {
        let input = "base-45";
        let output = b"UJCLQE7W581";

        

        assert_eq!(input.encode(BASE45), output);
    }
}

mod decoder {
    use makura::BASE45;
    use makura::Decode;

    #[test]
    fn test0() {
        let output = "QED8WEX0";
        let input = b"ietf!";

        assert_eq!(
            output.decode(BASE45).unwrap(),
            input
        );
    }
}
