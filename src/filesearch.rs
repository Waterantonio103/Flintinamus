use std::fs::{self, *};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub fn recurse_files(path: impl AsRef<Path> + std::fmt::Debug, recursive: bool) -> std::io::Result<Vec<PathBuf>> {
    let mut buf = vec![];
    let path = path.as_ref();

    if path.is_dir() {
        let entries = match read_dir(path) {
            Ok(entries) => {entries},
            Err(e) if matches!(
                e.kind(),
                ErrorKind::PermissionDenied |
                ErrorKind::NotFound
                ) => {return Ok(buf);}
            Err(e) => {
                eprintln!(
                    "FAILED: {} -> {}",
                    path.display(),
                    e
                );
    
                return Err(e)
            }
        };
    
        for entry in entries {
            let entry = entry?;
            let meta = entry.metadata()?;
    
            if meta.is_dir() && recursive {
                let mut subdir = recurse_files(entry.path(), recursive)?;
                buf.append(&mut subdir);
            }
    
            if meta.is_file() {
                buf.push(entry.path());
            }
        }
    
    } else if path.is_file() {
        return Ok(vec![path.to_path_buf()]);
    }

    Ok(buf)
}