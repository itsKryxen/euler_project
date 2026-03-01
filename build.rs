use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=problems");

    let problems_dir = PathBuf::from("problems");
    let mut files: Vec<PathBuf> = match fs::read_dir(&problems_dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok().map(|x| x.path()))
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("rs"))
            .collect(),
        Err(_) => Vec::new(),
    };

    files.sort_by_key(|p| p.file_name().map(|n| n.to_os_string()));

    let mut out = String::new();
    out.push_str("pub struct Problem { pub name: &'static str, pub solve: fn() }\n");

    for (i, file) in files.iter().enumerate() {
        let abs = fs::canonicalize(file).expect("failed to canonicalize problem file");
        let abs_str = abs.to_string_lossy().replace('\\', "\\\\");
        out.push_str(&format!("#[path = r\"{abs_str}\"] mod problem_{i};\n"));
    }

    out.push_str("pub static PROBLEMS: &[Problem] = &[\n");
    for (i, file) in files.iter().enumerate() {
        let name = file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown_problem");
        out.push_str(&format!(
            "    Problem {{ name: \"{name}\", solve: problem_{i}::solve }},\n"
        ));
    }
    out.push_str("];\n");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    fs::write(out_dir.join("problems_gen.rs"), out).expect("failed to write generated file");
}
