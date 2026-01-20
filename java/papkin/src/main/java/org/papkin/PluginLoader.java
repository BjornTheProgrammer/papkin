package org.papkin.loader;

import java.io.File;
import java.net.URL;
import java.net.URLClassLoader;
import org.bukkit.plugin.java.JavaPlugin;

public class PluginLoader {

    public static Object load(String jarPath, String mainClass) {
        try {
            File jarFile = new File(jarPath);
            URL[] urls = new URL[] { jarFile.toURI().toURL() };
            URLClassLoader classLoader = new URLClassLoader(
                urls,
                PluginLoader.class.getClassLoader()
            );

            Class<?> pluginClass = classLoader.loadClass(mainClass);
            Object instance = pluginClass
                .getDeclaredConstructor()
                .newInstance();

            // Note: This won't work properly because JavaPlugin
            // requires init() to be called first with proper parameters
            return instance;
        } catch (Exception e) {
            e.printStackTrace();
            return null;
        }
    }
}
