fn main() {
    // To mówi Cargo: "Szukaj bibliotek .lib w folderze /lib"
    println!("cargo:rustc-link-search=native=lib");

    #[cfg(target_os = "windows")]
    {
        // Linkujemy pomocniczą bibliotekę do Delay Load
        println!("cargo:rustc-link-arg=delayimp.lib");
        // Określamy, które DLL mają być ładowane z opóźnieniem
        println!("cargo:rustc-link-arg=/DELAYLOAD:wpcap.dll");
        println!("cargo:rustc-link-arg=/DELAYLOAD:Packet.dll");
    }
}
