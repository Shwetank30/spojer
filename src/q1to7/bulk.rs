use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

type Face = Vec<Point>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct FaceEdge {
    f: usize,
    v: usize,
    d: i32, // +1 or -1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Ray {
    pt: Point,
    u: i32, // unit vector code
}

fn get_uvec(p1: Point, p2: Point) -> i32 {
    if p1.x < p2.x {
        1
    } else if p1.x > p2.x {
        -1
    } else if p1.y < p2.y {
        2
    } else if p1.y > p2.y {
        -2
    } else if p1.z < p2.z {
        3
    } else {
        -3
    }
}

fn volume(p1: Point, p2: Point, p3: Point) -> i64 {
    let r1 = p1.x as i64 * (p2.y as i64 * p3.z as i64 - p2.z as i64 * p3.y as i64);
    let r2 = p2.x as i64 * (p3.y as i64 * p1.z as i64 - p3.z as i64 * p1.y as i64);
    let r3 = p3.x as i64 * (p1.y as i64 * p2.z as i64 - p1.z as i64 * p2.y as i64);
    r1 + r2 + r3
}

fn dfs(
    faces: &Vec<Face>,
    orientation: &mut Vec<i32>,
    fe: FaceEdge,
    m: &HashMap<Ray, Vec<FaceEdge>>,
    unvisited: &mut usize,
) -> i64 {
    if orientation[fe.f] != 0 {
        if orientation[fe.f] != fe.d {
            panic!("inconsistent orientations for face {}", fe.f);
        }
        return 0;
    }
    orientation[fe.f] = fe.d;
    while *unvisited < faces.len() && orientation[*unvisited] != 0 {
        *unvisited += 1;
    }
    let face = &faces[fe.f];
    let mut result = 0i64;
    for i in 0..face.len() {
        for &d in &[-1, 1] {
            let j = (i as i32 + d + face.len() as i32) % face.len() as i32;
            let j = j as usize;
            let u = get_uvec(face[i], face[j]);
            let ray = Ray { pt: face[i], u };
            if let Some(edges) = m.get(&ray) {
                for &new_fe in edges {
                    if new_fe.f == fe.f {
                        continue;
                    }
                    result += dfs(
                        faces,
                        orientation,
                        FaceEdge {
                            f: new_fe.f,
                            v: new_fe.v,
                            d: -new_fe.d * fe.d * d,
                        },
                        m,
                        unvisited,
                    );
                }
            } else {
                panic!("current edge not found in lookup table");
            }
        }
        if i > 0 && i + 1 < face.len() {
            result += fe.d as i64 * volume(face[0], face[i], face[i + 1]);
        }
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let f: usize = lines.next().unwrap().trim().parse().unwrap();
        let mut faces = Vec::with_capacity(f);
        let mut orientation = vec![0; f];
        let mut initial_fe = FaceEdge {
            f: usize::MAX,
            v: usize::MAX,
            d: 1,
        };
        let mut least_z = 2000;
        for i in 0..f {
            let line = lines.next().unwrap();
            let tokens: Vec<_> = line.split_whitespace().collect();
            let p: usize = tokens[0].parse().unwrap();
            let mut face = Vec::with_capacity(p);
            for j in 0..p {
                let x = tokens[1 + 3 * j].parse::<i32>().unwrap();
                let y = tokens[2 + 3 * j].parse::<i32>().unwrap();
                let z = tokens[3 + 3 * j].parse::<i32>().unwrap();
                face.push(Point { x, y, z });
                if z < least_z {
                    least_z = z;
                    initial_fe = FaceEdge {
                        f: i,
                        v: face.len() - 1,
                        d: 1,
                    };
                }
            }
            faces.push(face);
        }
        // build lookup table
        let mut m: HashMap<Ray, Vec<FaceEdge>> = HashMap::new();
        for (i, face) in faces.iter().enumerate() {
            for j in 0..face.len() {
                let k = (j + 1) % face.len();
                let u = get_uvec(face[j], face[k]);
                m.entry(Ray { pt: face[j], u })
                    .or_default()
                    .push(FaceEdge { f: i, v: j, d: 1 });
                m.entry(Ray { pt: face[k], u: -u })
                    .or_default()
                    .push(FaceEdge { f: i, v: k, d: -1 });
            }
        }
        let mut unvisited = 0;
        let mut vol = dfs(&faces, &mut orientation, initial_fe, &m, &mut unvisited).abs();
        while unvisited < faces.len() {
            let fe = FaceEdge {
                f: unvisited,
                v: 0,
                d: 1,
            };
            vol -= dfs(&faces, &mut orientation, fe, &m, &mut unvisited).abs();
        }
        println!("The bulk is composed of {} units.", vol / 6);
    }
}
