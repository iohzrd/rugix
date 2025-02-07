//! Implementation of the CLI.

use std::path::{Path, PathBuf};

use reportify::ResultExt;
use tracing::info;

use crate::project::{ProjectLoader, ProjectRef};
use crate::BakeryResult;

mod cmds;

pub mod args;

pub(crate) mod status;

/// Run Rugix Bakery with the provided command line arguments.
pub fn run(args: args::Args) -> BakeryResult<()> {
    match &args.cmd {
        args::Command::Bake(cmd) => cmds::run_bake::run(&args, cmd),
        args::Command::Test(cmd) => cmds::run_test::run(&args, cmd),
        args::Command::Run(cmd) => cmds::run_run::run(&args, cmd),
        args::Command::List(cmd) => cmds::run_list::run(&args, cmd),
        args::Command::Pull => cmds::run_pull::run(&args),
        args::Command::Init(cmd) => cmds::run_init::run(cmd),
        args::Command::Shell => cmds::run_shell::run(),
        args::Command::Bundler(cmd) => cmds::run_bundler::run(cmd),
        args::Command::Cache(cmd) => cmds::run_cache::run(&args, cmd),
    }
}

/// Get the current working directory.
fn current_dir() -> BakeryResult<PathBuf> {
    std::env::current_dir().whatever("unable to get current working directory")
}

/// Load the project from the current working directory.
fn load_project(args: &args::Args) -> BakeryResult<ProjectRef> {
    let dev_mode = std::env::var("RUGIX_DEV")
        .map(|dev| dev != "false")
        .unwrap_or(false);
    let bakery_image =
        std::env::var("RUGIX_BAKERY_IMAGE").whatever("unable to determine Docker image")?;
    let image_tag = Path::new("/project/.rugix/docker-image");
    if !dev_mode {
        if image_tag.exists() {
            let cache_image =
                std::fs::read_to_string(image_tag).whatever("unable to read image tag")?;
            if cache_image.trim() != bakery_image.trim() {
                info!("cache is based on older Docker image, deleting `.rugix` directory");
            }
            std::fs::remove_dir_all("/project/.rugix").ok();
        }
    }
    std::fs::create_dir_all("/project/.rugix").whatever("unable to create `.rugix` directory")?;
    std::fs::write(image_tag, bakery_image).whatever("unable to write Docker image tag")?;
    let project_identity =
        std::env::var("RUGIX_HOST_PROJECT_DIR").whatever("unable to determine host directory")?;
    ProjectLoader::current_dir()?
        .with_config_file(args.config.as_deref())
        .with_local_id(project_identity.as_bytes())
        .load()
}
