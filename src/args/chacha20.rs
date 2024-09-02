use std::{fs, path::Path};

use clap::{Parser, Subcommand};

use crate::{ensure_file_exists, process::EnDeCode, read_input, verify_file, CmdExector};
pub struct Chacha {
    pub key: Vec<u8>,
}

#[derive(Debug, Subcommand)]
pub enum TextSubCmd {
    /// 生成 key
    GenKey(GenKeyArgs),
    /// 加密文本
    Encrypt(TextEncryptArgs),
    /// 解密文本
    Decrypt(TextDecryptArgs),
}

#[derive(Debug, Parser)]
pub struct GenKeyArgs {
    #[arg(short, long, value_parser=ensure_file_exists)]
    pub output: String,
}

#[derive(Debug, Parser)]
pub struct TextEncryptArgs {
    #[arg(short, long, value_parser = verify_file, default_value = "_")]
    pub input: String,
    #[arg(short, long , value_parser = verify_key)]
    pub key: String,
    #[arg(short, long, value_parser=ensure_file_exists)]
    pub output: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptArgs {
    #[arg(short, long, value_parser = verify_file, default_value = "_")]
    pub input: String,
    #[arg(short, long)]
    pub key: String,
    #[arg(short, long, value_parser=ensure_file_exists)]
    pub output: String,
}

impl CmdExector for TextSubCmd {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCmd::GenKey(opts) => {
                let key = Chacha::genkey()?;
                fs::write(&opts.output, key)?;
                eprintln!("🚀 key generated successfully");
                Ok(())
            }
            TextSubCmd::Encrypt(args) => {
                let key = Chacha::readkey(Path::new(&args.key))?;
                let input = read_input(&args.input)?;
                let signer = Chacha::new(&key);
                let result = signer.encode(&input)?;
                fs::write(&args.output, &result)?;
                eprintln!("🚀 encrypt successfully");
                Ok(())
            }
            TextSubCmd::Decrypt(args) => {
                let key = Chacha::readkey(Path::new(&args.key))?;
                let input = read_input(&args.input)?;
                let signer = Chacha::new(&key);
                let output = signer.decode(&input)?;
                dbg!(&output);
                fs::write(&args.output, String::from_utf8(output)?)?;
                eprintln!("🚀 decrypt successfully");
                Ok(())
            }
        }
    }
}

fn verify_key(key: &str) -> anyhow::Result<String> {
    let p = Path::new(key);
    if p.exists() {
        return Ok(key.to_string());
    } else if key.len() != 32 {
        anyhow::bail!("key length must be 32 bytes")
    }
    Ok(key.to_string())
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn test_en_de() {
        // let key = Chacha::genkey().unwrap();
        // let singer = Chacha::new(&key);
        // let encrypted = singer.encode("hello world".as_bytes()).unwrap();
        // let decrypted = singer.decode(&encrypted).unwrap();
        // assert_eq!(
        //     String::from_utf8(decrypted).unwrap(),
        //     "hello world".to_string()
        // );
        let key = Chacha::genkey().unwrap();
        fs::write(Path::new("fixtures/chacha20.key"), &key).unwrap();
        let key = Chacha::readkey(Path::new("fixtures/chacha20.key")).unwrap();
        let input = "hello".as_bytes().to_vec();
        let signer = Chacha::new(&key);
        let result = signer.encode(&input).unwrap();
        fs::write(Path::new("fixtures/chacha20.sign"), &result).unwrap();

        let key = Chacha::readkey(Path::new("fixtures/chacha20.key")).unwrap();
        let input = read_input("fixtures/chacha20.sign").unwrap();
        let signer = Chacha::new(&key);
        let output = signer.decode(&input).unwrap();
        fs::write("fixtures/chacha.txt", &output).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "hello".to_string());
    }
}
