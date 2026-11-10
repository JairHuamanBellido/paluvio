# Driving more roles from the recipe

Today only Background and Foreground come from `PaletteRecipe`. The other 137
roles use library defaults. This guide shows how to move a role onto the recipe.

## Idea

Most roles are a point *between* background and foreground: surface, border,
muted text, hover states. One position `t` works in both modes, because each mode
has its own anchors:

- `t = 0.0`: background
- `t = 1.0`: foreground

Only a few roles need per-mode values: accents (Primary variants) and status
colors (Danger, Success, Warning, Info).

## Steps

1. **Pick the role** in `src/theme/role.rs` (for example `ColorRole::Border`).
2. **Add a recipe value** in `src/theme/palette.rs`. Extend `Palette` and
   `generate`:

   ```rust
   pub struct Palette {
       pub background: Hsla,
       pub foreground: Hsla,
       pub border: Hsla, // new
   }
   ```

   Compute it from the two anchors:

   ```rust
   fn mix(a: Anchor, b: Anchor, t: f32) -> Anchor {
       Anchor {
           saturation: a.saturation + (b.saturation - a.saturation) * t,
           brightness: a.brightness + (b.brightness - a.brightness) * t,
       }
   }
   ```

   Use `mix(recipe.background, recipe.foreground, 0.14)` and convert with
   `Hsv::new(seed.h, ..).to_color()`.
3. **Add an arm in both places** in `src/theme/variables.rs`, before the
   fallback: `new()` and `regenerate()`:

   ```rust
   ColorRole::Border => generated.border,
   ```
4. **Map it in `theme::apply`** (`src/theme/mod.rs`) so the live theme uses it:

   ```rust
   theme.colors.border = palette.variable(ColorRole::Border).value.to_color();
   ```
5. **Check both modes.** Toggle Light, Dark and System in the toolbar.

## When `t` should differ per mode

If a role needs different positions (e.g. border 0.14 in light, 0.20 in dark),
store the value per mode in `ModeRecipe` instead of a shared constant.

## Done when

Every arm in `regenerate()` is explicit and the
`role => Self::default_color(...)` fallback and `default_colors` can be deleted.
