use std::{
    env::args, 
    process::exit, 
    path::Path, 
    fs::{
        File, 
        create_dir_all, self
    },
    io::copy
};

use zip::ZipArchive;


fn main() {
    exit(real_main());
}

fn real_main() -> i32 {
    let args : Vec<_>= args().collect();

    if args.len() < 2 {
        eprintln!("Usage : {}, <filename>", args[0]);
        return 1;
    }

    let fname = Path::new(&args[1]);
    let file = File::open(fname).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let comment = file.comment();
        if !comment.is_empty() {
            println!("File : {}, comment : {}", i, comment);
        }

        if file.name().ends_with('/') {
            println!("File {} is extracted to \"{}\" ", i, outpath.display());
            create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} is extracted to \"{}\" {}",
                i,
                outpath.display(),
                file.size()
            );
        }

        if let Some(p) = outpath.parent() {
            if !p.exists() {
                create_dir_all(&p).unwrap();
            }
        }

        let mut outfile = File::create(&outpath).unwrap();
        copy(&mut file, &mut outfile).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, PermissionsExt::from_mode(mode)).unwrap();
            }
        }
    }

    return 0;

}
