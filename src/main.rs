use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <zip_file> [destination_dir]", args[0]);
        return ExitCode::FAILURE;
    }

    let archive_path = Path::new(&args[1]);
    let target_dir = if args.len() == 3 {
        PathBuf::from(&args[2])
    } else {
        PathBuf::from(".")
    };

    if let Err(err) = extract(archive_path, &target_dir) {
        eprintln!("Error: {err}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn extract(archive_path: &Path, target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(archive_path)
        .map_err(|e| format!("Failed to open '{}': {e}", archive_path.display()))?;
    let reader = BufReader::new(file);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| {
        format!(
            "Failed to read zip archive '{}': {e}",
            archive_path.display()
        )
    })?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let enclosed = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => {
                eprintln!("Warning: Skipping unsafe or invalid path for entry {i}");
                continue;
            }
        };

        let outpath = target_dir.join(enclosed);

        if file.is_dir() {
            println!("Creating directory: \"{}\"", outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;

            println!(
                "Extracted: \"{}\" ({} bytes)",
                outpath.display(),
                file.size()
            );
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                let _ = fs::set_permissions(&outpath, fs::Permissions::from_mode(mode));
            }
        }
    }

    Ok(())
}
