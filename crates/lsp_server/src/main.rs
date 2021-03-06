use actix::System;

use mylang_lsp_server::launch_lsp_server;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let system = System::new();
    system.block_on(async { launch_lsp_server() });

    system.run()?;
    Ok(())
}
