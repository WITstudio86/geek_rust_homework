mod args;
use args::*;

pub use args::{Args, Chacha, Command, TextDecryptArgs, TextEncryptArgs, TextSubCmd};

mod utils;
pub use utils::*;

use enum_dispatch::enum_dispatch;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}

mod process;
