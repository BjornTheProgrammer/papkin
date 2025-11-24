use std::{fs, path::Path, sync::Arc};

use glob::glob;
use j4rs::{
    ClasspathEntry, InvocationArg, JvmBuilder, MavenArtifact, MavenArtifactRepo, MavenSettings,
};
use pumpkin::plugin::Context;
use pumpkin_api_macros::{plugin_impl, plugin_method};

async fn on_load_inner(_plugin: &mut MyPlugin, server: Arc<Context>) -> Result<(), String> {
    log::info!("Starting Pigot");

    let pigot_folder = server
        .get_data_folder()
        .canonicalize()
        .map_err(|_err| "Failed to get absolute directory from relative")?;
    let mut pigot_plugin_folder = pigot_folder.clone();
    pigot_plugin_folder.push("plugins");
    fs::create_dir_all(&pigot_plugin_folder)
        .map_err(|err| format!("Failed to create plugin folder: {:?}", err))?;

    let mut jassets = pigot_folder.clone();
    jassets.push("jassets");
    fs::create_dir_all(&jassets)
        .map_err(|err| format!("Failed to create jassets folder: {:?}", err))?;

    let mut j4rs_jar_path = jassets.clone();
    j4rs_jar_path.push("j4rs-0.23.1-jar-with-dependencies.jar");

    let pigot_folder = pigot_folder.to_string_lossy();
    let pigot_plugin_folder = pigot_plugin_folder.to_string_lossy();
    let j4rs_jar_path = Path::new(&j4rs_jar_path);

    let mut entries = Vec::new();
    for entry in
        glob(&format!("{}/**/*.jar", pigot_plugin_folder)).expect("Failed to read glob pattern")
    {
        log::info!("jar found: {:?}", entry);
        match entry {
            Ok(inner_path) => match inner_path.canonicalize() {
                Ok(path) => match path.to_str() {
                    Some(path) => entries.push(path.to_string()),
                    None => log::error!("Couldn't convert '{}' into string", inner_path.display()),
                },
                Err(e) => log::error!("Failed to convert path to string: {:?}", e),
            },
            Err(e) => log::error!("Failed to canonicalize path: {:?}", e),
        }
    }

    let entries = entries
        .iter()
        .map(|entry| ClasspathEntry::new(entry))
        .collect::<Vec<_>>();

    if !j4rs_jar_path.exists() {
        let jar = include_bytes!("../resources/jassets/j4rs-0.23.1-jar-with-dependencies.jar");
        fs::write(j4rs_jar_path, jar)
            .map_err(|err| format!("Failed to install j4rs jar: {:?}", err))?;
    }
    log::info!("jassets: {:?}", jassets);

    let jvm = JvmBuilder::new()
        .classpath_entries(entries)
        .skip_setting_native_lib()
        .with_base_path(&pigot_folder)
        .build()
        .map_err(|err| format!("jvm failed to init: {:?}", err))?;

    let plugin_instance = jvm
        .create_instance(
            "net.zhendema.withersurvival.WitherSurvival", // The Java class to create an instance for
            InvocationArg::empty(), // An array of `InvocationArg`s to use for the constructor call - empty for this example
        )
        .map_err(|err| format!("Failed to init plugin: {:?}", err))?;

    log::info!("JVM initialized");

    Ok(())
}

#[plugin_method]
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    pumpkin::init_log!();
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
