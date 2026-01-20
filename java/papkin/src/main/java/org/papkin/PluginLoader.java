package org.papkin;

import io.papermc.paper.plugin.provider.util.ProviderUtil;
import java.io.File;
import java.net.URL;
import java.net.URLClassLoader;
import org.bukkit.plugin.java.JavaPlugin;

public class PluginLoader {

    public static JavaPlugin load(String jarPath, String mainClass)
        throws Exception {
        File jarFile = new File(jarPath);
        URL[] urls = new URL[] { jarFile.toURI().toURL() };
        URLClassLoader classLoader = new URLClassLoader(
            urls,
            PluginLoader.class.getClassLoader()
        );

        return ProviderUtil.loadClass(mainClass, JavaPlugin.class, classLoader);
    }
}
