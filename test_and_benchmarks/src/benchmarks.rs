mod fuz_json_parser {
    extern crate test;

    #[bench]
    fn easy_bench(b: &mut test::Bencher) {
        let tmp = std::fs::read_to_string("test_files/tst5").unwrap();
        b.iter(|| {
            let _ = fuz_json_parser::json_parser::parse(&tmp);
        });
    }

    #[bench]
    fn tst7(b: &mut test::Bencher) {
        let tmp = std::fs::read_to_string("test_files/tst7").unwrap();
        b.iter(|| {
            let _ = fuz_json_parser::json_parse(&tmp);
        });
    }
    #[bench]
    fn large(b: &mut test::Bencher) {
        let tmp = std::fs::read_to_string("test_files/large-file.json").unwrap();
        b.iter(|| {
            let _ = fuz_json_parser::json_parse(&tmp);
        });
    }
}
mod wrapped_fuz_json_parser {
    extern crate test;

    #[bench]
      fn easy_bench(b: &mut test::Bencher) {
        let tmp = std::fs::read_to_string("test_files/tst5").unwrap();
        b.iter(|| {
            let _ = fuz_json_parser::json_parser::wrapped_parse(&tmp);
        });
    }


    #[bench]
    fn tst7(b: &mut test::Bencher) {
        let tmp = std::fs::read_to_string("test_files/tst7").unwrap();
        b.iter(|| {
            let _ = fuz_json_parser::json_parser::wrapped_parse(&tmp);
        });
    }
    #[bench]
    fn large(b: &mut test::Bencher) {
        let tmp = std::fs::read_to_string("test_files/large-file.json").unwrap();
        b.iter(|| {
            let _ = fuz_json_parser::json_parser::wrapped_parse(&tmp);
        });
    }
}

mod json_crate {
    extern crate test;
    #[bench]
    fn easy_bench(b: &mut test::Bencher) {
        let tmp = std::fs::read_to_string("test_files/tst5").unwrap();
        b.iter(|| {
            let _ = json::parse(&tmp);
        });
    }

    #[bench]
    fn tst7(b: &mut test::Bencher) {
        let tmp = std::fs::read_to_string("test_files/tst7").unwrap();
        b.iter(|| {
            let _ = json::parse(&tmp);
        });
    }
    #[bench]
    fn large(b: &mut test::Bencher) {
        let tmp = std::fs::read_to_string("test_files/large-file.json").unwrap();
        b.iter(|| {
            let _ = json::parse(&tmp);
        });
    }
}
