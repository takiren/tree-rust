use std::fs;
use std::path::Path;

pub struct TreeDisplay {
    show_files: bool,
    show_dirs: bool,
    max_depth: Option<usize>,
}

impl TreeDisplay {
    pub fn new(show_files: bool, show_dirs: bool, max_depth: Option<usize>) -> Self {
        Self {
            show_files,
            show_dirs,
            max_depth,
        }
    }

    pub fn display(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if !path.exists() {
            return Err(format!("Path does not exist: {}", path.display()).into());
        }

        println!(
            "{}/",
            path.file_name().unwrap_or_default().to_string_lossy()
        );
        self.display_recursive(path, "", 0)?;
        Ok(())
    }

    fn display_recursive(
        &self,
        dir: &Path,
        prefix: &str,
        depth: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(max_depth) = self.max_depth {
            if depth >= max_depth {
                return Ok(());
            }
        }

        let entries = fs::read_dir(dir)?;
        let mut entries: Vec<_> = entries.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|a| a.file_name());

        for (i, entry) in entries.iter().enumerate() {
            let is_last = i == entries.len() - 1;
            let path = entry.path();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            let is_dir = path.is_dir();
            let should_show = match (self.show_files, self.show_dirs) {
                (true, false) => !is_dir, // files only
                (false, true) => is_dir,  // dirs only
                (false, false) => true,   // default: show both
                (true, true) => true,     // explicitly show both
            };

            if !should_show {
                continue;
            }

            let current_prefix = if is_last { "└── " } else { "├── " };
            let display_name = if is_dir {
                format!("{}/", file_name_str)
            } else {
                file_name_str.to_string()
            };

            println!("{}{}{}", prefix, current_prefix, display_name);

            if is_dir {
                let next_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                self.display_recursive(&path, &next_prefix, depth + 1)?;
            }
        }

        Ok(())
    }
}
