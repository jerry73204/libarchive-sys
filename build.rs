fn main() {
    #[cfg(not(feature = "doc-only"))]
    {
        let library = pkg_config::probe_library("libarchive")
            .expect("unable to locate libarchive on your system. Is it installed?");

        link(&library);

        #[cfg(feature = "generate-bindings")]
        generate_bindings(&library);
    }
}

#[cfg(not(feature = "doc-only"))]
fn link(library: &pkg_config::Library) {
    let mut build = cc::Build::new();

    build.file("csrc/wrapper.c");
    build.includes(&library.include_paths);

    for path in &library.link_paths {
        build.flag(&format!("-L{}", path.display()));
    }

    for name in &library.libs {
        build.flag(&format!("-l{name}"));
    }

    build.compile("archive-sys");
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(library: &pkg_config::Library) {
    use std::{env, fs, path::PathBuf};

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is not set"));

    let builder = bindgen::Builder::default()
        .header("csrc/wrapper.h")
        .allowlist_type("archive_.*")
        .allowlist_function("archive_.*")
        .allowlist_var("ARCHIVE_.*");
    let builder = library.include_paths.iter().fold(builder, |builder, path| {
        builder.clang_arg(format!("-I{}", path.display()))
    });
    let bindings = builder.generate().expect("unable to generate bindings");

    let out_file = out_dir.join("bindings.rs");
    bindings
        .write_to_file(&out_file)
        .unwrap_or_else(|err| panic!("unable to write bindings to {}: {err}", out_file.display()));

    let bindings_file = "src/bindings.rs";
    fs::copy(out_file, bindings_file).expect("unable to write to {bindings_file}");
}
