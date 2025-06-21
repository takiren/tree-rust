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

    /// ディレクトリを再帰的に表示する関数
    /// 
    /// # Arguments
    /// * `dir` - 表示するディレクトリのパス
    /// * `prefix` - 現在の行の接頭辞（ツリー構造の表示用）
    /// * `depth` - 現在の深さ
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - 成功時は`Ok(())`、エラー時はエラー情報
    fn display_recursive(
        &self,
        dir: &Path,
        prefix: &str,
        depth: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 最大深さに達した場合は再帰を停止
        if let Some(max_depth) = self.max_depth {
            if depth >= max_depth {
                return Ok(());
            }
        }

        // ディレクトリの内容を読み込み、ファイル名でソート
        let entries = fs::read_dir(dir)?;
        let mut entries: Vec<_> = entries.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|a| a.file_name());

        // 各エントリを順番に処理
        for (i, entry) in entries.iter().enumerate() {
            let is_last = i == entries.len() - 1; // 最後のエントリかどうか
            let path = entry.path();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            let is_dir = path.is_dir();
            // 表示条件を判定（ファイルのみ、ディレクトリのみ、または両方）
            let should_show = match (self.show_files, self.show_dirs) {
                (true, false) => !is_dir, // ファイルのみ表示
                (false, true) => is_dir,  // ディレクトリのみ表示
                (false, false) => true,   // デフォルト：両方表示
                (true, true) => true,     // 明示的に両方表示
            };

            // 表示条件に合わない場合はスキップ
            if !should_show {
                continue;
            }

            // ツリー構造の表示用プレフィックスを決定
            let current_prefix = if is_last { "└── " } else { "├── " };
            // ディレクトリの場合は末尾にスラッシュを追加
            let display_name = if is_dir {
                format!("{}/", file_name_str)
            } else {
                file_name_str.to_string()
            };

            // 現在のエントリを表示
            println!("{}{}{}", prefix, current_prefix, display_name);

            // ディレクトリの場合は再帰的に処理
            if is_dir {
                // 次のレベル用のプレフィックスを作成（垂直線の継続）
                let next_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                self.display_recursive(&path, &next_prefix, depth + 1)?;
            }
        }

        Ok(())
    }
}
