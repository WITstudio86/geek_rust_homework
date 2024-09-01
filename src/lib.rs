mod args;
use std::path::Path;

pub use args::{Args, Chacha, Command, TextDecryptArgs, TextEncryptArgs, TextSubCmd};

mod utils;
pub use utils::*;

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait CmdExector {
    fn execute(self) -> anyhow::Result<()>;
}

mod process;

pub trait EnDeCode {
    fn new(key: &[u8]) -> Self;
    fn readkey(path: &Path) -> anyhow::Result<Vec<u8>>;
    fn genkey() -> anyhow::Result<Vec<u8>>;
    fn encode(&self, data: &[u8]) -> anyhow::Result<Vec<u8>>;
    fn decode(&self, data: &[u8]) -> anyhow::Result<Vec<u8>>;
}
