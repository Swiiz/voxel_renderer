use std::path::Path;

pub fn load_wgsl_with_preprocessor(path: impl AsRef<Path>) -> String {
    std::fs::read_to_string(path.as_ref())
        .unwrap()
        .lines()
        .map(|line| {
            (if line.starts_with("#") {
                if let Some(import_path) = line.strip_prefix("#import ") {
                    let current_dir = path.as_ref().parent().unwrap();
                    let new_path = current_dir.join(import_path.to_owned() + ".wgsl");
                    load_wgsl_with_preprocessor(new_path)
                } else {
                    panic!("Wgsl Preprocessor failed to parse: {}", line);
                }
            } else {
                line.to_owned()
            }) + "\n"
        })
        .collect()
}
