use crate::gen::create;
use crate::FatalError;
use std::path::Path;

use std::io::Write;

pub(crate) fn write(dir: &Path) -> Result<(), FatalError> {
    let files = [
        ("config.rs", include_str!("../../snippets/config.rs")),
        ("error.rs", include_str!("../../snippets/error.rs")),
        ("helpers.rs", include_str!("../../snippets/helpers.rs")),
        ("mod.rs", include_str!("../../snippets/mod.rs")),
        ("traits.rs", include_str!("../../snippets/traits.rs")),
    ];

    for (file_name, data) in files {
        write_file(&dir.join(file_name), data)?;
    }

    Ok(())
}

fn write_file(path: &Path, data: &str) -> Result<(), FatalError> {
    let mut writer = create(path)?;
    writer.write_all(data.as_bytes())?;
    Ok(())
}
