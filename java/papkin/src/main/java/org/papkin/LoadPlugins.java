package org.papkin;

// import io.papermc.paper.plugin.
import io.papermc.paper.plugin.PluginInitializerManager;
import joptsimple.OptionSet;

public class LoadPlugins {

    public static void loadAllPlugins(OptionSet options) {
        PluginInitializerManager.load(options);
    }
}
