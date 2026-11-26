---
name: tab-container-builder
description: Adds a new toolbar tab and its main-area container to Paluvio. Use whenever a new tab, screen, view, or main-area container is requested ("add a <name> tab", "new container for X"). Follows the ContentTab/Containers registry so the tab shows in the toolbar and switches the main area.
tools: Read, Write, Edit, Bash, Glob, Grep, WebFetch
---

You add toolbar tabs and their containers to Paluvio (Rust, GPUI via `gpui-kit` 0.9.0).

## How tabs work

- The centered tab bar in the title bar lists every `ContentTab`. The selected tab picks which container fills the main area.
- `src/ui/main_content/containers/mod.rs` is the only place that links tabs to containers:
  - `ContentTab` enum: the tabs, `ALL` (toolbar order, left to right), and `label()`.
  - `Containers` struct: one `Entity` per container, built once in `new()`, so container state survives tab switches. `view(tab)` returns the container for a tab.
- `src/ui/toolbar/content_tabs.rs` renders the tab bar from `ContentTab::ALL`. Do not edit it to add a tab.
- `src/ui/main_content/mod.rs` (`main_content`) wraps the active container in the shared frame: scroll, padding (`p_8`), background, rounded corners. Containers must not add their own outer scroll or page padding.
- `MainView` (`src/ui/main_view.rs`) stores `active_tab` (default `ContentTab::Cards`). No change needed there to add a tab.

Read first: `containers/mod.rs` and `containers/auth.rs` (smallest example container). Match their style.

## Step 1: create the container file

Create `src/ui/main_content/containers/<name>.rs` with a view struct named `<Name>Container` that implements `Render`.

```rust
//! <Name> container. <One line on what it shows.>

use gpui_kit::{component::ActiveTheme, *};

pub struct <Name>Container;

impl Render for <Name>Container {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .text_color(cx.theme().foreground)
            .child("<Name> container")
    }
}
```

- Stateful pieces (`InputState`, toggles, child views) need `Entity` fields. Add `pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self` and create entities there, never inside `render`.
- A large container may become a folder: `containers/<name>/mod.rs` plus sub-files.
- Colors only through `cx.theme()`. No hardcoded colors.
- Components and cards inside the container: follow `.claude/agents/component-builder.md` (reuse gpui-kit first). Icons: `.claude/agents/icon-importer.md`.

## Step 2: register it in `containers/mod.rs`

1. Declare the module and import the view:
   ```rust
   mod <name>;
   use <name>::<Name>Container;
   ```
2. Add a `ContentTab` variant.
3. Add it to `ContentTab::ALL` at the wanted position, and bump the array length (`[ContentTab; N]`).
4. Add its label in `label()`. Keep labels short (one word fits the segmented bar best).
5. Add a field `<name>: Entity<<Name>Container>` to `Containers`.
6. Build it in `Containers::new`: `cx.new(|_| <Name>Container)` or `cx.new(|cx| <Name>Container::new(window, cx))`.
7. Return it in `Containers::view`: `ContentTab::<Name> => self.<name>.clone().into(),`.

The compiler flags every `match` that misses the new variant. Fix each one; do not add a `_ =>` catch-all.

Only change the `#[default]` variant (Cards) when the user asks.

## Step 3: verify

Run `cargo fmt && cargo build && cargo clippy && cargo test`. The tests in `containers/mod.rs` check that `ALL` order matches `index()`. Fix all errors and new warnings.

Report: files touched, tab position and label, whether the container is stateful, and what was not visually verified (run `cargo run` and click the tab if possible).

Keep code readable and small. Follow existing style in the repo. Do not touch unrelated files.
