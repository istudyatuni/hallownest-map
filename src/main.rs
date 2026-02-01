use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

const FILENAME_PREFIX: &str = "part";

fn main() -> Result<()> {
    let Some(arg) = std::env::args().nth(1) else {
        bail!("pass folder as first argument")
    };
    fix_folder(&PathBuf::from(arg)).context("failed to fix folder")
}

fn fix_folder(path: &Path) -> Result<()> {
    for entry in std::fs::read_dir(path).context("failed to read dir")? {
        let entry = entry.context("failed to get dir entry")?;
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let Some(basename) = path.file_stem() else {
            continue;
        };
        let Some(basename) = basename.to_str() else {
            continue;
        };
        if !basename.starts_with(FILENAME_PREFIX) {
            continue;
        }

        let basename = fix_filename(basename);

        let mut new_path = path.with_file_name(basename);
        if let Some(ext) = path.extension() {
            new_path.set_extension(ext);
        }

        std::fs::rename(&path, &new_path).with_context(|| {
            format!(
                "failed to move {} to {}",
                path.display(),
                new_path.display()
            )
        })?;
    }

    Ok(())
}

// [FILENAME_PREFIX]_0_0.999972 -> [FILENAME_PREFIX]_0_1
fn fix_filename(name: &str) -> String {
    let parts = name
        .split("_")
        .map(|p| {
            if p == FILENAME_PREFIX || !p.contains(".") {
                p.to_string()
            } else {
                round_num_str(p)
            }
        })
        .collect::<Vec<_>>();

    parts.join("_")
}

// "0.999972" -> "1"
fn round_num_str(num: &str) -> String {
    let num: f64 = num.parse().expect("should be valid f64");

    (num.round() as u64).to_string()
}
