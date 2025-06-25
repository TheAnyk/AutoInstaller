use std::{fs, path, env, process::Command};


fn main() {
    let query_msg: &'static str =
        "AutoUpdater requests your sudo password to install updates. The password is never stored or transmitted.";
    let status: std::process::ExitStatus = match Command::new("gnome-terminal")
        .args(["--wait", "--", "bash", "-c", format!("echo {query_msg}; sudo -v").as_str()])
        .status()
    {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };
    if ! status.success() {
        panic!("Sudo authentication failed or canceled");
    }

    let timeout_s: i32 = 10;
    loop {
        check_downloads();
        _ = Command::new("sleep").arg(timeout_s.to_string()).output();
    }
}


fn run_command(file: fs::DirEntry) {
    let file_path: String = match file.path().to_str() {
        Some(v) => v.to_owned(),
        None => panic!("Something went wrong while getting full path of a file")
    };
    _ = Command::new("sh")
        .arg("-c")
        .arg(format!("sudo dpkg -i {f} && rm {f}", f=file_path))
        .output();
}


fn check_downloads() {
    let home: String = match env::var("HOME") {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };
    let downloads: path::PathBuf = [home.as_str(), "Downloads"].iter().collect();

    let files: fs::ReadDir = match fs::read_dir(downloads) {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };
    for file in files {
        let file: fs::DirEntry = match file {
            Ok(v) => v,
            Err(e) => panic!("{}", e)
        };

        match file.path().extension() {
            Some(ext) => {
                match ext.to_owned().to_str() {
                    Some(ext_str) => {
                        if ext_str == "deb" {
                            run_command(file);
                        };
                    },
                    None => panic!("Something went wrong while getting extension of a file")
                }
            },
            None => continue
        };
    };

}