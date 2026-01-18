plugins {
    `java-library`
}

repositories {
    maven {
        name = "papermc"
        url = uri("https://repo.papermc.io/repository/maven-public/")
    }
}

dependencies {
    compileOnly(files("libs/paper-server-1.21.10-R0.1-SNAPSHOT.jar"))
    compileOnly("io.papermc.paper:paper-api:1.21.10-R0.1-SNAPSHOT")
    implementation("net.sf.jopt-simple:jopt-simple:6.0-alpha-3")
}

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(21)
    }
}
