mod chacha20;
pub use chacha20::{Chacha, TextDecryptArgs, TextEncryptArgs, TextSubCmd};

mod jwt;
pub use jwt::*;

use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExector)]
pub enum Command {
    /// chacha 对文本进行加解密处理
    #[command(subcommand, name = "text", about = "chacha20 对文本进行加解密处理")]
    Text(TextSubCmd),
    /// jwt 签署
    #[command(subcommand, name = "jwt", about = "jwt 签署")]
    Jwt(JwtSignCmd),
}
