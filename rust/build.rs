use std::{
    collections::HashSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use j4rs::{JvmBuilder, LocalJarArtifact, MavenArtifact, MavenArtifactRepo, MavenSettings};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=rust/src/build.rs");
    println!("cargo::rerun-if-changed=java/build/libs/");
    env_logger::init();

    let dependencies = [
        ("com.google.guava", "guava", "33.3.1-jre"),
        (
            "net.md-5",
            "bungeecord-chat",
            "1.21-R0.2-deprecated+build.21",
        ),
        ("net.kyori", "adventure-text-logger-slf4j", "4.25.0"),
        ("net.kyori", "adventure-text-minimessage", "4.25.0"),
        ("net.kyori", "adventure-text-serializer-legacy", "4.25.0"),
        ("net.kyori", "adventure-text-serializer-plain", "4.25.0"),
        ("net.kyori", "adventure-text-serializer-json", "4.25.0"),
        ("net.kyori", "adventure-api", "4.25.0"),
        ("net.kyori", "adventure-key", "4.25.0"),
        ("net.kyori", "adventure-text-serializer-commons", "4.25.0"),
        ("net.kyori", "adventure-text-serializer-gson", "4.25.0"),
        ("com.google.code.gson", "gson", "2.11.0"),
        ("org.yaml", "snakeyaml", "2.2"),
        ("org.joml", "joml", "1.10.8"),
        ("it.unimi.dsi", "fastutil", "8.5.15"),
        ("org.apache.logging.log4j", "log4j-api", "2.24.1"),
        (
            "org.apache.maven.resolver",
            "maven-resolver-connector-basic",
            "1.9.18",
        ),
        (
            "org.apache.maven.resolver",
            "maven-resolver-transport-http",
            "1.9.18",
        ),
        ("org.apache.maven", "maven-resolver-provider", "3.9.6"),
        ("org.apache.maven.resolver", "maven-resolver-impl", "1.9.18"),
        ("org.slf4j", "jcl-over-slf4j", "1.7.36"),
        (
            "org.apache.maven.resolver",
            "maven-resolver-named-locks",
            "1.9.18",
        ),
        ("org.slf4j", "slf4j-api", "2.0.16"),
        ("com.mojang", "brigadier", "1.3.10"),
        ("org.jspecify", "jspecify", "1.0.0"),
        ("com.google.guava", "failureaccess", "1.0.2"),
        (
            "com.google.guava",
            "listenablefuture",
            "9999.0-empty-to-avoid-conflict-with-guava",
        ),
        ("com.google.code.findbugs", "jsr305", "3.0.2"),
        ("org.checkerframework", "checker-qual", "3.43.0"),
        ("com.google.errorprone", "error_prone_annotations", "2.28.0"),
        ("com.google.j2objc", "j2objc-annotations", "3.0.0"),
        ("org.apache.maven", "maven-model-builder", "3.9.6"),
        ("org.apache.maven", "maven-model", "3.9.6"),
        ("org.apache.maven", "maven-repository-metadata", "3.9.6"),
        ("org.apache.maven.resolver", "maven-resolver-spi", "1.9.18"),
        ("org.apache.maven.resolver", "maven-resolver-util", "1.9.18"),
        ("org.apache.maven.resolver", "maven-resolver-api", "1.9.18"),
        ("org.apache.maven", "maven-artifact", "3.9.6"),
        ("org.codehaus.plexus", "plexus-utils", "3.5.1"),
        ("javax.inject", "javax.inject", "1"),
        ("org.apache.httpcomponents", "httpclient", "4.5.14"),
        ("org.apache.httpcomponents", "httpcore", "4.4.16"),
        ("commons-codec", "commons-codec", "1.16.0"),
        ("net.kyori", "examination-string", "1.3.0"),
        ("net.kyori", "examination-api", "1.3.0"),
        ("org.codehaus.plexus", "plexus-interpolation", "1.26"),
        ("org.apache.maven", "maven-builder-support", "3.9.6"),
        ("org.eclipse.sisu", "org.eclipse.sisu.inject", "0.9.0.M2"),
        ("org.apache.commons", "commons-lang3", "3.12.0"),
        ("net.kyori", "option", "1.1.0"),
        ("io.papermc.paper", "paper-api", "1.21.10-R0.1-SNAPSHOT"),
        ("net.sf.jopt-simple", "jopt-simple", "6.0-alpha-3"),
        ("ca.spottedleaf", "concurrentutil", "0.0.7"),
        ("org.jline", "jline-terminal-ffm", "3.27.1"),
        ("org.jline", "jline-terminal-jni", "3.27.1"),
        ("net.minecrell", "terminalconsoleappender", "1.3.0"),
        ("net.kyori", "adventure-text-logger-slf4j", "4.25.0"),
        ("net.kyori", "adventure-text-minimessage", "4.25.0"),
        ("net.kyori", "adventure-text-serializer-legacy", "4.25.0"),
        ("net.kyori", "adventure-text-serializer-plain", "4.25.0"),
        ("net.kyori", "adventure-text-serializer-gson", "4.25.0"),
        ("net.kyori", "adventure-text-serializer-json", "4.25.0"),
        ("net.kyori", "adventure-api", "4.25.0"),
        ("net.kyori", "adventure-key", "4.25.0"),
        ("net.kyori", "adventure-text-serializer-commons", "4.25.0"),
        ("net.kyori", "adventure-text-serializer-ansi", "4.25.0"),
        ("org.apache.logging.log4j", "log4j-core", "2.24.1"),
        ("com.velocitypowered", "velocity-native", "3.4.0-SNAPSHOT"),
        ("io.netty", "netty-codec-haproxy", "4.1.118.Final"),
        ("org.apache.logging.log4j", "log4j-iostreams", "2.24.1"),
        ("net.neoforged", "AutoRenamingTool", "2.0.3"),
        ("io.papermc", "reflection-rewriter", "0.0.3"),
        ("io.papermc", "reflection-rewriter-proxy-generator", "0.0.3"),
        ("io.papermc", "asm-utils", "0.0.3"),
        ("org.ow2.asm", "asm-commons", "9.8"),
        ("org.spongepowered", "configurate-yaml", "4.2.0"),
        ("com.googlecode.json-simple", "json-simple", "1.1.1"),
        ("net.neoforged", "srgutils", "1.0.9"),
        ("io.papermc", "reflection-rewriter-runtime", "0.0.3"),
        ("me.lucko", "spark-api", "0.1-20240720.200737-2"),
        ("me.lucko", "spark-paper", "1.10.152"),
        ("commons-lang", "commons-lang", "2.6"),
        ("org.xerial", "sqlite-jdbc", "3.49.1.0"),
        ("com.mysql", "mysql-connector-j", "9.2.0"),
        ("com.lmax", "disruptor", "3.4.4"),
        ("com.google.guava", "guava", "33.3.1-jre"),
        (
            "net.md-5",
            "bungeecord-chat",
            "1.21-R0.2-deprecated+build.21",
        ),
        ("net.neoforged.javadoctor", "gson-io", "2.0.17"),
        ("com.google.code.gson", "gson", "2.11.0"),
        ("org.yaml", "snakeyaml", "2.2"),
        ("org.joml", "joml", "1.10.8"),
        ("it.unimi.dsi", "fastutil", "8.5.15"),
        ("org.apache.logging.log4j", "log4j-api", "2.24.1"),
        (
            "org.apache.maven.resolver",
            "maven-resolver-connector-basic",
            "1.9.18",
        ),
        (
            "org.apache.maven.resolver",
            "maven-resolver-transport-http",
            "1.9.18",
        ),
        ("org.apache.maven", "maven-resolver-provider", "3.9.6"),
        ("org.apache.maven.resolver", "maven-resolver-impl", "1.9.18"),
        ("org.slf4j", "jcl-over-slf4j", "1.7.36"),
        (
            "org.apache.maven.resolver",
            "maven-resolver-named-locks",
            "1.9.18",
        ),
        ("org.slf4j", "slf4j-api", "2.0.17"),
        ("com.mojang", "brigadier", "1.3.10"),
        ("org.jspecify", "jspecify", "1.0.0"),
        ("org.jline", "jline-reader", "3.20.0"),
        ("org.jline", "jline-terminal", "3.27.1"),
        ("org.jline", "jline-native", "3.27.1"),
        ("io.netty", "netty-codec", "4.1.118.Final"),
        ("io.netty", "netty-transport", "4.1.118.Final"),
        ("io.netty", "netty-buffer", "4.1.118.Final"),
        ("org.ow2.asm", "asm-tree", "9.8"),
        ("org.ow2.asm", "asm", "9.8"),
        ("org.spongepowered", "configurate-core", "4.2.0"),
        ("net.sf.jopt-simple", "jopt-simple", "6.0-alpha-3"),
        ("net.neoforged.installertools", "cli-utils", "2.1.4"),
        (
            "com.fasterxml.jackson.core",
            "jackson-annotations",
            "2.13.4",
        ),
        ("com.fasterxml.jackson.core", "jackson-core", "2.13.4"),
        ("com.fasterxml.jackson.core", "jackson-databind", "2.13.4.2"),
        ("com.github.oshi", "oshi-core", "6.6.5"),
        ("com.github.stephenc.jcip", "jcip-annotations", "1.0-1"),
        ("com.google.guava", "failureaccess", "1.0.2"),
        ("com.microsoft.azure", "msal4j", "1.17.2"),
        ("com.mojang", "authlib", "7.0.61"),
        ("com.mojang", "datafixerupper", "8.0.16"),
        ("com.mojang", "jtracy", "1.0.36"),
        ("com.mojang", "logging", "1.5.10"),
        ("com.nimbusds", "content-type", "2.3"),
        ("com.nimbusds", "lang-tag", "1.7"),
        ("com.nimbusds", "nimbus-jose-jwt", "9.40"),
        ("com.nimbusds", "oauth2-oidc-sdk", "11.18"),
        ("commons-io", "commons-io", "2.17.0"),
        ("io.netty", "netty-codec-http", "4.1.118.Final"),
        ("io.netty", "netty-resolver", "4.1.118.Final"),
        ("io.netty", "netty-common", "4.1.118.Final"),
        ("io.netty", "netty-handler", "4.1.118.Final"),
        ("io.netty", "netty-transport-classes-epoll", "4.1.118.Final"),
        ("io.netty", "netty-transport-native-epoll", "4.1.118.Final"),
        ("io.netty", "netty-transport-native-epoll", "4.1.118.Final"),
        (
            "io.netty",
            "netty-transport-native-unix-common",
            "4.1.118.Final",
        ),
        ("net.java.dev.jna", "jna", "5.15.0"),
        ("net.java.dev.jna", "jna-platform", "5.15.0"),
        ("net.minidev", "accessors-smart", "2.5.1"),
        ("net.minidev", "json-smart", "2.5.1"),
        ("org.apache.maven", "maven-model-builder", "3.9.6"),
        ("org.apache.maven", "maven-artifact", "3.9.6"),
        ("org.apache.commons", "commons-lang3", "3.17.0"),
        ("org.apache.logging.log4j", "log4j-slf4j2-impl", "2.24.1"),
        ("org.lz4", "lz4-java", "1.8.0"),
        ("io.papermc.parchment.data", "parchment", "1.21.10+build.12"),
        ("com.google.protobuf", "protobuf-java", "4.29.0"),
        (
            "com.google.guava",
            "listenablefuture",
            "9999.0-empty-to-avoid-conflict-with-guava",
        ),
        ("com.google.code.findbugs", "jsr305", "3.0.2"),
        ("org.checkerframework", "checker-qual", "3.43.0"),
        ("com.google.errorprone", "error_prone_annotations", "2.28.0"),
        ("com.google.j2objc", "j2objc-annotations", "3.0.0"),
        ("org.apache.maven", "maven-model", "3.9.6"),
        ("org.apache.maven", "maven-repository-metadata", "3.9.6"),
        ("org.apache.maven.resolver", "maven-resolver-spi", "1.9.18"),
        ("org.apache.maven.resolver", "maven-resolver-util", "1.9.18"),
        ("org.apache.maven.resolver", "maven-resolver-api", "1.9.18"),
        ("org.codehaus.plexus", "plexus-utils", "3.5.1"),
        ("javax.inject", "javax.inject", "1"),
        ("org.apache.httpcomponents", "httpclient", "4.5.14"),
        ("org.apache.httpcomponents", "httpcore", "4.4.16"),
        ("commons-codec", "commons-codec", "1.16.0"),
        ("io.leangen.geantyref", "geantyref", "1.3.16"),
        ("net.kyori", "option", "1.1.0"),
        ("net.neoforged.javadoctor", "spec", "2.0.17"),
        ("net.kyori", "examination-string", "1.3.0"),
        ("net.kyori", "examination-api", "1.3.0"),
        ("net.kyori", "ansi", "1.1.1"),
        ("org.codehaus.plexus", "plexus-interpolation", "1.26"),
        ("org.apache.maven", "maven-builder-support", "3.9.6"),
        ("org.eclipse.sisu", "org.eclipse.sisu.inject", "0.9.0.M2"),
    ];

    let jvm = JvmBuilder::new()
        .with_maven_settings(MavenSettings::new(vec![MavenArtifactRepo::from(
            "papermc::https://repo.papermc.io/repository/maven-public/",
        )]))
        .skip_setting_native_lib()
        .with_base_path(
            &Path::new("./resources")
                .canonicalize()
                .unwrap()
                .to_string_lossy(),
        )
        .build()
        .map_err(|err| format!("jvm failed to init: {:?}", err))
        .unwrap();

    let expected: HashSet<String> = dependencies
        .iter()
        .map(|d| format!("{}-{}.jar", d.1, d.2))
        .collect();

    for entry in fs::read_dir("./resources/jassets").unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name().to_string_lossy().into_owned();

        if file_name.starts_with("j4rs-") {
            continue;
        }

        if !expected.contains(&file_name) {
            let remove_path = entry.path();
            fs::remove_file(remove_path).unwrap();
        }
    }

    if Path::new("./resources/deps").exists() {
        fs::remove_dir_all("./resources/deps").unwrap();
    }

    for dep in dependencies {
        if !Path::new(&format!("./resources/jassets/{}-{}.jar", dep.1, dep.2)).exists() {
            jvm.deploy_artifact(&MavenArtifact::from(format!(
                "{}:{}:{}",
                dep.0, dep.1, dep.2
            )))
            .unwrap();
        }
    }

    if !Path::new("../java/papkin/build/libs/papkin.jar").exists() {
        panic!(
            "Failed to find papkin.jar, build the java library first by running `gradle build` in the java directory!"
        );
    }

    jvm.deploy_artifact(&LocalJarArtifact::new(
        "../java/papkin/build/libs/papkin.jar",
    ))
    .unwrap();

    // jvm.deploy_artifact(&LocalJarArtifact::new(
    //     "../java/libs/paper-server-1.21.10-R0.1-SNAPSHOT.jar",
    // ))
    // .unwrap();
    //

    fs::create_dir_all(Path::new("./resources/paper")).unwrap();

    fs::copy(
        "../java/libs/paper-server-1.21.10-R0.1-SNAPSHOT.jar",
        Path::new("./resources/paper/paper-server-1.21.10-R0.1-SNAPSHOT.jar"),
    )
    .unwrap();

    // let output = Command::new("jar")
    //     .args([
    //         "uf",
    //         "paper-server-1.21.10-R0.1-SNAPSHOT.jar",
    //         "--delete",
    //         "META-INF/services/*",
    //     ])
    //     .current_dir(Path::new("./resources/paper"))
    //     .output()
    //     .unwrap();

    // if !output.status.success() {
    //     eprintln!(
    //         "Failed for reason: {}",
    //         String::from_utf8(output.stderr).unwrap()
    //     );
    //     panic!("Failed to remove services files from paper-server jar");
    // }
    let output = Command::new("zip")
        .args([
            "-d",
            "paper-server-1.21.10-R0.1-SNAPSHOT.jar",
            "META-INF/services/io.papermc.paper.ServerBuildInfo",
        ])
        .current_dir(Path::new("./resources/paper"))
        .output()
        .unwrap();

    if !output.status.success() {
        eprintln!(
            "Failed for reason: {}",
            String::from_utf8(output.stderr).unwrap()
        );
        panic!("Failed to remove services files from paper-server jar");
    }

    let cdylib = std::env::var("CARGO_CDYLIB_FILE_J4RS").unwrap();
    let cdylib = PathBuf::from(cdylib);

    let mut cdylib_to = PathBuf::from("./resources/deps");
    fs::create_dir_all(&cdylib_to).unwrap();

    let original_name = cdylib.file_name().unwrap().to_string_lossy();
    let stem = original_name.split('-').next().unwrap(); // before the first '-'
    let ext = cdylib.extension().unwrap().to_string_lossy();

    cdylib_to.push(format!("{}.{}", stem, ext));

    fs::copy(&cdylib, &cdylib_to)
        .map_err(|err| format!("Failed to copy j4rs native lib: {:?}", err))
        .unwrap();

    Ok(())
}
