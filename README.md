# vkfetch

this is a rewrite of https://github.com/Wunkolo/vkfetch \
I referenced the code and copied the ASCII art from it \
thanks to wunkolo for writing the original \
I rewrote it because I wanted to have vkfetch on an easy to access package manager and wanted to extend it \
I believe we now have feature parity with the original vkfetch + some extra info not present in the original

# installation

## dependencies

### nix

```nix
  environment.systemPackages = [
    pkgs.vulkan-loader
  ];
```

```sh
nix-shell -p vulkan-loader
```

```sh
nix-env -iA nixos.vulkan-loader
```

### ubuntu

```sh
wget -qO- https://packages.lunarg.com/lunarg-signing-key-pub.asc | sudo tee /etc/apt/trusted.gpg.d/lunarg.asc
sudo wget -qO /etc/apt/sources.list.d/lunarg-vulkan-noble.list http://packages.lunarg.com/vulkan/lunarg-vulkan-noble.list
sudo apt update
sudo apt install vulkan-sdk
```
or
```sh
sudo apt update
sudo apt install vulkan-sdk
```

### brew

```sh
brew install vulkan-loader
```

### other distributions

check your own package managers or

https://vulkan.lunarg.com/sdk/home#linux


### windows and mac

https://vulkan.lunarg.com/sdk/home

## vkfetch-rs


```sh
cargo install vkfetch-rs
```

## build it from source

if you use nix then you can use my flake `nix develop`

```sh
git clone https://github.com/float3/vkfetch-rs
cd vkfetch-rs
cargo build
```


# TODO
don't do ansi codes if stdout is a file