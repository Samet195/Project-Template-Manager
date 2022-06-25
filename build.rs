//! Project Template Manager (Build Script)

const DIRS: &'static str = "./assets/templates";
const ZIPS: &'static str = "./assets/compressed";

use std::fs::{self, File};
use std::io::{Read, Seek, Write};
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use zip::{
    result::{ZipError, ZipResult},
    write::FileOptions,
    CompressionMethod, ZipWriter,
};

fn main() {
    fs::create_dir_all(DIRS).unwrap();
    fs::create_dir_all(ZIPS).unwrap();

    for path in list_dir(DIRS) {
        let dir_name = Path::new(path.iter().last().unwrap().to_str().unwrap());

        let src_dir = path.to_str().unwrap();
        let mut dst_file = PathBuf::new();
        dst_file.push(ZIPS);
        dst_file.push(dir_name);

        match walk_dir(
            src_dir,
            dst_file.to_str().unwrap(),
            CompressionMethod::Deflated,
        ) {
            Ok(_) => {}
            Err(e) => println!("Error: {:?}", e),
        }
    }
    //

    let mut buffer = String::from("//! Project Template Manager (Built-in Data)\n//! This file generated by `build.rs` for automatically embedding built-in templates.\n\n");
    let mut names: Vec<String> = vec![];
    let mut paths: Vec<String> = vec![];

    for path in list_dir(ZIPS) {
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split(".")
            .collect::<Vec<&str>>()[0]
            .to_string();

        names.push(name);
        paths.push(format!("include_bytes!('.{}')", path.display()))
    }

    buffer.push_str(format!("pub const INDEX: [&str; {}] = {:?};\n", names.len(), names).as_str());
    buffer.push_str(
        format!(
            "pub const DATA: [&'static [u8]; {}] = {};\n",
            paths.len(),
            format!("{:?}", paths).replace("\"", "").replace("'", "\"")
        )
        .as_str(),
    );

    fs::write("./src/data.rs", buffer).unwrap();
}

fn list_dir(path: &str) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = fs::read_dir(path)
        .unwrap()
        .map(|res| res.map(|e| e.path()).unwrap())
        .collect();
    entries.sort();
    entries
}

fn walk_dir(src_dir: &str, dst_file: &str, method: CompressionMethod) -> ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}
