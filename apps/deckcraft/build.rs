//! Windows only: embed the app icon and version info (VERSIONINFO) into `fisilab-slides.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `DECKCRAFT_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/fisilab/fisilab-slides.ico");
    println!("cargo:rerun-if-env-changed=DECKCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/fisilab/fisilab-slides.ico")
        .set("ProductName", "Slides")
        .set("FileDescription", "Slides — Apresentações (Suíte FisiLab)")
        .set("CompanyName", "FisiLab — LARFIC/UFRGS")
        .set(
            "LegalCopyright",
            "Baseado no DeckCraft (© ArtCraft Team e contribuidores). Modificações © 2026 FisiLab — LARFIC/UFRGS. MIT OR Apache-2.0.",
        )
        .set("OriginalFilename", "fisilab-slides.exe")
        .set("InternalName", "fisilab-slides");
    if let Err(e) = res.compile() {
        if std::env::var_os("DECKCRAFT_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=fisilab-slides.exe built without icon/version resources: {e}");
    }
}
