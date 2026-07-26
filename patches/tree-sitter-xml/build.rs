fn main() {
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let target = std::env::var("TARGET").unwrap_or_default();
    if target == "wasm32-wasip2" {
        println!("cargo:rustc-link-lib=static=tree_sitter_xml");
        println!("cargo:rustc-link-search=native={}/lib", manifest);
        println!("cargo:rerun-if-changed=lib/libtree_sitter_xml.a");
        return;
    }

    let mut cfg = cc::Build::new();
    cfg
        .include("dtd/src")
        .include("xml/src");
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "msvc" {
        cfg.flag("-utf-8");
    }
    cfg
        .file("dtd/src/parser.c")
        .file("dtd/src/scanner.c")
        .file("xml/src/parser.c")
        .file("xml/src/scanner.c")
        .compile("tree_sitter_xml");
}
