use std::error::Error;

use aoc::gif::Gif;
use aoc::input::read_input_file;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = read_input_file(9)?;

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
        XDIM as u16,
        YDIM as u16,
        SCALE as u16,
        SCALE as u16,
    )?;

    let mut draw_frame = |alloc_in: &Vec<Block2>,
                          alloc_out: &Vec<(bool, Block2)>,
                          free: &Vec<Block2>,
                          freed: &Vec<Block2>|
     -> Result<(), Box<dyn Error>> {
        let mut frame = gif.empty_frame();

        for a in alloc_in.iter() {
            for i in a.pos..(a.pos + a.len as u32) {
                let y = i as usize / XDIM;
                let x = i as usize % XDIM;
                frame[y][x] = 1;
            }
        }

        for (moved, a) in alloc_out.iter() {
            for i in a.pos..(a.pos + a.len as u32) {
                let y = i as usize / XDIM;
                let x = i as usize % XDIM;
                frame[y][x] = if *moved { 2 } else { 1 };
            }
        }

        for f in free.iter() {
            for i in f.pos..(f.pos + f.len as u32) {
                let y = i as usize / XDIM;
                let x = i as usize % XDIM;
                frame[y][x] = 3;
            }
        }

        for f in freed.iter() {
            for i in f.pos..(f.pos + f.len as u32) {
                let y = i as usize / XDIM;
                let x = i as usize % XDIM;
                frame[y][x] = 4;
            }
        }

        gif.draw_frame_identical_check(frame, 2, aoc::gif::IdenticalAction::Ignore)?;

        Ok(())
    };

    draw_frame(&alloc_in, &alloc_out, &free, &freed)?;

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

        draw_frame(&alloc_in, &alloc_out, &free, &freed)?;
    }

    gif.delay(500)?;

    Ok(())
}
