//! The `run` command.

use std::path::Path;
use std::time::Duration;

use reportify::ResultExt;
use rugix_tasks::block_on;
use tempfile::TempDir;
use tracing::info;

use crate::cli::{args, load_project};
use crate::config::tests::SystemConfig;
use crate::tester::qemu;
use crate::{oven, BakeryResult};

/// Run the `run` command.
pub fn run(args: &args::Args, cmd: &args::RunCommand) -> BakeryResult<()> {
    let project = load_project(args)?;

    let output = Path::new("build").join(&cmd.system);
    oven::bake_system(&project, &cmd.release.release_info(), &cmd.system, &output)
        .whatever("error baking image")?;

    let image_path = output.join("system.img");

    let tempdir = TempDir::new().whatever("unable to create temporary directory")?;

    let temp_img = tempdir.path().join("system.img");

    // We copy the image such that new builds do not corrupt the VM.
    std::fs::copy(&image_path, &temp_img).whatever("unable to copy image")?;

    let image_path = temp_img;

    let image_config = project.config().resolve_system_config(&cmd.system)?;
    let system = SystemConfig {
        system: cmd.system.clone(),
        disk_size: None,
        ssh: None,
    };

    block_on(async {
        let _vm = qemu::start(
            image_config.architecture,
            &image_path.to_string_lossy(),
            &system,
        )
        .await?;

        info!("VM started");

        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
        }

        #[expect(unreachable_code)]
        BakeryResult::Ok(())
    })?;

    Ok(())
}
