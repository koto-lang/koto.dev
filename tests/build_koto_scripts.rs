use {glob::glob, koto::Koto, std::fs};

mod build_koto_scripts {
    use super::*;

    #[test]
    fn build_koto_scripts() {
        let mut found_script = false;
        let mut koto = Koto::new();

        for path in glob("**/*.koto").unwrap() {
            let path = path.unwrap();
            let script = fs::read_to_string(&path).unwrap();

            if let Err(error) = koto.compile(&script) {
                panic!("\nError while compiling {}:\n{error}", path.display());
            }

            found_script = true;
        }

        assert!(found_script);
    }
}
