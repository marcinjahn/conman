use std::{path::Path, error::Error, fs};

pub fn is_empty_dir(path: &Path) -> Result<bool, Box<dyn Error>> {
    Ok(path.read_dir()?.next().is_none())
}

pub fn backup_file(file_path: &Path) -> Result<Option<String>, Box<dyn Error>> {
    let mut path_string = file_path.to_str().unwrap().to_owned();
    let mut bak = path_string + ".bak";
    let mut backup_path = Path::new(&bak);

    while backup_path.exists() {
        path_string = backup_path.to_str().unwrap().to_owned();
        bak = path_string + "1";
        backup_path = Path::new(&bak);
    }

    fs::copy(file_path, backup_path)?;

    Ok(Some(backup_path.to_str().unwrap().to_string()))
}
