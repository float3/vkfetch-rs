# vkfetch

this is a rewrite of https://github.com/Wunkolo/vkfetch \
I referenced the code and took some of the ASCII art from it \
thanks to wunkolo for writing the original \
I rewrote it because I wanted to have vkfetch on an easy to access package manager \
I believe we now have feature parity with the original vkfetch

# vkfetch-rs

you need to have vulkanloader installed
on nix that's the `vulkanloader` package
on ubuntu it looks like this:


```sh
wget -qO- https://packages.lunarg.com/lunarg-signing-key-pub.asc | sudo tee /etc/apt/trusted.gpg.d/lunarg.asc
sudo wget -qO /etc/apt/sources.list.d/lunarg-vulkan-noble.list http://packages.lunarg.com/vulkan/lunarg-vulkan-noble.list
sudo apt update
sudo apt install vulkan-sdk
```

```sh
cargo install vkfetch-rs
```

## build it from source

```sh
git clone https://github.com/float3/vkfetch-rs
cd vkfetch-rs
cargo build
```
