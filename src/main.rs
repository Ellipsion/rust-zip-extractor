use std::fs;
use std::io;
use std::time;

fn main() {
    std::process::exit(unzip());
}

// TODO
// implement output dir

fn unzip() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: program <input-file>");
        return 1;
    }

    let fname = std::path::Path::new(&args[1]);

    if !fname.exists() || !fname.to_str().unwrap().ends_with(".zip") {
        eprintln!("Error: Invalid file. Please provide a valid zip file.");
        return 1;
    }

    println!("Reading file");
    let start = time::Instant::now();
    let file = fs::File::open(fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.enclosed_name().unwrap();

        /*
            file has two similar methods: .name() and .enclosed_name()
            .name returns &str and the later returns &Path
        */

        // * files in the archive can have comments.
        // println!("{:?} : {:?}", file.enclosed_name().unwrap(), file.comment());

        //check if directory
        if file.name().ends_with("/") {
            // create the directory
            fs::create_dir_all(outpath).unwrap();
            println!("Created directory {}", file.name());
        } else {
            //* create file

            // check if parent directory exists
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }

            // create file
            let mut outfile = match fs::File::create(outpath) {
                Ok(f) => f,
                Err(e) => {
                    println!("Error creating file {:?} : {:?}", outpath, e);
                    continue;
                }
            };

            // copy contents to the file
            io::copy(&mut file, &mut outfile).unwrap();
            println!("Successfully created file {}", file.name());
        }

        // for unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(outpath, fs::Permissions::from_mode(mode));
            }
        }
    }
    println!("Succesfully unzipped");
    println!("Elapsed Time: {} ms", start.elapsed().as_millis());
    return 0;
}
