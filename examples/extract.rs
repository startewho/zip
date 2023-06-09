use std::fs;
use std::io;
use std::path::PathBuf;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <zipfile> <filename> ", args[0]);
        return 1;
    }
   
    let fname = &*args[2];
    let zipfile = fs::File::open(std::path::Path::new(&*args[1])).unwrap();
   
    let mut archive: zip::ZipArchive<fs::File> = zip::ZipArchive::new(zipfile).unwrap();


        let mut file = archive.by_name(fname).unwrap();
        let outpath: PathBuf = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None =>PathBuf::from(fname),
        };
        let comment = file.comment();
            if !comment.is_empty() {
                println!("File {fname} comment: {comment}");
            }
        

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
            1
        } 
        else 
        {
            
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
            1
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = zipfile.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
