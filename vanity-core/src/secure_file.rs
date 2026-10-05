//! Owner-only files for saved private keys.

use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

/// Open `path` for appending, creating it if needed. On Unix the file is
/// created as `0600` and an existing file is tightened to `0600`, so other
/// local users can't read saved keys.
pub fn open_private_append(path: &Path) -> io::Result<File> {
    let mut opts = OpenOptions::new();
    opts.create(true).append(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    let file = opts.open(path)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(std::fs::Permissions::from_mode(0o600))?;
    }
    Ok(file)
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn creates_and_tightens_to_0600() {
        let dir = std::env::temp_dir().join(format!("vanity-secure-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("keys.txt");

        std::fs::write(&path, "old\n").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();

        let mut f = open_private_append(&path).unwrap();
        writeln!(f, "new").unwrap();
        drop(f);

        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "old\nnew\n");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
