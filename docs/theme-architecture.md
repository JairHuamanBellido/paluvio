# Theme architecture

Paluvio is a palette generator. Given one seed color (Primary), it produces a
full theme for **light and dark mode**: 139 color roles, shown as editable
variables in the sidebar.

## Files

| File | Responsibility |
| --- | --- |
| `src/theme/palette.rs` | Engine: `Rule`, `RuleTable`, `PaletteRecipe`. `generate(seed, mode)` turns rule tables into colors. |
| `src/theme/strategy/mod.rs` | `PaletteStrategy` enum. `recipe()` returns its `PaletteRecipe`. |
| `src/theme/strategy/default/` | Default strategy (tinted surfaces): `light.rs`, `dark.rs`. Base rules for every strategy. |
| `src/theme/strategy/pure/` | Pure strategy (white/black backgrounds): `light.rs`, `dark.rs`, layered over default. |
| `src/theme/state.rs` | `PaletteState`: one `ColorVariable` per `ColorRole`. Regenerates on seed or mode change. |
| `src/theme/role.rs` | `ColorRole`: enum of the 139 roles. `role/label.rs` holds `name()` (UI labels), `role/key.rs` holds `key()` (export keys). |
| `src/theme/color.rs` | `Hsv`: HSV color type and conversions. |
| `src/theme/gpui_bridge.rs` | gpui-component boundary: `apply()` writes colors into the live theme, `default_color(s)` reads library defaults per role. |
| `src/theme/mod.rs` | Module declarations and re-exports only. |
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

## Strategies

One folder per strategy, one file per mode:

```
src/theme/strategy/
├── mod.rs        PaletteStrategy enum, recipe()
├── default/      light.rs, dark.rs   tinted surfaces (base rules)
└── pure/         light.rs, dark.rs   white/black surfaces over default
```

Each mode file exports `RULES: RuleTable`. Each strategy `mod.rs` exports
`LIGHT` / `DARK`: the list of tables applied in order, where a later table
overrides roles of an earlier one. Pure is
`[default::light::RULES, pure::light::RULES]`, so it swaps only the surface
roles.

The toolbar picker (`ui/toolbar/palette_strategy.rs`, left of the theme mode
icon) calls `PaletteState::set_strategy`. Like a Primary change, it clears
every unlocked edit in both modes and regenerates. The choice is saved to
`<config dir>/paluvio/palette-strategy`.

To add a strategy: create `strategy/<name>/{mod,light,dark}.rs`, add a
variant to `PaletteStrategy`, and a `Select*` action in `palette_strategy.rs`.

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
- Edits are stored per mode. An edit made in Light stays in Light, an edit made
  in Dark stays in Dark, and both survive switching modes.
- **Primary** is the seed and is shared by both modes. Editing it changes the
  seed hue, so the customizations of *both* modes are cleared and everything
  regenerates.
- `reset(role)` drops the customization of the active mode only and restores
  the generated value. Primary resets to `#8CC671`.

## Known limits

- `theme::apply` maps only Background and Foreground to the live theme.
- Primary has the same value in both modes.
