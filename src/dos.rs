use std::fs;
use std::path::PathBuf;

pub fn create_folders(name: &str) -> std::io::Result<()> {
    let mut root_path = PathBuf::from("/home/ronnyascencio/projects");
    root_path.push(name);

    for folder in ["test1", "test2", "test3"] {
        fs::create_dir_all(root_path.join(folder))?;
        println!("{:?}", root_path.join(folder));
    }
    println!("{:?}", root_path);
    Ok(())
}
