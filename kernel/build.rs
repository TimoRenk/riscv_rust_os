fn main() {
    println!("cargo:rustc-link-arg=-Tkernel/src/lds/kernel.lds");
}
