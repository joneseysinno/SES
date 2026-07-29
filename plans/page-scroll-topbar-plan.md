# SES — Page Scroll Landmarks & Page Top Bar
## Cursor Implementation Plan

---

## Overview

This document specifies two tightly related features:

1. **Landmark Scroll Bar** — a custom scroll bar on the page area where users can pin named landmark icons to positions on the track, optionally grouping multiple pods under a single landmark. Clicking a landmark jumps the scroll view to that region.

2. **Page Top Bar** — an optional sticky strip at the top of each workspace's page area, persisting above the scrollable pod region. It hosts buttons, info panels, and live flow-channel displays, defined declaratively per workspace.

Both features are additive. No existing `PageNode`, `PageLeaf`, `IoLayout`, or `WorkspaceDef` logic is broken — we extend the data model and add new UI components.

---

## Crate & File Map

```
crates/ses-shell/src/
  page.rs              ← add PageTopBar, TopBarSlot, TopBarSlotKind
  workspace.rs         ← add top_bar: Option<PageTopBar> to WorkspaceDef
  landmark.rs          ← NEW: LandmarkDef, LandmarkGroup, LandmarkId
  ops.rs               ← add landmark ops (add, remove, group, reorder)

crates/ses-ui/src/
  page/
    scroll_bar.rs      ← NEW: LandmarkScrollBar component
    page_area.rs       ← NEW: PageArea wrapper (scroll container + scroll bar)
    top_bar.rs         ← NEW: PageTopBar component (distinct from pod/top_bar.rs)
    leaf.rs            ← no change
    node.rs            ← no change
  screen.rs            ← wire PageArea instead of bare PageNodeView

assets/styles/
  scroll_bar.css       ← NEW: landmark scroll bar styles
  page_top_bar.css     ← NEW: page top bar styles
```

---

## Phase 1 — Shell Data Model

### 1.1 `crates/ses-shell/src/landmark.rs` (new file)

Define the data structures for landmarks. A landmark anchors to a `LeafId` (a pod in the page tree). A group bundles multiple `LeafId`s under one icon.

```rust
use crate::ids::LeafId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Opaque ID for a landmark or landmark group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LandmarkId(pub Uuid);

impl LandmarkId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Visual representation of a landmark on the scroll bar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandmarkIcon {
    /// Short label shown inside the icon (1–3 chars, emoji OK).
    pub label: String,
    /// Optional CSS color token (e.g. "var(--ses-accent)"). None = default.
    pub color: Option<String>,
}

impl LandmarkIcon {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), color: None }
    }
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

/// A single landmark pinned to one or more page leaves.
/// When `leaf_ids` has more than one entry this is a group landmark —
/// the scroll bar shows a bracket spanning all member leaves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LandmarkDef {
    pub id: LandmarkId,
    /// Ordered list of leaf ids covered by this landmark.
    /// Single-leaf landmark → one entry. Group → multiple entries.
    pub leaf_ids: Vec<LeafId>,
    pub icon: LandmarkIcon,
    /// Tooltip shown on hover.
    pub tooltip: Option<String>,
    /// Optional keyboard shortcut index (0-based). None = no shortcut.
    /// UI maps shortcut_index 0 → Alt+1, 1 → Alt+2, etc.
    pub shortcut_index: Option<u8>,
    /// If true, clicking focuses/zooms to fit the landmark's pods.
    pub focus_on_click: bool,
}

impl LandmarkDef {
    pub fn single(leaf_id: LeafId, icon: LandmarkIcon) -> Self {
        Self {
            id: LandmarkId::new(),
            leaf_ids: vec![leaf_id],
            icon,
            tooltip: None,
            shortcut_index: None,
            focus_on_click: false,
        }
    }

    pub fn group(leaf_ids: Vec<LeafId>, icon: LandmarkIcon) -> Self {
        Self {
            id: LandmarkId::new(),
            leaf_ids,
            icon,
            tooltip: None,
            shortcut_index: None,
            focus_on_click: false,
        }
    }
}
```

### 1.2 `crates/ses-shell/src/page.rs` — add `PageTopBar`

Append to the bottom of the existing file (no existing types are changed).

```rust
/// Kind of slot that can live in the page top bar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TopBarSlotKind {
    /// A plain clickable button with a label. Action is identified by string
    /// so the UI layer can dispatch it (e.g. "export", "run-all").
    Button { label: String, action_id: String },

    /// A read-only text panel. Content is a static string set by the author.
    Label { text: String },

    /// A live panel bound to a flow channel. Displays the channel's current
    /// FlowValue, updating reactively. Optionally shows the channel name.
    FlowDisplay {
        channel: String,
        show_channel_name: bool,
    },

    /// A visual separator (vertical rule).
    Separator,
}

/// Alignment of a slot within the top bar flex row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopBarAlign {
    Left,
    Center,
    Right,
}

/// One slot in the page top bar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopBarSlot {
    pub kind: TopBarSlotKind,
    pub align: TopBarAlign,
}

impl TopBarSlot {
    pub fn left(kind: TopBarSlotKind) -> Self {
        Self { kind, align: TopBarAlign::Left }
    }
    pub fn center(kind: TopBarSlotKind) -> Self {
        Self { kind, align: TopBarAlign::Center }
    }
    pub fn right(kind: TopBarSlotKind) -> Self {
        Self { kind, align: TopBarAlign::Right }
    }
}

/// Discrete height sizes for the page top bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopBarHeight {
    Compact, // 28px
    Standard, // 36px
    Tall, // 52px
}

impl TopBarHeight {
    pub fn px(self) -> u32 {
        match self {
            Self::Compact => 28,
            Self::Standard => 36,
            Self::Tall => 52,
        }
    }
}

/// Optional sticky top bar for a workspace page area.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageTopBar {
    pub visible: bool,
    pub height: TopBarHeight,
    pub slots: Vec<TopBarSlot>,
}

impl PageTopBar {
    pub fn new() -> Self {
        Self {
            visible: true,
            height: TopBarHeight::Standard,
            slots: Vec::new(),
        }
    }
    pub fn with_slot(mut self, slot: TopBarSlot) -> Self {
        self.slots.push(slot);
        self
    }
}
```

### 1.3 `crates/ses-shell/src/workspace.rs` — extend `WorkspaceDef`

Add two new fields to `WorkspaceDef`. Because both are `Option`, existing serialized workspaces deserialize cleanly (Serde fills `None`).

```rust
// Inside WorkspaceDef struct — add these two fields:
pub top_bar: Option<PageTopBar>,
pub landmarks: Vec<LandmarkDef>,
```

Add constructor helpers:

```rust
impl WorkspaceDef {
    // existing new() is unchanged; add:

    pub fn with_top_bar(mut self, bar: PageTopBar) -> Self {
        self.top_bar = Some(bar);
        self
    }

    pub fn add_landmark(&mut self, lm: LandmarkDef) {
        self.landmarks.push(lm);
    }

    pub fn remove_landmark(&mut self, id: LandmarkId) {
        self.landmarks.retain(|lm| lm.id != id);
    }
}
```

### 1.4 `crates/ses-shell/src/ops.rs` — landmark operations

Add these pure functions (no UI coupling):

```rust
/// Add a single-leaf landmark to the active workspace.
pub fn add_landmark(ws: &mut WorkspaceDef, leaf_id: LeafId, icon: LandmarkIcon) -> LandmarkId {
    let lm = LandmarkDef::single(leaf_id, icon);
    let id = lm.id;
    ws.landmarks.push(lm);
    id
}

/// Group existing landmark ids into a new group landmark, removing the originals.
pub fn group_landmarks(
    ws: &mut WorkspaceDef,
    landmark_ids: &[LandmarkId],
    icon: LandmarkIcon,
) -> Option<LandmarkId> {
    let leaf_ids: Vec<LeafId> = ws
        .landmarks
        .iter()
        .filter(|lm| landmark_ids.contains(&lm.id))
        .flat_map(|lm| lm.leaf_ids.clone())
        .collect();
    if leaf_ids.is_empty() {
        return None;
    }
    ws.landmarks.retain(|lm| !landmark_ids.contains(&lm.id));
    let group = LandmarkDef::group(leaf_ids, icon);
    let id = group.id;
    ws.landmarks.push(group);
    Some(id)
}

/// Remove a landmark by id.
pub fn remove_landmark(ws: &mut WorkspaceDef, id: LandmarkId) {
    ws.landmarks.retain(|lm| lm.id != id);
}
```

### 1.5 `crates/ses-shell/src/lib.rs` — re-exports

Add to the existing pub use block:

```rust
pub mod landmark;
pub use landmark::{LandmarkDef, LandmarkIcon, LandmarkId};
pub use page::{PageTopBar, TopBarAlign, TopBarHeight, TopBarSlot, TopBarSlotKind};
pub use ops::{add_landmark, group_landmarks, remove_landmark, /* existing... */};
```

---

## Phase 2 — UI Components

### 2.1 `crates/ses-ui/src/page/page_area.rs` (new file)

`PageArea` is the new root wrapper that replaces the bare `PageNodeView` in `screen.rs`. It owns the scrollable pod region and composes in the landmark scroll bar and the optional page top bar.

**Responsibilities:**
- Renders `PageTopBar` (if `ws.top_bar.is_some()`) as a sticky non-scrolling strip
- Renders the pod content area (`PageNodeView`) in a scrollable `div`
- Renders `LandmarkScrollBar` overlaid on the right edge of the scroll area
- Exposes a scroll-position signal that `LandmarkScrollBar` reads to move its thumb

**Rough structure (pseudo-Dioxus):**

```
div.ses-page-area (flex-col, fills workspace-area)
  ├── PageTopBarView  [sticky, height from top_bar.height.px()]
  └── div.ses-page-scroll-host (flex-row, flex:1, overflow:hidden)
        ├── div.ses-page-scroll-content (flex:1, overflow-y:scroll, scroll events → signal)
        │     └── PageNodeView { node, path: [] }
        └── LandmarkScrollBar { landmarks, scroll_pos, total_height }
```

Key signal: `scroll_fraction: Signal<f32>` — updated by `onscroll` on the content div. Derived as `scroll_top / (scroll_height - client_height)`.

### 2.2 `crates/ses-ui/src/page/scroll_bar.rs` (new file)

**`LandmarkScrollBar` component**

Props:
```rust
landmarks: Vec<LandmarkDef>,
scroll_fraction: Signal<f32>,     // read: where are we now
on_jump: EventHandler<f32>,       // write: jump to this fraction
leaf_positions: HashMap<LeafId, f32>, // fraction 0.0–1.0 of each leaf's top edge
```

**Visual layers (bottom to top in z-order):**

1. **Track** — a thin vertical line (`6px` wide), full height, `var(--ses-border)` color
2. **Thumb** — a rounded rectangle positioned by `scroll_fraction`, `var(--ses-text-dim)` color, draggable
3. **Group brackets** — for each multi-leaf landmark, a colored side bar spanning from the top leaf's fraction to the bottom leaf's fraction. Color comes from `landmark.icon.color`
4. **Landmark icons** — circular or pill-shaped badges sitting at the top of their anchor leaf's fraction position. Display `landmark.icon.label`. On hover, show tooltip. On click, call `on_jump` with the leaf's fraction

**Hover expand behavior:**
- Default width: `12px` (just the track and thumb)
- On hover: expand to `36px` to reveal landmark icons. Transition: `width 120ms ease`
- This ensures the scroll bar doesn't compete visually when idle

**Click behavior:**
- Click landmark icon → call `on_jump(leaf_fraction)`. `PageArea` listens and sets `scrollTop` on the content div.
- If `landmark.focus_on_click` is true → additionally dispatch a shell signal to temporarily maximize/zoom those leaves

**Keyboard shortcuts:**
- `PageArea` registers `Alt+{n}` listeners. For each landmark with `shortcut_index = Some(n)`, fire `on_jump` with that landmark's fraction

**Landmark position calculation:**
- `leaf_positions` is computed in `PageArea` after the content div mounts, by measuring the `offsetTop` of each `PageLeafView` relative to the scroll container. Re-measured on layout changes.
- For a multi-leaf group, the icon sits at the minimum fraction among member leaves; the bracket extends to the maximum fraction

### 2.3 `crates/ses-ui/src/page/top_bar.rs` (new file)

**`PageTopBarView` component**

Props:
```rust
bar: PageTopBar,
on_action: EventHandler<String>,  // dispatched when a Button slot is clicked
```

Renders a `div.ses-page-top-bar` with `height` set from `bar.height.px()`.

Internally, split into three sub-divs (left / center / right flex zones). Slots with `TopBarAlign::Left` go in the left zone, etc.

Slot rendering per `TopBarSlotKind`:

- **Button** → `<button class="ses-topbar-btn" onclick=...>{label}</button>`. On click, calls `on_action(action_id)`.
- **Label** → `<span class="ses-topbar-label">{text}</span>`
- **FlowDisplay** → reads `use_flow()`, subscribes to `channel`. Renders `<div class="ses-topbar-flow">` containing a `BindingIndicator` (reuse existing) + the current value text. Shows channel name beneath if `show_channel_name` is true.
- **Separator** → `<div class="ses-topbar-sep" />`

Overflow: if the bar is too narrow, slots overflow into a `…` button that opens a dropdown (standard overflow menu pattern — implementation can be a simple `use_signal` toggle on a `div.ses-topbar-overflow`).

### 2.4 `crates/ses-ui/src/screen.rs` — wire `PageArea`

Change:

```rust
// Before:
PageNodeView { node, path: vec![] }

// After:
PageArea { node, top_bar: ws.top_bar.clone(), landmarks: ws.landmarks.clone() }
```

---

## Phase 3 — Styling

### 3.1 `assets/styles/scroll_bar.css`

```css
/* Landmark scroll bar */

.ses-page-scroll-host {
  position: relative;
  flex: 1;
  min-height: 0;
  display: flex;
}

.ses-page-scroll-content {
  flex: 1;
  min-width: 0;
  overflow-y: scroll;
  scrollbar-width: none; /* hide native scrollbar — our custom bar replaces it */
}

.ses-page-scroll-content::-webkit-scrollbar {
  display: none;
}

.ses-landmark-bar {
  position: relative;
  width: 12px;
  flex-shrink: 0;
  transition: width 120ms ease;
  background: transparent;
  cursor: default;
}

.ses-landmark-bar:hover {
  width: 36px;
}

.ses-landmark-track {
  position: absolute;
  right: 4px;
  top: 0;
  bottom: 0;
  width: 3px;
  background: var(--ses-border);
  border-radius: 2px;
}

.ses-landmark-thumb {
  position: absolute;
  right: 4px;
  width: 3px;
  min-height: 24px;
  background: var(--ses-text-dim);
  border-radius: 2px;
  cursor: grab;
  transition: background 80ms;
}

.ses-landmark-thumb:hover,
.ses-landmark-thumb.ses-dragging {
  background: var(--ses-accent);
  width: 5px;
  right: 3px;
}

.ses-landmark-bracket {
  position: absolute;
  right: 10px;
  width: 3px;
  border-radius: 2px;
  opacity: 0.6;
}

.ses-landmark-icon {
  position: absolute;
  right: 10px;
  width: 20px;
  height: 20px;
  border-radius: 4px;
  background: var(--ses-bg-raised);
  border: 1px solid var(--ses-border);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 10px;
  cursor: pointer;
  opacity: 0;
  transition: opacity 80ms;
  transform: translateY(-50%);
  white-space: nowrap;
  overflow: hidden;
}

.ses-landmark-bar:hover .ses-landmark-icon {
  opacity: 1;
}

.ses-landmark-icon:hover {
  border-color: var(--ses-accent);
  background: var(--ses-bg);
}
```

### 3.2 `assets/styles/page_top_bar.css`

```css
/* Page-level top bar */

.ses-page-area {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.ses-page-top-bar {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  border-bottom: 1px solid var(--ses-border);
  background: var(--ses-header);
  padding: 0 8px;
  gap: 4px;
  overflow: hidden;
}

.ses-topbar-zone {
  display: flex;
  align-items: center;
  gap: 4px;
}

.ses-topbar-zone-left  { flex: 1; justify-content: flex-start; }
.ses-topbar-zone-center { flex: 1; justify-content: center; }
.ses-topbar-zone-right  { flex: 1; justify-content: flex-end; }

.ses-topbar-btn {
  padding: 2px 10px;
  border-radius: 3px;
  font-size: 11px;
  background: var(--ses-bg-raised);
  border: 1px solid var(--ses-border);
  color: var(--ses-text);
}

.ses-topbar-btn:hover {
  border-color: var(--ses-accent);
  color: var(--ses-accent);
}

.ses-topbar-label {
  font-size: 11px;
  color: var(--ses-text-dim);
  white-space: nowrap;
}

.ses-topbar-flow {
  display: flex;
  align-items: center;
  gap: 4px;
  font-family: var(--ses-font-mono);
  font-size: 11px;
  background: var(--ses-bg);
  border: 1px solid var(--ses-border);
  border-radius: 3px;
  padding: 2px 6px;
  min-width: 60px;
  white-space: nowrap;
}

.ses-topbar-flow-channel {
  font-size: 9px;
  color: var(--ses-text-dim);
  display: block;
}

.ses-topbar-sep {
  width: 1px;
  height: 18px;
  background: var(--ses-border);
  margin: 0 4px;
}

.ses-topbar-overflow {
  margin-left: auto;
  padding: 2px 6px;
  color: var(--ses-text-dim);
  font-size: 12px;
}
```

---

## Phase 4 — Context Menu Integration (Landmark Creation UX)

The user needs a way to *create* a landmark from a pod header without writing data directly. The existing `PageLeafView` header already has a `select` and maximize/split controls. Add a context menu trigger.

**Changes to `crates/ses-ui/src/page/leaf.rs`:**

Add a `⚑` (or `⋯`) button to the leaf header. On click, open a small popover with:

- "Add landmark here" — opens a mini form: label input (max 3 chars), color picker (preset swatches), tooltip input, submit
- "Select pods to group..." — enters a multi-select mode where the user clicks other leaf headers to add them, then confirms with an icon/tooltip form
- If a landmark already exists on this leaf: "Edit landmark" and "Remove landmark"

On form submit, dispatch to `ShellState` via the `add_landmark` or `group_landmarks` ops.

This popover is a standard `use_signal(|| false)` toggle pattern, same as the existing join overlay in `split_handle.rs`.

---

## Phase 5 — Keyboard Shortcut Registration

In `PageArea`, after mounting, register document-level `keydown` listeners:

```
Alt+1 through Alt+9 → jump to landmark with shortcut_index 0–8
```

Use Dioxus's `use_effect` with a cleanup return to deregister on unmount. The listener reads the current `landmarks` vec from shell state reactively.

---

## Phase 6 — Defaults Update

In `crates/ses-shell/src/defaults.rs`, update `default_shell()` to demonstrate the new features:

- First default workspace: no top bar (clean default)
- Second default workspace (if one exists): add a `PageTopBar` with a `Label` slot showing the workspace name and a `FlowDisplay` slot on `"calc.result"` so the existing Calculation pod's output is immediately visible in the bar

---

## Implementation Order

1. `landmark.rs` — data types, no deps
2. `page.rs` additions — `PageTopBar` and slot types
3. `workspace.rs` — add fields to `WorkspaceDef`
4. `ops.rs` — landmark ops
5. `lib.rs` — re-exports
6. `scroll_bar.css` + `page_top_bar.css`
7. `page/page_area.rs` — scaffold with layout, no scroll logic yet
8. `page/top_bar.rs` — `PageTopBarView`, wire `FlowDisplay` to `use_flow`
9. `page/scroll_bar.rs` — track + thumb (no landmarks yet, just working custom scrollbar)
10. Add landmark icon rendering to scroll bar
11. Add bracket rendering to scroll bar
12. Leaf header context menu for landmark creation
13. Keyboard shortcut registration in `PageArea`
14. `defaults.rs` update
15. Tests: landmark ops (add, group, remove), `TopBarHeight::px()`, scroll fraction math

---

## What Is NOT Changed

- `PageNode`, `PageLeaf`, `IoLayout`, `IoPlacement` — untouched
- `PodKind`, `PodDescriptor` — untouched
- `FlowBus`, `FlowChannelId`, `FlowValue` — untouched (consumed read-only by `FlowDisplay`)
- `WorkspaceBar`, `TopBarPod` (global shell top bar), `StatusBarPod` — untouched
- All existing split/join/maximize ops — untouched
- All existing pod components (`ViewPod`, `OutlinerPod`, `CalculationPod`, etc.) — untouched
