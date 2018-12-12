#![allow(dead_code)]
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fmt;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32
}


fn top_left(a: &Point, b: &Point) -> Point {
    Point {
        x: cmp::min(a.x, b.x),
        y: cmp::min(a.y, b.y)
    }
}

fn bottom_right(a: &Point, b: &Point) -> Point {
    Point {
        x: cmp::max(a.x, b.x),
        y: cmp::max(a.y, b.y)
    }
}

struct Rect {
    tl: Point,
    br: Point
}

struct Claim {
    id: u32,
    rect: Rect
}

impl Rect {

    fn w(&self) -> u32 {
        self.br.x - self.tl.x
    }

    fn h(&self) -> u32 {
        self.br.y - self.tl.y
    }

    fn intersect(&self, other: Rect) -> Option<Rect> {
        let ap1 = &self.tl;
        let ap2 = &self.br;
        let bp1 = &other.tl;
        let bp2 = &other.br;

        if ap1.x > bp2.x
            || ap2.x < bp1.x
            || ap1.y > bp2.y
            || ap2.y < bp1.y{
            return None;
        }

        Some(
            Rect{
                tl: bottom_right(ap1, bp1),
                br: top_left(ap2, bp2)
            })
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}: {}x{}", self.tl.x, self.tl.y, self.w(), self.h())
    }
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} @ {}", self.id, self.rect)
    }
}

fn rect_from_xywh(x: u32, y: u32, w: u32, h: u32) -> Rect {
    Rect {
        tl: Point {
            x,
            y
        },
        br: Point {
            x: x + w,
            y: y + h
        }
    }
}

fn rect_from_points(p1: &Point, p2: &Point) -> Rect {
    Rect {
        tl: Point {
            x: cmp::min(p1.x, p2.x),
            y: cmp::min(p1.y, p2.y),
        },
        br: Point {
            x: cmp::max(p1.x, p2.x),
            y: cmp::max(p1.y, p2.y)
        }
    }
}

fn bounding_rect(r1: &Rect, r2: &Rect) -> Rect {
    Rect {
        tl: top_left(&r1.tl, &r2.tl),
        br: bottom_right(&r1.br, &r2.br)
    }
}

//#1 @ 1,3: 4x4
fn claim_from_str(str: &str) -> Claim {
    let hash = str.find("#").unwrap();
    let at = str.find("@").unwrap();
    let comma = str.find(",").unwrap();
    let colon = str.find(":").unwrap();
    let mul = str.find("x").unwrap();

    let id = str[hash+1..at-1].parse::<u32>().unwrap();
    let x = str[at+2..comma].parse::<u32>().unwrap();
    let y = str[comma+1..colon].parse::<u32>().unwrap();
    let w = str[colon+2..mul].parse::<u32>().unwrap();
    let h = str[mul+1..].parse::<u32>().unwrap();

    Claim {
        id,
        rect: rect_from_xywh(x, y, w, h)
    }
}

struct Land<'a> {
    claimed: HashMap<Point, Vec<&'a Claim>>
}

impl <'a> Land<'a> {
    fn add(&mut self, claim: &'a Claim) {
        for x in claim.rect.tl.x..claim.rect.br.x {
            for y in claim.rect.tl.y..claim.rect.br.y {
                let point = Point{x, y};
                self.claimed
                    .entry(point)
                    .or_insert_with(|| Vec::new())
                    .push(claim);
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("D:\\dev\\advent_of_code_2018\\rust-03\\input.txt")
        .expect("peut");

//    let mut claims = Vec::new();
//    claims.push(claim_from_str("#1 @ 1,3: 4x4"));
//    claims.push(claim_from_str("#2 @ 3,1: 4x4"));
//    claims.push(claim_from_str("#3 @ 5,5: 2x2"));

    let claims: Vec<Claim> = contents.lines()
        .map(|line| claim_from_str(line))
        .collect();

    let zero = rect_from_xywh(0, 0, 0, 0);
    let bb = claims.iter()
        .fold(zero, |a, b| bounding_rect(&a, &b.rect));

    let mut land = Land{
        claimed: HashMap::new()
    };

    claims.iter().for_each(|c| land.add(c));
    let dupes: Vec<&Vec<&Claim>> = land.claimed.values()
        .filter(|claims| claims.len() > 1)
        .collect();

    println!("bounding box: {}", bb);
    println!("part one: claims {}, dupes {}", claims.len(), dupes.len());

    let ids: HashSet<u32> = dupes
        .iter()
        .flat_map(
            |claims| claims.iter().map(|claim| claim.id))
        .collect();

    let option = claims.iter()
        .find(|claim| !ids.contains(&claim.id));
    print!("part two: ");
    match option {
        Some(claim) => println!("{}", claim),
        None => println!("none")
    }
}
