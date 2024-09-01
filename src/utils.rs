use std::{fs, io::Read, path::Path};

/// 参数为 "_" 的时候返回 "_" , 否则判断并返回文件路径
pub fn verify_file(path: &str) -> anyhow::Result<String> {
    let p = Path::new(path);
    if p.exists() || path == "_" {
        Ok(path.into())
    } else {
        anyhow::bail!("{} not found", path)
    }
}

/// 确保文件存在
pub fn ensure_file_exists(path: &str) -> anyhow::Result<String> {
    let p = Path::new(path);
    if !p.exists() {
        fs::File::create(p)?;
    }
    Ok(path.into())
}

/// 参数为 "_" 的时候从stdin 读取 , 否则从指定文件读取
pub fn read_input(input: &str) -> anyhow::Result<Vec<u8>> {
    if input == "_" {
        let mut context = String::new();
        std::io::stdin().read_to_string(&mut context)?;
        dbg!(&context);
        Ok(context.as_bytes().to_vec())
    } else {
        Ok(fs::read(input)?)
    }
}
