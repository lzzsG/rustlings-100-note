use regex::Regex;
use std::fs;
use std::io::{self, Read, Write};
use walkdir::WalkDir;

fn replace_in_file(file_path: &str, re: &Regex, replacement: &str) -> io::Result<()> {
    let mut content = String::new();
    fs::File::open(file_path)?.read_to_string(&mut content)?;

    let result = re.replace_all(&content, replacement).to_string();

    fs::File::create(file_path)?.write_all(result.as_bytes())?;

    Ok(())
}

fn main() -> io::Result<()> {
    let folder_path = "./exercises"; // 请确保这里的路径是正确的

    // let pattern = r"// hint.";

    // let pattern = r"(?s)note.\n\s*hint.";

    let pattern = r"watch subcommand for lzz's note";

    //
    let re = Regex::new(pattern).unwrap();
    // 注意替换文本中的换行符

    let replacement = "watch subcommand for lzz's note.";

    // let replacement = "// Execute `rustlings hint $1` or use the `hint` watch subcommand for a hint.\n// Execute `rustlings note $1` or use the `note $1` watch subcommand for lzz's note.";

    for entry in WalkDir::new(folder_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path().to_str().unwrap();
        println!("Processing: {}", path);
        replace_in_file(path, &re, replacement)?;
    }

    Ok(())
}
