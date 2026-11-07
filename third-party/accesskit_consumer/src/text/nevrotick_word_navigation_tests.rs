// SPDX-License-Identifier: GPL-3.0-only
// Copyright 2026 David Osipov.

//! Exercise the actual consumer API with valid inclusive text-run endpoints.

use accesskit::{Action, Node, NodeId, Role, Tree as TreeData, TreeId, TreeUpdate};
use alloc::{string::String, vec, vec::Vec};

use crate::Tree;

/// Build one text input from consistent text, character lengths and word starts.
fn input_tree(runs: &[(String, Vec<u8>, Vec<u8>)]) -> Tree {
    let root_id = NodeId(0);
    let mut root = Node::new(Role::TextInput);
    root.add_action(Action::Focus);
    root.set_children(
        (1..=runs.len())
            .map(|index| NodeId(index as u64))
            .collect::<Vec<_>>(),
    );
    let mut nodes = vec![(root_id, root)];
    for (index, (text, lengths, starts)) in runs.iter().enumerate() {
        let mut run = Node::new(Role::TextRun);
        run.set_value(text.as_str());
        run.set_character_lengths(lengths.clone());
        run.set_word_starts(starts.clone());
        if index > 0 {
            run.set_previous_on_line(NodeId(index as u64));
        }
        if index + 1 < runs.len() {
            run.set_next_on_line(NodeId((index + 2) as u64));
        }
        nodes.push((NodeId((index + 1) as u64), run));
    }
    Tree::new(
        TreeUpdate {
            nodes,
            tree: Some(TreeData::new(root_id)),
            tree_id: TreeId::ROOT,
            focus: root_id,
        },
        true,
    )
}

fn multiword_run() -> (String, Vec<u8>, Vec<u8>) {
    (
        "x ".repeat(128),
        vec![1; 256],
        (0..=254).step_by(2).collect(),
    )
}

fn assert_backward_word(tree: &Tree, position: usize, expected: usize) {
    let input = tree.state().root();
    assert!(input.supports_text_ranges());
    let current = input.text_position_from_global_usv_index(position).unwrap();
    assert_eq!(current.to_global_usv_index(), position);
    assert_eq!(
        current.backward_to_word_start().to_global_usv_index(),
        expected
    );
}

#[test]
fn backward_word_at_inclusive_256_endpoint_keeps_last_word() {
    let tree = input_tree(&[multiword_run()]);
    let input = tree.state().root();
    let end = input.document_range().end();
    assert!(end.is_document_end());
    assert_eq!(end.to_global_usv_index(), 256);
    assert_backward_word(&tree, 256, 254);
}

#[test]
fn backward_word_at_neighbor_255_keeps_last_word() {
    assert_backward_word(&input_tree(&[multiword_run()]), 255, 254);
}

#[test]
fn backward_word_at_exact_word_start_selects_previous_word() {
    assert_backward_word(&input_tree(&[multiword_run()]), 254, 252);
}

#[test]
fn backward_word_at_document_start_stays_at_start() {
    assert_backward_word(&input_tree(&[multiword_run()]), 0, 0);
}

#[test]
fn backward_word_at_single_word_256_endpoint_reaches_start() {
    let tree = input_tree(&[("x".repeat(256), vec![1; 256], vec![0])]);
    assert_backward_word(&tree, 256, 0);
}

#[test]
fn backward_word_retains_u8_maximum_word_start() {
    let tree = input_tree(&[("x".repeat(254) + " y", vec![1; 256], vec![0, 255])]);
    assert_backward_word(&tree, 256, 255);
    assert_backward_word(&tree, 255, 0);
}

#[test]
fn backward_word_uses_character_position_with_surrogate_pairs() {
    let tree = input_tree(&[(
        "💡 ".repeat(128),
        [4, 1].repeat(128),
        (0..=254).step_by(2).collect(),
    )]);
    let input = tree.state().root();
    let end = input.text_position_from_global_utf16_index(384).unwrap();
    assert!(end.is_document_end());
    assert_eq!(end.to_global_usv_index(), 256);
    assert_eq!(end.backward_to_word_start().to_global_utf16_index(), 381);
}

#[test]
fn backward_word_at_second_run_endpoint_preserves_run_origin() {
    let tree = input_tree(&[(String::from("hi "), vec![1; 3], vec![0]), multiword_run()]);
    assert_backward_word(&tree, 259, 257);
    assert_backward_word(&tree, 3, 0);
}
