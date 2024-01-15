# Arunim's Power Manager (apm)

<!-- TODO: Add screenshot -->

> apm is a simple power menu built with `gtk-layer-shell` and `gtk4`.

## Running

Just use the command `apm` to run.

## Installation

### From source

You will need the following dependencies to build the binary.

- pkg-config
- glib
- gtk4
- gtk4-layer-shell

```sh
cargo build --release
```

If you are using Nix or NixOS with flakes support, then run:

```sh
nix build
```

### Nix/NixOS + Flakes

```sh
nix profile install github:arunim-io/apm
```

<details>
<summary>
You can also include it in your flake config.
</summary>

Example:

```nix
{
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
        apm.url = "github:arunim-io/apm";
    };
}
```

</details>

## Configuration

`apm` is configured using `toml` and is styled using `css`.

### Options

| Name        | Type       | Default Value | Description                          |
| ----------- | ---------- | ------------- | ------------------------------------ |
| spacing     | `integer`  | 25            | The gap between each button          |
| icon_size   | `integer`  | 50            | The size of the icon in the button   |
| icon_margin | `integer`  | 10            | The margin of the icon in the button |
| buttons     | `Button[]` | []            | The buttons to display.              |

#### Button Options

| Name  | Type     | Default Value | Description                                       |
| ----- | -------- | ------------- | ------------------------------------------------- |
| icon  | `string` | 25            | The icon to display                               |
| label | `string` | 50            | The label to show under the button                |
| cmd   | `string` | 10            | The command to execute when the button is called. |
| key   | `string` | []            | The key to use for activating the button.         |

There is an example config in the [`examples/`](examples/) folder.

## License

`apm` is [GNU GPLv3](LICENSE) licensed.