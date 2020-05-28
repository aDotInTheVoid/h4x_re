#![no_main]

use h4x_re_fuzz::check_regex_same;

check_regex_same!("win$");