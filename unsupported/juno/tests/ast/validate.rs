/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use juno::ast::{validate_tree, Context, Node, NodeKind, NodePtr};

#[test]
fn test_valid() {
    use NodeKind::*;
    let mut ctx = Context::new();
    let return_stmt = make_node!(ctx, ReturnStatement { argument: None });
    assert!(validate_tree(&ctx, return_stmt).is_ok());

    let return_stmt = make_node!(
        ctx,
        ReturnStatement {
            argument: Some(make_node!(ctx, NumericLiteral { value: 1.0 }))
        }
    );
    assert!(validate_tree(&ctx, return_stmt).is_ok());

    let return_stmt = make_node!(
        ctx,
        ReturnStatement {
            argument: Some(make_node!(ctx, ReturnStatement { argument: None })),
        }
    );
    assert!(validate_tree(&ctx, return_stmt).is_err());
}

#[test]
fn test_error() {
    use NodeKind::*;
    let mut ctx = Context::new();
    let ast = make_node!(
        ctx,
        BlockStatement {
            body: vec![make_node!(
                ctx,
                ReturnStatement {
                    argument: Some(make_node!(ctx, ReturnStatement { argument: None })),
                }
            )],
        }
    );
    let bad_ret: NodePtr = match &ast.get(&ctx).kind {
        BlockStatement { body } => body[0],
        _ => {
            unreachable!("bad match");
        }
    };
    match validate_tree(&ctx, ast) {
        Ok(()) => {
            panic!("Must be error");
        }
        Err(e) => {
            assert_eq!(e.len(), 1);
            assert_eq!(
                e[0].node.get(&ctx) as *const Node,
                bad_ret.get(&ctx) as *const Node
            );
        }
    }
}
