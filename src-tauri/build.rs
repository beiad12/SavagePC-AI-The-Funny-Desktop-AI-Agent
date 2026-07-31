fn main() {
    // tauri-build only declares cargo:rerun-if-changed on tauri.conf.json itself, not on
    // the icon files it embeds — so editing an icon without touching that file can leave
    // a stale icon baked into the binary. Force a re-run whenever any icon asset changes.
    println!("cargo:rerun-if-changed=icons");
    tauri_build::build()
}
