fn main() {
    // Set target feature
    println!("cargo:rustc-cfg=feature=\"+d,+zba,+zbb,+zbc,+zbs\"");

    println!("cargo:rustc-link-arg=-Tmemory.x");
    println!("cargo:rustc-link-arg=-Tdevice.x");
    // Set the linker script to the one provided by cortex-m-rt.
    println!("cargo:rustc-link-arg=-Tlink-fixed.x");

    // Set the extra linker script from defmt
    println!("cargo:rustc-link-arg=-Tdefmt.x");

    println!("cargo:rustc-link-arg=--nmagic");
    // Use flip-link overflow check: https://github.com/knurling-rs/flip-link
    // println!("cargo:rustc-linker=flip-link");
}
