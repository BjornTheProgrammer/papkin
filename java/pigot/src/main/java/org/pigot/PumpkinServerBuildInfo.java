package org.pigot;

import io.papermc.paper.ServerBuildInfo;
import io.papermc.paper.util.JarManifests;
import java.time.Instant;
import java.util.Optional;
import java.util.OptionalInt;
import net.kyori.adventure.key.Key;

public final class PumpkinServerBuildInfo implements ServerBuildInfo {

    Key brandId;

    @Override
    public Key brandId() {
        this.brandId = Key.key("pumpkinmc:paper");
        return brandId;
    }

    @Override
    public boolean isBrandCompatible(Key brandId) {
        return (
            brandId.equals(this.brandId) ||
            brandId.equals(Key.key("papermc:paper"))
        );
    }

    @Override
    public String brandName() {
        return "Pumpkin";
    }

    @Override
    public String minecraftVersionId() {
        return "1.21.10";
    }

    @Override
    public String minecraftVersionName() {
        return "";
    }

    @Override
    public OptionalInt buildNumber() {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException(
            "Unimplemented method 'buildNumber'"
        );
    }

    @Override
    public Instant buildTime() {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException(
            "Unimplemented method 'buildTime'"
        );
    }

    @Override
    public Optional<String> gitBranch() {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException(
            "Unimplemented method 'gitBranch'"
        );
    }

    @Override
    public Optional<String> gitCommit() {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException(
            "Unimplemented method 'gitCommit'"
        );
    }

    @Override
    public String asString(StringRepresentation representation) {
        // TODO Auto-generated method stub
        throw new UnsupportedOperationException(
            "Unimplemented method 'asString'"
        );
    }
}
