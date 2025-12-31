# Example Themes

This directory contains example theme files demonstrating various theme configurations and use cases.

## Included Examples

### nord-dark.toml
A complete Nord dark theme with all optional colors and properties. Demonstrates:
- Full color palette including optional colors
- Theme properties (border radius, spacing, etc.)
- Variant specification

### nord-light.toml
A light variant of the Nord theme. Shows how to create light variants with appropriate color adjustments.

### dracula.toml
The Dracula theme with vibrant colors. Includes all optional colors and demonstrates a different color palette style.

### tokyo-night.toml
Tokyo Night theme with a modern color scheme. Shows another complete theme example.

### minimal-dark.toml
A minimal theme with only required colors. Useful as a starting point or for themes that don't need optional colors.

## Using Example Themes

### Copy to themes directory

```bash
cp examples/*.toml themes/
```

### Apply a theme

```bash
themectl apply nord-dark
```

### Validate a theme

```bash
themectl validate examples/nord-dark.toml
```

### Preview a theme

```bash
themectl preview nord-dark
```

## Creating Your Own Theme

1. Start with a minimal theme or copy an example
2. Modify colors to match your preferences
3. Validate the theme: `themectl validate your-theme.toml`
4. Test by applying: `themectl apply your-theme --dry-run`
5. Apply: `themectl apply your-theme`

## Theme Format

See [Theme Format Specification](../docs/THEME_FORMAT.md) for detailed documentation on the theme file format.

## Color Resources

- [Coolors](https://coolors.co/) - Color palette generator
- [Adobe Color](https://color.adobe.com/) - Color wheel and palettes
- [Material Design Colors](https://material.io/design/color/the-color-system.html) - Material design color system

## Best Practices

1. **Test contrast**: Ensure background and foreground colors meet WCAG AA standards (4.5:1)
2. **Use descriptive names**: Choose names that clearly indicate the color scheme
3. **Include descriptions**: Help users understand what the theme looks like
4. **Provide variants**: Create both dark and light variants when possible
5. **Validate before sharing**: Always validate themes before committing or sharing
