**purgá** [pʊrˈɡa] - small tool to pass cli arguments to nix flakes



## Try it out

```bash
❯ nix shell github:nikolaiser/purga
❯ purga -a name1=value1 -a argument2=value2 -a foo=bar -- nix run github:nikolaiser/purga-demo

warning: not writing modified lock file of flake 'github:nikolaiser/purga-demo':
• Updated input 'purgaArgs':
    'file:///dev/null?narHash=sha256-d6xi4mKdjkX2JFicDIv5niSzpyI0m/Hnm8GGAIU04kY%3D'
  → 'file:///tmp/tmp.ZQwBmZMMzH?narHash=sha256-r9lbD5KrIdg7a7jN5BvjOER3O%2BRd3diJPlTEbb1%2Bfyk%3D'
name = argument2, value = value2
name = foo, value = bar
name = name1, value = value1
```


## Usage

1. Modify the flake that you want to pass arguments to. By defalt `purga` will use the input called `purgaArgs`, however it can be overridden
```nix
{
  inputs = {
    ...
    purgaArgs = {
      url = "file+file:///dev/null";
      flake = false;
    };
    ...
  };

  outputs = {purgaArgs, ...}: 
    let
    ...
      args = lib.trivial.importJson purgaArgs.outPath;

      # now you can access all arguments by `args.<name>`
    ...
    in 
    {
    ...
    }
}
```
2. Prefix your nix operation (`nix build`, `nixos-rebuild switch` etc) with 
```bash
purga -a argument1=value1 -a argument2=value2 -- <your nix command>
```


##  Reference

```bash
❯ purga --help
Usage: purga [OPTIONS] [--]...

Arguments:
  [--]...  Nix command to call

Options:
  -i, --input <INPUT>  Flake input that will be used to provide the arguments [default: purgaArgs]
  -a, --arg <ARGS>     Key-value pairs to pass to the flake. Format is '--arg name1=value1 --arg name2=value2' If you provide the same key multiple times it will be passed to the flake as an array
  -h, --help           Print help
  -V, --version        Print version
```
