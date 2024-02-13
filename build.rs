use std::env;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;

fn main() {
    if env::var("DOCS_RS").unwrap_or_else(|_| "0".to_string()) == "0" {
        let toolchain_path =
            "/home/build/inkbox/kernel/toolchain/armv7l-linux-musleabihf-cross/bin";

        let mut path_var = match env::var("PATH") {
            Ok(existing_path) => existing_path,
            Err(_) => String::new(),
        };

        path_var.push_str(":");
        path_var.push_str(toolchain_path);

        env::set_var("PATH", &path_var);

        let fbink_dir = "FBInk";
        if std::env::set_current_dir(&fbink_dir).is_err() {
            eprintln!("Failed to enter FBInk directory");
            exit(1);
        }

        // Run 'make distclean'
        let make_distclean_result = Command::new("make").arg("distclean").status();

        if let Ok(exit_status) = make_distclean_result {
            if !exit_status.success() {
                eprintln!(
                    "Failed to execute 'make distclean' with exit code: {:?}",
                    exit_status
                );
                exit(exit_status.code().unwrap_or(1));
            }
        } else {
            eprintln!("Failed to execute 'make distclean'");
            exit(1);
        }

        // Run 'make -j8 static MINIMAL=1 BITMAP=1 OPENTYPE=1 IMAGE=1 CROSS_COMPILE=armv7l-linux-musleabihf-'
        let make_result = Command::new("make")
            .arg("-j8")
            .arg("static")
            .arg("MINIMAL=1")
            .arg("BITMAP=1")
            .arg("OPENTYPE=1")
            .arg("IMAGE=1")
            .arg("CROSS_COMPILE=armv7l-linux-musleabihf-")
            .status();

        if let Ok(exit_status) = make_result {
            if exit_status.success() {
                println!("Make command executed successfully!");
            } else {
                eprintln!("Make command failed with exit code: {:?}", exit_status);
                exit(exit_status.code().unwrap_or(1));
            }
        } else {
            eprintln!("Failed to execute 'make' command");
            exit(1);
        }
    }

    if std::env::set_current_dir("../").is_err() {
        eprintln!("Failed to enter previous dir directory");
        exit(1);
    }

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=FBInk/Release/static/libfbink.a");

    let bindings = bindgen::Builder::default()
        //.clang_args(vec!["--target=armv7l-linux-musleabihf"])
        .clang_arg("--target=armv7l-linux-musleabihf")
        //.default_enum_style(EnumVariation::NewType { is_bitfield: false })
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("out_path: {:?}", out_path);

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
