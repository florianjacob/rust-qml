extern crate pkg_config;

use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    let cwd = env::current_dir().unwrap();
    let ext_path = cwd.join("ext/libqmlbind");

    if cfg!(target_os = "macos") {
        let qmake_path = Path::new("/usr/local/opt/qt5/bin/qmake");
        if Path::exists(qmake_path) {
            Command::new(qmake_path).current_dir(&ext_path).output().unwrap_or_else(|e| {
                panic!("Failed to run qmake: {}", e);
            });
            env::set_var("PKG_CONFIG_PATH", Path::new("/usr/local/opt/qt5/lib/pkgconfig"));
        } else {
            panic!("The qmake executable was not found at the expected location ({}) please install it via homebrew.", qmake_path.display());
        }
    } else {
        Command::new("qmake").current_dir(&ext_path).output().unwrap_or_else(|e| {
            panic!("Failed to run qmake: {}", e);
        });
    }


    Command::new("make").current_dir(&ext_path).output().unwrap_or_else(|e| {
        panic!("Failed to run make: {}", e);
    });
                                        // staticlib
    Command::new("make").arg("staticlib").current_dir(&ext_path.join("qmlbind")).output().unwrap_or_else(|e| {
        panic!("Failed to run make: {}", e);
    });

    println!("cargo:rustc-link-lib=static=qmlbind");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-search=native={}", ext_path.join("qmlbind").display());
    pkg_config::find_library("Qt5Core Qt5Gui Qt5Qml Qt5Quick Qt5QuickWidgets").unwrap();
}
