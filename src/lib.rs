mod args;
use args::*;

pub use args::{Args, Chacha, Command, TextDecryptArgs, TextEncryptArgs, TextSubCmd};

mod utils;
pub use utils::*;

use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait CmdExector {
    fn execute(self) -> anyhow::Result<()>;
}

mod process;
