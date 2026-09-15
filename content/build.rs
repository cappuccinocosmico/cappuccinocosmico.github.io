use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR"));
    let blog_dir = manifest_dir.join("blog");
    let recipe_dir = manifest_dir.join("recipies");

    let blog_sources = collect_markdown(&blog_dir);
    let recipe_sources = collect_markdown(&recipe_dir);

    assert!(!blog_sources.is_empty(), "content/blog must contain at least one markdown post");
    assert!(!recipe_sources.is_empty(), "content/recipies must contain at least one markdown recipe");

    println!("cargo:rerun-if-changed={}", blog_dir.display());
    println!("cargo:rerun-if-changed={}", recipe_dir.display());
    for path in blog_sources.iter().chain(recipe_sources.iter()) {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    let mut generated = String::new();
    emit_sources(&mut generated, "BLOG_SOURCES", &blog_sources);
    emit_sources(&mut generated, "RECIPE_SOURCES", &recipe_sources);

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("cargo sets OUT_DIR"));
    fs::write(out_dir.join("embedded_content.rs"), generated).expect("generated manifest must be writable");
}

fn collect_markdown(dir: &Path) -> Vec<PathBuf> {
    let mut sources = fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("content directory {} must be readable: {err}", dir.display()))
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "md"))
        .collect::<Vec<_>>();
    sources.sort();
    assert!(!sources.is_empty(), "content directory {} must contain markdown files", dir.display());
    sources
}

fn emit_sources(out: &mut String, name: &str, sources: &[PathBuf]) {
    out.push_str(&format!("pub static {name}: &[(&str, &str)] = &[\n"));
    for path in sources {
        let slug = path.file_stem().and_then(|stem| stem.to_str()).expect("filenames must be valid utf-8");
        assert!(!slug.is_empty(), "content file slugs must not be empty");
        assert!(
            slug.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
            "content file slugs must be [a-z0-9_-], got: {slug}"
        );
        out.push_str(&format!("    (\"{slug}\", include_str!({:?})),\n", path.canonicalize().expect("content files must exist")));
    }
    out.push_str("];\n");
}
