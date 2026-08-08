use std::{env, fs, io, path};

fn main() {
    std::process::exit(extract_from_zip());
}

fn extract_from_zip() -> i32 {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <zip_file>", args[0]);
        return 1;
    }

    for i in 1..args.len() {
        println!("#{}: {}", i, args[i]);
    }

    let filename = path::Path::new(&args[1]);
    let file = fs::File::open(filename);

    if file.is_err() {
        println!("Error opening file: {}", args[1]);
        return 1;
    }

    let archive = zip::ZipArchive::new(file.unwrap());
    if archive.is_err() {
        println!("Error reading zip archive: {}", args[1]);
        return 1;
    }

    let mut archive = archive.unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if (file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();

            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    return 0;
}
