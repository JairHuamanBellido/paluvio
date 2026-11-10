# Theme architecture

Paluvio is a palette generator. Given one seed color (Primary), it produces a
full theme for **light and dark mode**: 139 color roles, shown as editable
variables in the sidebar.

## Files

| File | Responsibility |
| --- | --- |
| `src/theme/palette.rs` | `PaletteRecipe`: saturation/brightness anchors per mode. `generate(seed, mode)` turns them into colors. |
| `src/theme/variables.rs` | `PaletteState`: one `ColorVariable` per `ColorRole`. Regenerates on seed or mode change. |
| `src/theme/role.rs` | `ColorRole`: enum of the 139 roles. |
| `src/theme/color.rs` | `Hsv`: HSV color type and conversions. |
| `src/theme/mod.rs` | Re-exports, plus `apply()` which writes colors into the live gpui theme. |
| `src/ui/main_view.rs` | Listens to system appearance and the Light/Dark/System toggle, keeps palette in sync. |

## Core idea: recipe, not stored colors

A `PaletteRecipe` stores only two `Anchor`s (saturation, brightness) per mode:
background and foreground. Hue always comes from the seed.

```
seed hue ──┐
           ├─► generate(seed, mode) ─► Palette { background, foreground }
anchors ───┘
```

Light and dark are tuned separately. Dark is not "light inverted": a dark
background needs a visible tint, a light foreground needs near-zero saturation.

## Data flow

```
OS appearance change ─┐
                      ├─► Theme::sync_system_appearance / Theme::change
Toolbar toggle ───────┘            │
                                   ▼
                    Theme::global(cx).mode   (System already resolved)
                                   │
                                   ▼
                    MainView::sync_palette_mode
                                   │
                                   ▼
                    PaletteState::set_mode ─► regenerate()
                                   │
                                   ▼
                    MainView::refresh
                      ├─ update every ColorEditor
                      ├─ theme::apply  (writes live Theme)
                      └─ cx.notify()
```

Key rule: **the palette never decides the mode.** gpui-component owns it
(including the System preference). We read `Theme::global(cx).mode` after every
change and push it into `PaletteState`.

## How a variable gets its value

`PaletteState::regenerate` runs for every variable that is not `customized`:

1. `Primary`: skipped. It is the seed.
2. `Background`, `Foreground`: from `PaletteRecipe::generate`.
3. Every other role: library default for the current mode
   (`ThemeColor::light()` / `ThemeColor::dark()`), via `default_color`.

Step 3 is the temporary fallback. The goal is to replace it, role by role, with
values derived from the recipe (see `adding-roles.md`).

## Customization rules

- Editing a role sets `customized = true`. Regeneration skips it.
- Editing **Primary** changes the seed hue, so all other customizations are
  cleared and everything regenerates.
- Switching mode clears all customizations. Overrides are not stored per mode yet.
- `reset(role)` drops the customization and restores the generated value.
  Primary resets to `#3b82f6`.

## Known limits

- `theme::apply` maps only Background and Foreground to the live theme.
- Overrides are lost when the mode switches.
- Primary has the same value in both modes.
