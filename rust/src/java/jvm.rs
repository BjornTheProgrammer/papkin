use std::path::PathBuf;

use j4rs::{ClasspathEntry, InvocationArg, Jvm, JvmBuilder};

pub fn initialize_jvm(
    classpath_entries: Vec<ClasspathEntry>,
    j4rs_folder: &PathBuf,
) -> Result<Jvm, String> {
    log::info!("Starting the JVM");

    let jvm = JvmBuilder::new()
        .classpath_entries(classpath_entries)
        .with_base_path(j4rs_folder)
        .build()
        .map_err(|err| format!("JVM failed to init: {:?}", err))?;

    log::info!("Started the JVM");

    Ok(jvm)
}

pub fn setup_papkin_server(jvm: &Jvm) -> Result<(), String> {
    let papkin_server = jvm
        .create_instance("org.papkin.PapkinServer", InvocationArg::empty())
        .map_err(|err| format!("Failed to create PapkinServer instance: {:?}", err))?;

    log::info!("After creating instance of server");

    jvm.invoke_static(
        "org.bukkit.Bukkit",
        "setServer",
        &[InvocationArg::from(papkin_server)],
    )
    .map_err(|err| format!("Failed to set Bukkit server: {:?}", err))?;

    log::info!("After setting server");

    Ok(())
}
