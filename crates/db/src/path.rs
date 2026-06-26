use std::path::PathBuf;

pub fn resolve_db_path(filename: &str) -> std::io::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let dir = exe.parent().unwrap().join("db");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join(filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_to_exe_sibling_db_dir() {
        let path = resolve_db_path("test.db").unwrap();
        let exe = std::env::current_exe().unwrap();
        let expected = exe.parent().unwrap().join("db").join("test.db");
        assert_eq!(path, expected);
    }

    #[test]
    fn creates_db_dir_if_missing() {
        let _ = resolve_db_path("probe.db").unwrap();
        let exe = std::env::current_exe().unwrap();
        let dir = exe.parent().unwrap().join("db");
        assert!(dir.exists(), "db dir should exist after resolve_db_path");
    }
}
