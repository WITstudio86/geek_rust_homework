use clap::Parser;

use crate::{process::open_file_serve, CmdExector};

#[derive(Debug, Parser)]
pub struct FileServeOpts {
    #[clap(long, default_value = ".")]
    pub path: String,
    #[clap(long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExector for FileServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        open_file_serve(&self.path, self.port).await
    }
}
