use std::{fs::File, io::Write};

use taffy::prelude::*;

fn main() -> Result<(), taffy::TaffyError> {
    let mut taffy = Taffy::new();

    let child = taffy.new_leaf(Style {
        size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto },
        ..Default::default()
    })?;

    let mut s = Style {
        size: Size { width: Dimension::Length(0.5), height: Dimension::Auto },
        position: Position::Relative,
        min_size: Size { width: Dimension::Length(50000.0), height: Dimension::Length(5000.0) },
        ..Default::default()
    };
    s.padding.left = LengthPercentage::Percent(5.0);

    let child2 = taffy.new_leaf(s)?;

    let node = taffy.new_with_children(
        Style {
            size: Size { width: Dimension::Length(100.0), height: Dimension::Length(100.0) },
            justify_content: Some(JustifyContent::Center),
            position: Position::Relative,
            ..Default::default()
        },
        &[child],
    )?;

    taffy.set_children(node, &[child, child2])?;

    taffy.compute_layout(
        node,
        Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
    )?;

    // or just use undefined for 100 x 100
    // taffy.compute_layout(node, Size::NONE)?;

    println!("node: {:#?}", taffy.layout(node)?);
    println!("child: {:#?}", taffy.layout(child)?);
    println!("child: {:#?}", taffy.layout(child2)?);

    let mut file = File::create("test.txt").unwrap();

    let content = vec![
        taffy.layout(node).unwrap(),
        taffy.layout(child).unwrap(),
        taffy.layout(child2).unwrap(),
    ];
    let content = format!("{:#?}", content);
    println!("{}", content);

    file.write_all(content.as_bytes()).unwrap();

    Ok(())
}