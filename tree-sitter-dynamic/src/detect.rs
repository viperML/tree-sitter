use std::fs::File;
use std::path::Path;

use eyre::{eyre, ContextCompat};

use crate::PackageJson;

pub fn detect_language<P>(grammar_path: P, file: &Path) -> eyre::Result<String>
where
    P: AsRef<Path>,
{
    let grammar_path = grammar_path.as_ref();
    //let file = file.as_ref();

    let extension = file
        .extension()
        .and_then(|x| x.to_str())
        .wrap_err("Couldn't get extension")?;

    for grammar in grammar_path.read_dir()? {
        let grammar = grammar?.path();

        let package_json = grammar.join("package.json");
        let parsed: PackageJson = serde_json::from_reader(File::open(package_json)?)?;

        for language in parsed.tree_sitter {
            for ft in language.file_types {
                if ft == extension {
                    let language: String = match language.path {
                        Some(p) => p.file_name().unwrap().to_owned().into_string().unwrap(),
                        None => grammar
                            .file_name()
                            .unwrap()
                            .to_owned()
                            .into_string()
                            .unwrap()
                            .replace("tree-sitter-", ""),
                    };

                    return Ok(language);
                }
            }
        }
    }

    return Err(eyre!("Could not find language for .{extension}"));
}
