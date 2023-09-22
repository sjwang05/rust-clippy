#![allow(clippy::collapsible_if)]
#![warn(clippy::ifs_in_if_conditions)]

fn main() {
    let cond = true;
    let something_else = true;

    if cond {
        println!();
    } else if something_else {
        println!("check FNs");
    } else {
        println!("check");
    }

    if cond {
        println!();
    }

    if cond {
        println!();
    }

    if cond {
        println!();
    }

    let a = 13;

    if a == 13 {
        println!("check FNs 1");
    } else if a == 14 {
        println!("check FNs 2");
    } else {
        println!("check FNs 3");
    }

    if if a == 13 { 10 } else { 0 } > 5 {
        println!("nested if");
    } else if if a == 12 {
        -10
    } else if if a == 13 { 10 } else { 0 } > 5 {
        0
    } else {
        1
    } < -5
    {
        println!("nested else if");
    }

    if if if if if a == 13 { 12 } else { 13 } == 12 { 11 } else { 13 } == 11 {
        10
    } else {
        13
    } == 10
    {
        9
    } else {
        13
    } == 9
    {
        println!("recursion");
    }
}
