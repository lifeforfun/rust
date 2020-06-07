fn main() {
    #[cfg(target_os = "windows")]
    {
        //    if let Ok(vulkan_sdk) = std::env::var("VULKAN_SDK") {
        //        println!("cargo:rustc-link-lib=static=vulkan");
        //        println!("cargo:rustc-link-search=native={}/Lib", vulkan_sdk);
        //    }
    }

    #[cfg(target_os = "linux")]
    {
        //    println!("cargo:rustc-link-lib=dylib=vulkan");
    }
    #[cfg(target_os = "macos")]
    {
        //    if let Ok(vulkan_sdk) = std::env::var("VULKAN_SDK") {
        //        println!("cargo:rustc-link-lib=dylib=vulkan");
        //        println!("cargo:rustc-link-search=native={}/lib", vulkan_sdk);
        //    } else {
        //        panic!("unspecified environment `VULKAN_SDK`");
        //    }
    }
}
