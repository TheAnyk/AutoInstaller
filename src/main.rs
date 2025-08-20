use std::{fs, path, env, process::Command};
use tinyfiledialogs::message_box_ok;


fn main() {
    let timeout_s: i32 = 10;
    loop {
        check_downloads();
        _ = Command::new("sleep").arg(timeout_s.to_string()).output();
    }
}


fn show_error(error_msg: &str) {
    message_box_ok("AutoInstaller crashed", error_msg, tinyfiledialogs::MessageBoxIcon::Error);
    panic!("{}", error_msg)
}


fn run_command(file: fs::DirEntry) {
    let file_path: String = match file.path().to_str() {
        Some(v) => v.to_owned(),
        None => {
            show_error("Something went wrong while getting full path of a file");
            panic!("Something went wrong while getting full path of a file")
        }
    };

    let query_msg: &str =
        "Found a .deb in Downloads! Please enter your sudo password to install it. The password is not stored or transmitted anywhere.";
    let status: std::process::ExitStatus = match Command::new("gnome-terminal")
        .args(["--wait", "--", "bash", "-c", format!("echo {query_msg}; sudo dpkg -i {file_path} && rm {file_path}").as_str()])
        .status()
    {
        Ok(v) => v,
        Err(e) => {
            show_error(&e.to_string());
            panic!("{}", e)
        }
    };
    if ! status.success() {
        show_error("Something went wrong when installing an update");
    }
}


fn check_downloads() {
    let home: String = match env::var("HOME") {
        Ok(v) => v,
        Err(e) => {
            show_error(&e.to_string());
            panic!("{}", e)
        }
    };
    let downloads: path::PathBuf = [home.as_str(), "Downloads"].iter().collect();

    let files: fs::ReadDir = match fs::read_dir(downloads) {
        Ok(v) => v,
        Err(e) => {
            show_error(&e.to_string());
            panic!("{}", e)
        }
    };
    for file in files {
        let file: fs::DirEntry = match file {
            Ok(v) => v,
            Err(e) => {
                show_error(&e.to_string());
                panic!("{}", e)
            }
        };

        match file.path().extension() {
            Some(ext) => {
                match ext.to_owned().to_str() {
                    Some(ext_str) => {
                        if ext_str == "deb" {
                            run_command(file);
                        };
                    },
                    None => show_error("Something went wrong while getting extension of a file")
                }
            },
            None => continue
        };
    };

}