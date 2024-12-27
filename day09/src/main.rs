use std::error::Error;

use aoc::input::parse_input;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input(9, |s| s.to_string())?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[derive(Clone, Copy)]
enum Block1 {
    Free,
    Used(u16),
}

fn part1(input: &str) -> u64 {
    let mut layout = Vec::new();

    for (id, grp) in input.trim_ascii_end().as_bytes().chunks(2).enumerate() {
        layout.extend(vec![Block1::Used(id as u16); (grp[0] - b'0') as usize]);

        if grp.len() > 1 {
            layout.extend(vec![Block1::Free; (grp[1] - b'0') as usize]);
        }
    }

    let mut free_ptr = 0;
    let mut occ_ptr = layout.len() - 1;

    loop {
        // Move free ptr
        while matches!(layout[free_ptr], Block1::Used(_)) {
            free_ptr += 1;
        }

        // Move occupied ptr
        while matches!(layout[occ_ptr], Block1::Free) {
            occ_ptr -= 1;
        }

        // Check
        if occ_ptr < free_ptr {
            break;
        }

        // Update
        layout[free_ptr] = layout[occ_ptr];
        layout[occ_ptr] = Block1::Free;
    }

    layout[..free_ptr]
        .iter()
        .enumerate()
        .map(|(i, b)| match b {
            Block1::Used(bn) => i as u64 * *bn as u64,
            _ => 0,
        })
        .sum()
}

struct Block2 {
    pos: u32,
    len: u8,
}

fn part2(input: &str) -> u64 {
    let mut alloc_in = Vec::new();
    let mut free = Vec::new();

    let mut pos: u32 = 0;

    for grp in input.trim_ascii_end().as_bytes().chunks(2) {
        let len = grp[0] - b'0';
        alloc_in.push(Block2 { pos, len });
        pos += len as u32;

        if grp.len() > 1 {
            let len = grp[1] - b'0';
            if len > 0 {
                free.push(Block2 { pos, len });
                pos += len as u32;
            }
        }
    }

    let mut alloc_out = Vec::new();

    while let Some(a) = alloc_in.pop() {
        // Find first free
        if free[0].pos > a.pos {
            alloc_in.push(a);
            break;
        }

        if let Some(f) = free.iter().position(|f| f.len >= a.len && f.pos < a.pos) {
            // Move to free block
            alloc_out.push(Block2 {
                pos: free[f].pos,
                len: a.len,
            });

            // Adjust / remove free block
            if free[f].len > a.len {
                free[f].pos += a.len as u32;
                free[f].len -= a.len;
            } else {
                free.remove(f);
            }
        } else {
            // Don't move
            alloc_out.push(a)
        }
    }

    alloc_in
        .iter()
        .chain(alloc_out.iter().rev())
        .enumerate()
        .map(|(id, a)| {
            (0..a.len)
                .map(|i| (a.pos + i as u32) as u64 * id as u64)
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests;
