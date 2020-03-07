#[cfg(target_os = "windows")]
fn main() {
//    if let Ok(vulkan_sdk) = std::env::var("VULKAN_SDK") {
//        println!("cargo:rustc-link-lib=static=vulkan");
//        println!("cargo:rustc-link-search=native={}/Lib", vulkan_sdk);
//    }
}

#[cfg(target_os = "linux")]
fn main() {
//    println!("cargo:rustc-link-lib=dylib=vulkan");
}

#[cfg(target_os="macos")]
fn main () {
//    if let Ok(vulkan_sdk) = std::env::var("VULKAN_SDK") {
//        println!("cargo:rustc-link-lib=dylib=vulkan");
//        println!("cargo:rustc-link-search=native={}/lib", vulkan_sdk);
//    } else {
//        panic!("unspecified environment `VULKAN_SDK`");
//    }
}