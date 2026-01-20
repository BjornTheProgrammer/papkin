package org.papkin;

import org.bukkit.plugin.java.JavaPlugin;

import java.lang.reflect.Constructor;
import java.net.URL;
import java.net.URLClassLoader;
import java.nio.file.Path;
import java.util.jar.JarEntry;
import java.util.jar.JarFile;

public class SimplePaperPluginLoader {

    public JavaPlugin loadPlugin(Path jarPath) throws Exception {
        // 1. Create a URLClassLoader for the plugin JAR
        URL[] urls = { jarPath.toUri().toURL() };
        URLClassLoader classLoader = new URLClassLoader(
            urls,
            getClass().getClassLoader()
        );

        // 2. Read paper-plugin.yml from the JAR
        try (JarFile jarFile = new JarFile(jarPath.toFile())) {
            JarEntry entry = jarFile.getJarEntry("paper-plugin.yml");
            if (entry == null) {
                // Fall back to plugin.yml for Bukkit plugins
                entry = jarFile.getJarEntry("plugin.yml");
            }

            if (entry == null) {
                throw new IllegalArgumentException(
                    "No plugin descriptor found"
                );
            }

            // 3. Parse YAML to get main class
            String mainClass = parseMainClass(jarFile.getInputStream(entry));

            // 4. Load and instantiate using ProviderUtil pattern
            return loadClass(mainClass, JavaPlugin.class, classLoader);
        }
    }

    // Adapted from ProviderUtil
    private <T> T loadClass(
        String clazz,
        Class<T> classType,
        ClassLoader loader
    ) {
        try {
            Class<?> jarClass = Class.forName(clazz, true, loader);
            Class<? extends T> pluginClass = jarClass.asSubclass(classType);
            Constructor<? extends T> constructor =
                pluginClass.getDeclaredConstructor();
            constructor.setAccessible(true);
            return constructor.newInstance();
        } catch (Exception e) {
            throw new RuntimeException(
                "Failed to load plugin class: " + clazz,
                e
            );
        }
    }
}
