use std::error::Error;

use aoc::gif::Gif;
use aoc::input::parse_input;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input(9, |s| s.to_string())?;

    part2(&input)?;

    Ok(())
}

const XDIM: usize = 380;
const YDIM: usize = 260;
const SCALE: usize = 3;

struct Block2 {
    pos: u32,
    len: u8,
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut alloc_in = Vec::new();
    let mut free = Vec::new();
    let mut freed = Vec::new();

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

    let palette = vec![[0, 0, 0], [64, 255, 64], [255, 255, 64], [64, 0, 0], [
        128, 0, 0,
    ]];

    let mut gif = Gif::new(
        "vis/day09-2.gif",
        &palette,
        (XDIM * SCALE) as u16 + 1,
        (YDIM * SCALE) as u16 + 1,
        1,
        1,
    )?;

    draw_frame(&mut gif, &alloc_in, &alloc_out, &free, &freed)?;

    while let Some(a) = alloc_in.pop() {
        // Find first free
        if free[0].pos > a.pos {
            alloc_in.push(a);
            break;
        }

        if let Some(f) = free.iter().position(|f| f.len >= a.len && f.pos < a.pos) {
            // Move to free block
            alloc_out.push((true, Block2 {
                pos: free[f].pos,
                len: a.len,
            }));

            // Adjust / remove free block
            if free[f].len > a.len {
                free[f].pos += a.len as u32;
                free[f].len -= a.len;
            } else {
                free.remove(f);
            }

            freed.push(Block2 {
                pos: a.pos,
                len: a.len,
            });
        } else {
            // Don't move
            alloc_out.push((false, a))
        }

        draw_frame(&mut gif, &alloc_in, &alloc_out, &free, &freed)?;
    }

    gif.delay(500)?;

    Ok(())
}

fn draw_frame(
    gif: &mut Gif,
    alloc_in: &[Block2],
    alloc_out: &[(bool, Block2)],
    free: &[Block2],
    freed: &[Block2],
) -> Result<(), Box<dyn Error>> {
    let mut frame = gif.empty_frame();

    let mut draw_block = |block: &Block2, colour| {
        for i in 0..block.len {
            let pos = block.pos as usize + i as usize;
            let y = (pos / XDIM) * SCALE;
            let x = (pos % XDIM) * SCALE;

            for dy in 1..SCALE {
                let sx = if i == 0 { 1 } else { 0 };

                for dx in sx..SCALE {
                    frame[y + dy][x + dx] = colour;
                }
            }
        }
    };

    for a in alloc_in.iter() {
        draw_block(a, 1);
    }

    for (moved, a) in alloc_out.iter() {
        draw_block(a, if *moved { 2 } else { 1 });
    }

    for f in free.iter() {
        draw_block(f, 3);
    }

    for f in freed.iter() {
        draw_block(f, 4);
    }

    gif.draw_frame_identical_check(frame, 2, aoc::gif::IdenticalAction::Ignore)?;

    Ok(())
}
