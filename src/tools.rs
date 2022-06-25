//! Project Template Manager (Tools)

use std::fs;
use std::io::{self, Read, Seek};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use zip::{result::ZipResult, ZipArchive};

/// Extracts zips files
pub fn extract(reader: impl Read + Seek, outpath: PathBuf) -> ZipResult<()> {
    let mut archive = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => outpath.join(path.to_owned()),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        #[cfg(unix)]
        {
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(())
}
