#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set("ProductName", "warpgui");
    res.set("FileDescription", "warpgui");
    res.set("LegalCopyright", "Copyright (C) 2022");
    res.compile()
        .expect("Failed to run the Windows resource compiler (rc.exe)");
}

#[cfg(not(windows))]
fn main() {}