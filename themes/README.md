# Color Themes

Color themes for `pretty-pileup` are specified using a "dotfile" called
`.pretty.toml` (similar to `.bashrc` or `.zshrc` dotfiles).

```bash
cp dracula.toml ${HOME}/.pretty.toml
```

## Config format

As indicated by the file extension, the file
format for specifying color themes in `pretty-pileup` is the 
[TOML format](https://toml.io/en/). There are two main sections that define
the TOML table, the `base_color` section and the `quality_gradient` section.
Within each of these sections you can define the colors that are assigned to
the DNA bases as well as the color range of the gradient used for
representing base quality scores, respectively.

### Example

```toml
# .pretty.toml file: pretty-pileup color theme specification
# Substitute desired color for each <hex color>
[base_color]
base_a = "<hex color>"
base_c = "<hex color>"
base_g = "<hex color>"
base_t = "<hex color>"
base_n = "<hex color>"

[quality_gradient]
low = "<hex color>"
high = "<hex color>"
```

## Config file

There are two options for storing color schemes depending on if you want to set
a global or a local/project-specific color scheme.

 - **Global**: Place the `.pretty.toml` config file in your `HOME` directory.
 - **Local**: Place the `.pretty.toml` file at the root of your project
   directory.
   
