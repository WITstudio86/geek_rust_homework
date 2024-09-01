mod chacha20;
pub use chacha20::{Chacha, TextDecryptArgs, TextEncryptArgs, TextSubCmd};

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
    #[command(subcommand)]
    Text(TextSubCmd),
}
