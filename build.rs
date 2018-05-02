extern crate cc;

use std::env;
use std::path::Path;
use cc::Build;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src = Path::new(&dir).join("quirc").join("lib");
    let fs = &["decode.c", "identify.c", "quirc.c", "version_db.c"];
    let fs = fs.iter().map(|f| src.join(f));
    let features = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or(String::new());

    let mut cc = Build::new();

    cc.include(&src).files(fs);
    if features.contains("quirc_max_regions_65534") {
        cc.flag("-DQUIRC_MAX_REGIONS=65534");
    }

    cc.compile("quirc");
}
