use std::fs;
use std::io;

fn main() {
    // Exit with the return value of `real_main`.
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    // Collect command-line arguments into a vector.
    let args: Vec<_> = std::env::args().collect();

    // If there's no additional argument, provide the usage pattern.
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    // Get the filename provided as an argument.
    let fname = std::path::Path::new(&args[1]);

    // Open the specified file.
    let file = fs::File::open(&fname).unwrap();

    // Create a ZIP archive reader for the file.
    let mut archive = zip::ZipArchive::new(file).unwrap();

    // Iterate over each file in the ZIP archive.
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        // Determine the extraction path for the current file.
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // Print file comments, if they exist.
        if !file.comment().is_empty() {
            println!("File {} comment: {}", i, file.comment());
        }

        // Check if the current file is a directory.
        if file.name().ends_with('/') {
            println!("Directory {} extracted to \"{}\"", i, outpath.display());
            // Create the directory and its parents if they don't exist.
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            // Ensure parent directories exist.
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            // Extract the file content.
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // If on a Unix system, set file permissions.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}
