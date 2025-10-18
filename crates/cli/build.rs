fn main() {
    // This tells cargo to rerun this build script if the templates change
    println!("cargo:rerun-if-changed=../../commands/make/service/_maker_system/skeletons");
}