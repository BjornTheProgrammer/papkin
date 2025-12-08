use std::{collections::HashSet, fs, sync::Arc};

use glob::glob;
use j4rs::{ClasspathEntry, InvocationArg, JvmBuilder};
use pumpkin::plugin::Context;
use pumpkin_api_macros::{plugin_impl, plugin_method};
use rust_embed::Embed;

pub mod java;

#[derive(Embed)]
#[folder = "resources/"]
struct Resources;

async fn on_load_inner(_plugin: &mut MyPlugin, server: Arc<Context>) -> Result<(), String> {
    log::info!("Starting Papkin");

    let papkin_folder = server
        .get_data_folder()
        .canonicalize()
        .map_err(|_err| "Failed to get absolute directory from relative")?;
    let mut papkin_plugin_folder = papkin_folder.clone();
    papkin_plugin_folder.push("papkin-plugins");

    let mut papkin_plugin_update_folder = papkin_plugin_folder.clone();
    papkin_plugin_update_folder.push("update");
    fs::create_dir_all(&papkin_plugin_update_folder)
        .map_err(|err| format!("Failed to create plugin folder: {:?}", err))?;

    let mut j4rs_folder = papkin_folder.clone();
    j4rs_folder.push("j4rs");
    let mut jassets = j4rs_folder.clone();
    jassets.push("jassets");
    fs::create_dir_all(&jassets)
        .map_err(|err| format!("Failed to create jassets folder: {:?}", err))?;

    let papkin_plugin_folder = papkin_plugin_folder.to_string_lossy();

    let mut entries = Vec::new();
    for entry in
        glob(&format!("{}/**/*.jar", papkin_plugin_folder)).expect("Failed to read glob pattern")
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

    let allowed: HashSet<String> = Resources::iter().map(|p| p.to_string()).collect();

    for entry in walkdir::WalkDir::new(&j4rs_folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let rel_path = entry
            .path()
            .strip_prefix(&j4rs_folder)
            .unwrap()
            .to_string_lossy()
            .to_string();

        if !allowed.contains(&rel_path) {
            log::warn!("Removing stale j4rs file: {rel_path}");
            let _ = fs::remove_file(entry.path()); // ignore error (locked file, perms, etc)
        }
    }

    for resource_path_str in Resources::iter() {
        let mut resource_path = j4rs_folder.clone();
        resource_path.push(resource_path_str.to_string());
        if !resource_path.exists() {
            let resource = Resources::get(&resource_path_str).unwrap();
            let mut resource_parent = resource_path.clone();
            resource_parent.pop();
            fs::create_dir_all(resource_parent)
                .map_err(|err| format!("Failed to create parent for resource: {:?}", err))?;

            fs::write(resource_path, resource.data)
                .map_err(|err| format!("Failed to add resource: {:?}", err))?;
        } else {
            let resource = Resources::get(&resource_path_str).unwrap();
            let old_resource = fs::read(&resource_path)
                .map_err(|err| format!("Failed to read resource: {:?}", err))?;

            if resource.data == old_resource {
                continue;
            }

            fs::write(resource_path, resource.data)
                .map_err(|err| format!("Failed to add resource: {:?}", err))?;
        }
    }

    let mut paper_jar = j4rs_folder.clone();
    paper_jar.push("paper/paper-server-1.21.10-R0.1-SNAPSHOT.jar");
    let paper_jar = paper_jar.to_string_lossy();
    let paper_jar_entry = ClasspathEntry::new(&paper_jar);

    let jvm = JvmBuilder::new()
        .classpath_entries(entries)
        .classpath_entry(paper_jar_entry)
        .with_base_path(&j4rs_folder.to_string_lossy())
        .build()
        .map_err(|err| format!("jvm failed to init: {:?}", err))?;

    let papkin_server = jvm
        .create_instance("org.papkin.PapkinServer", InvocationArg::empty())
        .map_err(|err| format!("Failed to init plugin: {:?}", err))?;

    jvm.invoke_static(
        "org.bukkit.Bukkit",
        "setServer",
        &[InvocationArg::from(papkin_server)],
    )
    .map_err(|err| format!("Failed to init plugin: {:?}", err))?;

    let options = jvm
        .invoke_static(
            "org.papkin.Options",
            "defaultOptions",
            InvocationArg::empty(),
        )
        .map_err(|err| format!("Failed to init plugin: {:?}", err))?;

    jvm.invoke_static(
        "io.papermc.paper.plugin.PluginInitializerManager",
        "load",
        &[InvocationArg::from(options)],
    )
    .map_err(|err| format!("Failed to init plugin: {:?}", err))?;

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
