use std::sync::Arc;

use pumpkin::plugin::Context;
use pumpkin_api_macros::{plugin_impl, plugin_method};

pub mod config;
pub mod directories;
pub mod java;

use directories::setup_directories;
use java::{
    jar::{create_classpath_entries, discover_jar_files},
    jvm::{initialize_jvm, setup_papkin_server},
    resources::{cleanup_stale_files, sync_embedded_resources},
};

async fn on_load_inner(_plugin: &mut MyPlugin, server: Arc<Context>) -> Result<(), String> {
    server.init_log();
    log::info!("Starting Papkin");

    // Setup directories
    let dirs = setup_directories(&server)?;

    // Discover and prepare JAR files
    let jar_paths = discover_jar_files(&dirs.plugins);
    let classpath_entries = create_classpath_entries(&jar_paths);

    // Manage embedded resources
    cleanup_stale_files(&dirs.j4rs);
    sync_embedded_resources(&dirs.j4rs)?;

    // Initialize JVM and Papkin server
    let jvm = initialize_jvm(classpath_entries, &dirs.j4rs)?;
    setup_papkin_server(&jvm)?;

    Ok(())
}

#[plugin_method]
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    on_load_inner(self, server).await
}

#[plugin_impl]
pub struct MyPlugin {}

impl MyPlugin {
    pub fn new() -> Self {
        MyPlugin {}
    }
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self::new()
    }
}
