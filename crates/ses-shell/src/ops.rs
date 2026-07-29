//! Pure layout operations on page trees and workspaces.

use crate::ids::LeafId;
use crate::landmark::{LandmarkDef, LandmarkIcon, LandmarkId};
use crate::page::{Axis, PageLeaf, PageNode};
use crate::pod::{PodDescriptor, PodKind};
use crate::workspace::WorkspaceDef;

/// Split a leaf into two along `axis`. The original leaf becomes `first`;
/// `second` gets a fresh leaf with the same pod kind (or `new_pod` if given).
pub fn split_leaf(
    root: &mut PageNode,
    leaf_id: LeafId,
    axis: Axis,
    ratio: f32,
    new_pod: Option<PodDescriptor>,
) -> bool {
    split_leaf_inner(root, leaf_id, axis, ratio, new_pod)
}

fn split_leaf_inner(
    node: &mut PageNode,
    leaf_id: LeafId,
    axis: Axis,
    ratio: f32,
    new_pod: Option<PodDescriptor>,
) -> bool {
    match node {
        PageNode::Leaf(leaf) if leaf.id == leaf_id => {
            let original = leaf.clone();
            let second_pod = new_pod.unwrap_or_else(|| original.pod.clone());
            let second = PageLeaf::new(second_pod);
            *node = PageNode::split(
                axis,
                ratio,
                PageNode::Leaf(original),
                PageNode::Leaf(second),
            );
            true
        }
        PageNode::Leaf(_) => false,
        PageNode::Split { first, second, .. } => {
            split_leaf_inner(first, leaf_id, axis, ratio, new_pod.clone())
                || split_leaf_inner(second, leaf_id, axis, ratio, new_pod)
        }
    }
}

/// Update split ratio for the split node that directly contains `leaf_id`
/// as one of its immediate children (or any ancestor split matching path).
/// Simpler API: set ratio on the split whose first or second subtree contains the leaf
/// at the shallowest split — actually we walk and set on the *parent* split of the leaf.
pub fn set_split_ratio(root: &mut PageNode, leaf_id: LeafId, ratio: f32) -> bool {
    set_parent_split_ratio(root, leaf_id, ratio.clamp(0.05, 0.95))
}

fn set_parent_split_ratio(node: &mut PageNode, leaf_id: LeafId, ratio: f32) -> bool {
    match node {
        PageNode::Leaf(_) => false,
        PageNode::Split {
            ratio: r,
            first,
            second,
            ..
        } => {
            let in_first = matches!(first.as_ref(), PageNode::Leaf(l) if l.id == leaf_id)
                || first.find_leaf(leaf_id).is_some();
            let in_second = matches!(second.as_ref(), PageNode::Leaf(l) if l.id == leaf_id)
                || second.find_leaf(leaf_id).is_some();

            // If leaf is an immediate child, set this split's ratio.
            let first_immediate = matches!(first.as_ref(), PageNode::Leaf(l) if l.id == leaf_id);
            let second_immediate = matches!(second.as_ref(), PageNode::Leaf(l) if l.id == leaf_id);
            if first_immediate || second_immediate {
                *r = ratio;
                return true;
            }
            if in_first {
                set_parent_split_ratio(first, leaf_id, ratio)
            } else if in_second {
                set_parent_split_ratio(second, leaf_id, ratio)
            } else {
                false
            }
        }
    }
}

/// Set ratio on a specific split by walking with a path index, or by matching
/// any split — used by the UI drag handle which knows the split path.
pub fn set_split_ratio_at(root: &mut PageNode, path: &[usize], ratio: f32) -> bool {
    let ratio = ratio.clamp(0.05, 0.95);
    let mut node = root;
    for &idx in path {
        match node {
            PageNode::Split { first, second, .. } => {
                node = if idx == 0 { first } else { second };
            }
            PageNode::Leaf(_) => return false,
        }
    }
    match node {
        PageNode::Split { ratio: r, .. } => {
            *r = ratio;
            true
        }
        PageNode::Leaf(_) => false,
    }
}

/// Join `leaf_id` into its sibling: replace the parent split with the other child.
/// Returns false if the leaf is missing or is the sole root leaf.
pub fn join_leaf(root: &mut PageNode, leaf_id: LeafId) -> bool {
    join_leaf_inner(root, leaf_id)
}

fn join_leaf_inner(node: &mut PageNode, leaf_id: LeafId) -> bool {
    match node {
        PageNode::Leaf(_) => false,
        PageNode::Split { first, second, .. } => {
            let first_is = matches!(first.as_ref(), PageNode::Leaf(l) if l.id == leaf_id);
            let second_is = matches!(second.as_ref(), PageNode::Leaf(l) if l.id == leaf_id);
            if first_is {
                let kept = second.as_ref().clone();
                *node = kept;
                return true;
            }
            if second_is {
                let kept = first.as_ref().clone();
                *node = kept;
                return true;
            }
            join_leaf_inner(first, leaf_id) || join_leaf_inner(second, leaf_id)
        }
    }
}

/// Join at a split by path: discard `first` or `second` and promote the other.
pub fn join_split_at(root: &mut PageNode, path: &[usize], discard_first: bool) -> bool {
    let mut node = root;
    for &idx in path {
        match node {
            PageNode::Split { first, second, .. } => {
                node = if idx == 0 { first } else { second };
            }
            PageNode::Leaf(_) => return false,
        }
    }
    match node {
        PageNode::Split { first, second, .. } => {
            let kept = if discard_first {
                second.as_ref().clone()
            } else {
                first.as_ref().clone()
            };
            *node = kept;
            true
        }
        PageNode::Leaf(_) => false,
    }
}

pub fn set_leaf_pod(root: &mut PageNode, leaf_id: LeafId, kind: PodKind, module_id: &str) -> bool {
    if let Some(leaf) = root.find_leaf_mut(leaf_id) {
        leaf.pod = PodDescriptor::new(kind, module_id);
        true
    } else {
        false
    }
}

/// Maximize a leaf within a workspace (fills workspace area).
pub fn maximize_leaf(ws: &mut WorkspaceDef, leaf_id: LeafId) -> bool {
    if ws.layout.find_leaf(leaf_id).is_none() {
        return false;
    }
    if ws.maximized.is_none() {
        ws.layout_before_maximize = Some(ws.layout.clone());
    }
    ws.maximized = Some(leaf_id);
    true
}

/// Restore layout after maximize.
pub fn restore_layout(ws: &mut WorkspaceDef) {
    ws.maximized = None;
    // layout itself was never replaced — maximized is a view flag.
    ws.layout_before_maximize = None;
}

/// Effective node to render for a workspace (single leaf when maximized).
pub fn effective_layout(ws: &WorkspaceDef) -> PageNode {
    if let Some(id) = ws.maximized {
        if let Some(leaf) = ws.layout.find_leaf(id) {
            return PageNode::Leaf(leaf.clone());
        }
    }
    ws.layout.clone()
}

/// Add a single-leaf landmark to the workspace.
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

/// Scroll fraction from scroll metrics: `scroll_top / (scroll_height - client_height)`.
/// Returns 0.0 when content does not overflow.
pub fn scroll_fraction(scroll_top: f64, scroll_height: f64, client_height: f64) -> f32 {
    let range = scroll_height - client_height;
    if range <= 0.0 {
        0.0
    } else {
        (scroll_top / range).clamp(0.0, 1.0) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::ModuleId;
    use crate::pod::PodKind;

    fn sample_leaf(kind: PodKind) -> PageNode {
        PageNode::leaf(PodDescriptor::new(kind, ModuleId::new("core-ui")))
    }

    #[test]
    fn split_creates_two_leaves() {
        let mut root = sample_leaf(PodKind::View);
        let id = match &root {
            PageNode::Leaf(l) => l.id,
            _ => panic!(),
        };
        assert!(split_leaf(
            &mut root,
            id,
            Axis::Horizontal,
            0.5,
            Some(PodDescriptor::new(PodKind::Outliner, "core-ui"))
        ));
        assert_eq!(root.leaf_ids().len(), 2);
    }

    #[test]
    fn join_left_child_keeps_right() {
        let left = sample_leaf(PodKind::View);
        let left_id = match &left {
            PageNode::Leaf(l) => l.id,
            _ => panic!(),
        };
        let right = sample_leaf(PodKind::Outliner);
        let right_id = match &right {
            PageNode::Leaf(l) => l.id,
            _ => panic!(),
        };
        let mut root = PageNode::split(Axis::Horizontal, 0.5, left, right);
        assert!(join_leaf(&mut root, left_id));
        assert_eq!(root.leaf_ids(), vec![right_id]);
    }

    #[test]
    fn join_right_child_keeps_left() {
        let left = sample_leaf(PodKind::View);
        let left_id = match &left {
            PageNode::Leaf(l) => l.id,
            _ => panic!(),
        };
        let right = sample_leaf(PodKind::Outliner);
        let right_id = match &right {
            PageNode::Leaf(l) => l.id,
            _ => panic!(),
        };
        let mut root = PageNode::split(Axis::Horizontal, 0.5, left, right);
        assert!(join_leaf(&mut root, right_id));
        assert_eq!(root.leaf_ids(), vec![left_id]);
    }

    #[test]
    fn join_nested_leaf() {
        let a = sample_leaf(PodKind::View);
        let a_id = match &a {
            PageNode::Leaf(l) => l.id,
            _ => panic!(),
        };
        let b = sample_leaf(PodKind::Outliner);
        let c = sample_leaf(PodKind::Properties);
        let c_id = match &c {
            PageNode::Leaf(l) => l.id,
            _ => panic!(),
        };
        let inner = PageNode::split(Axis::Vertical, 0.5, b, c);
        let mut root = PageNode::split(Axis::Horizontal, 0.4, a, inner);
        assert!(join_leaf(&mut root, c_id));
        assert_eq!(root.leaf_ids().len(), 2);
        assert!(root.leaf_ids().contains(&a_id));
        assert!(!root.leaf_ids().contains(&c_id));
    }

    #[test]
    fn join_sole_leaf_fails() {
        let mut root = sample_leaf(PodKind::View);
        let id = match &root {
            PageNode::Leaf(l) => l.id,
            _ => panic!(),
        };
        assert!(!join_leaf(&mut root, id));
        assert_eq!(root.leaf_ids().len(), 1);
    }

    #[test]
    fn join_split_at_discards_first_horizontal() {
        let left = sample_leaf(PodKind::View);
        let right = sample_leaf(PodKind::Outliner);
        let right_id = match &right {
            PageNode::Leaf(l) => l.id,
            _ => panic!(),
        };
        let mut root = PageNode::split(Axis::Horizontal, 0.5, left, right);
        assert!(join_split_at(&mut root, &[], true));
        assert_eq!(root.leaf_ids(), vec![right_id]);
    }

    #[test]
    fn join_split_at_discards_second_vertical() {
        let top = sample_leaf(PodKind::View);
        let top_id = match &top {
            PageNode::Leaf(l) => l.id,
            _ => panic!(),
        };
        let bottom = sample_leaf(PodKind::Properties);
        let mut root = PageNode::split(Axis::Vertical, 0.5, top, bottom);
        assert!(join_split_at(&mut root, &[], false));
        assert_eq!(root.leaf_ids(), vec![top_id]);
    }

    #[test]
    fn join_split_at_nested_path() {
        let a = sample_leaf(PodKind::View);
        let a_id = match &a {
            PageNode::Leaf(l) => l.id,
            _ => panic!(),
        };
        let b = sample_leaf(PodKind::Outliner);
        let c = sample_leaf(PodKind::Properties);
        let c_id = match &c {
            PageNode::Leaf(l) => l.id,
            _ => panic!(),
        };
        let inner = PageNode::split(Axis::Vertical, 0.5, b, c);
        let mut root = PageNode::split(Axis::Horizontal, 0.4, a, inner);
        // Discard first of nested split (path [1]) → keep c
        assert!(join_split_at(&mut root, &[1], true));
        assert_eq!(root.leaf_ids(), vec![a_id, c_id]);
    }

    #[test]
    fn join_split_at_bad_path_fails() {
        let mut root = sample_leaf(PodKind::View);
        assert!(!join_split_at(&mut root, &[], true));
        assert!(!join_split_at(&mut root, &[0], true));
    }

    #[test]
    fn landmark_add_group_remove() {
        let leaf_a = LeafId::new();
        let leaf_b = LeafId::new();
        let mut ws = WorkspaceDef::new("T", sample_leaf(PodKind::View));
        let id_a = add_landmark(&mut ws, leaf_a, LandmarkIcon::new("A"));
        let id_b = add_landmark(&mut ws, leaf_b, LandmarkIcon::new("B"));
        assert_eq!(ws.landmarks.len(), 2);

        let group_id =
            group_landmarks(&mut ws, &[id_a, id_b], LandmarkIcon::new("G")).expect("group");
        assert_eq!(ws.landmarks.len(), 1);
        assert_eq!(ws.landmarks[0].id, group_id);
        assert_eq!(ws.landmarks[0].leaf_ids, vec![leaf_a, leaf_b]);

        remove_landmark(&mut ws, group_id);
        assert!(ws.landmarks.is_empty());
    }

    #[test]
    fn group_landmarks_empty_returns_none() {
        let mut ws = WorkspaceDef::new("T", sample_leaf(PodKind::View));
        assert!(group_landmarks(&mut ws, &[], LandmarkIcon::new("G")).is_none());
    }

    #[test]
    fn scroll_fraction_math() {
        assert_eq!(scroll_fraction(0.0, 100.0, 100.0), 0.0);
        assert_eq!(scroll_fraction(50.0, 200.0, 100.0), 0.5);
        assert_eq!(scroll_fraction(100.0, 200.0, 100.0), 1.0);
        assert_eq!(scroll_fraction(-10.0, 200.0, 100.0), 0.0);
    }
}
