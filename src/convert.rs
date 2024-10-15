use markdown::to_html;

use crate::{errors::ShellpageError, fs_utils, ConfigFile};

pub fn md_to_html(config: &ConfigFile, file_name: &str) -> Result<String, ShellpageError> {
    let mdfile_path = format!("{}{}.md", config.md_storage, file_name);
    
    let file = fs_utils::read(&mdfile_path)?;

    Ok(to_html(&file))
}
