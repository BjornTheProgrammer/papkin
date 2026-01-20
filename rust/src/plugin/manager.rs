use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::config::{paper::PaperPluginYml, spigot::SpigotPluginYml};

#[derive(Debug)]
pub enum Plugin {
    Paper(PaperPlugin),
    Spigot(SpigotPlugin),
}

#[derive(Debug)]
pub struct PaperPlugin {
    pub paper_config: PaperPluginYml,
    pub spigot_config: Option<SpigotPluginYml>,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct SpigotPlugin {
    pub spigot_config: SpigotPluginYml,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct PluginManager {
    plugins: Vec<Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Plugin) {
        self.plugins.push(plugin);
    }

    pub fn load_paper_plugin<P: AsRef<Path>>(
        &mut self,
        jar_path: P,
        paper_plugin_config: &str,
        spigot_plugin_config: &Option<String>,
    ) -> Result<()> {
        let parsed_paper_plugin = PaperPluginYml::from_str(paper_plugin_config)?;
        let parsed_spigot_plugin = match spigot_plugin_config {
            Some(config) => Some(SpigotPluginYml::from_str(config)?),
            None => None,
        };

        self.add_plugin(Plugin::Paper(PaperPlugin {
            path: jar_path.as_ref().to_path_buf(),
            paper_config: parsed_paper_plugin,
            spigot_config: parsed_spigot_plugin,
        }));
        Ok(())
    }

    pub fn load_spigot_plugin<P: AsRef<Path>>(
        &mut self,
        jar_path: P,
        spigot_plugin_config: &str,
    ) -> Result<()> {
        let parsed_spigot_plugin = SpigotPluginYml::from_str(spigot_plugin_config)?;

        self.add_plugin(Plugin::Spigot(SpigotPlugin {
            path: jar_path.as_ref().to_path_buf(),
            spigot_config: parsed_spigot_plugin,
        }));
        Ok(())
    }
}
