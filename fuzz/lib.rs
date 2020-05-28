#[macro_export]
macro_rules! check_regex_same {
    ($regex:expr) => {
        
        use h4x_re::Regex as HRexex;
        use lazy_static::lazy_static;
        use libfuzzer_sys::fuzz_target;
        use regex::Regex as RRegex;

        lazy_static! {
            static ref rr: RRegex = RRegex::new($regex).unwrap();
            static ref hr: HRexex<'static> = HRexex::new($regex);
        }

        fuzz_target!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                assert_eq!(rr.is_match(s), hr.is_match(s));
            }
        });
    };
}
