#[allow(dead_code)]
extern crate zip;

use std::io;
use std::{fs, path};

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename> ", args[0]);
        return 1;
    }

    let f_name: &path::Path = path::Path::new(&*args[1]);
    let file: fs::File = fs::File::open(&f_name).unwrap();

    let mut archive: zip::ZipArchive<fs::File> = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file: zip::read::ZipFile = archive.by_index(i).unwrap();

        let output: path::PathBuf = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment: &str = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, output.display());

            fs::create_dir_all(&output).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                output.display(),
                file.size()
            );

            if let Some(p) = output.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&output).unwrap();

            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::permissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&output, fs::Permissions::from_mode(value)).unwrap();
            }
        }
    }

    0
}
