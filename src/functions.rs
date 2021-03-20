use std::env;
use std::path::Path;
use std::process::Command;

use super::paths;

pub fn radula_behave_fetch() {}

pub fn radula_behave_swallow(x: &str) {
    let mut ceras: [&str; 7] = [""; 7];
    let output = Command::new(paths::RADULA_SHELL)
        .args(&[
            paths::RADULA_SHELL_FLAGS,
            &format!(
                ". {} && echo $nom~~$ver~~$cmt~~$url~~$sum~~$cys~~$cnt~~$lic",
                Path::new(&env::var("CERD").unwrap())
                    .join(x)
                    .join("ceras")
                    .to_str()
                    .unwrap(),
            ),
        ])
        .output();

    for (pos, n) in std::string::String::from_utf8_lossy(&output.unwrap().stdout)
        .to_string()
        .trim_end()
        .split("~~")
        .into_iter()
        .enumerate()
    {
        ceras[pos] = n;
        println!("{} :: {:?}", pos, n);
    }
}
