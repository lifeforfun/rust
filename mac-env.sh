#OUT_DIR=$HOME/workspace/test/rust/
export VULKAN_SDK=$HOME/workspace/test/sdk-vulkan/macOS
# LD_LIBRARY_PATH或DYLD_LIBRARY_PATH必选一个设置，否则即便rust能找到vulkan动态链接库在macos上也加载不了
#export LD_LIBRARY_PATH=$VULKAN_SDK/lib:$LD_LIBRARY_PATH
export DYLD_LIBRARY_PATH=$VULKAN_SDK/lib:$DYLD_LIBRARY_PATH
#VULKAN_LIB_DIR=$VULKAN_SDK