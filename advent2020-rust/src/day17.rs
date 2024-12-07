use std::collections::HashMap;

#[derive(Clone)]
struct Cubes {
    cube_map: HashMap<(i32, i32, i32), bool>,
    xmin: i32,
    ymin: i32,
    zmin: i32,
    xmax: i32,
    ymax: i32,
    zmax: i32,
}

impl Cubes {
    fn from_lines(lines: &Vec<String>) -> Cubes {
        let mut cube_map: HashMap<(i32, i32, i32), bool> = HashMap::new();

        let mut x = 0;
        let mut y = 0;
        let z = 0;

        let mut xmin: i32 = 0;
        let mut ymin: i32 = 0;
        let mut zmin: i32 = 0;
        let mut xmax: i32 = 0;
        let mut ymax: i32 = 0;
        let mut zmax: i32 = 0;

        for line in lines {
            for c in line.chars() {
                if c == '#' {
                    cube_map.insert((x, y, z), true);
                    if x < xmin {
                        xmin = x;
                    }
                    if y < ymin {
                        ymin = y;
                    }
                    if z < zmin {
                        zmin = z;
                    }
                    if x > xmax {
                        xmax = x;
                    }
                    if y > ymax {
                        ymax = y;
                    }
                    if z > zmax {
                        zmax = z;
                    }
                }
                x += 1;
            }
            y -= 1;
            x = 0;
        }

        Cubes {
            cube_map,
            xmin,
            ymin,
            zmin,
            xmax,
            ymax,
            zmax,
        }
    }

    fn tick(&self) -> Cubes {
        let mut cube_map: HashMap<(i32, i32, i32), bool> = HashMap::new();

        let mut xmin: i32 = self.xmin;
        let mut ymin: i32 = self.ymin;
        let mut zmin: i32 = self.zmin;
        let mut xmax: i32 = self.xmax;
        let mut ymax: i32 = self.ymax;
        let mut zmax: i32 = self.zmax;

        for x in (xmin - 1)..=(xmax + 1) {
            for y in (ymin - 1)..=(ymax + 1) {
                for z in (zmin - 1)..=(zmax + 1) {
                    let mut neighbors: usize = 0;

                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            for dz in -1..=1 {
                                if dx == 0 && dy == 0 && dz == 0 {
                                    continue;
                                }
                                if None != self.cube_map.get(&(x + dx, y + dy, z + dz)) {
                                    neighbors += 1;
                                }
                            }
                        }
                    }

                    match self.cube_map.get(&(x, y, z)) {
                        None => {
                            if neighbors == 3 {
                                cube_map.insert((x, y, z), true);

                                if x < xmin {
                                    xmin = x;
                                }
                                if y < ymin {
                                    ymin = y;
                                }
                                if z < zmin {
                                    zmin = z;
                                }
                                if x > xmax {
                                    xmax = x;
                                }
                                if y > ymax {
                                    ymax = y;
                                }
                                if z > zmax {
                                    zmax = z;
                                }
                            }
                        }
                        Some(_) => {
                            if neighbors == 2 || neighbors == 3 {
                                cube_map.insert((x, y, z), true);

                                if x < xmin {
                                    xmin = x;
                                }
                                if y < ymin {
                                    ymin = y;
                                }
                                if z < zmin {
                                    zmin = z;
                                }
                                if x > xmax {
                                    xmax = x;
                                }
                                if y > ymax {
                                    ymax = y;
                                }
                                if z > zmax {
                                    zmax = z;
                                }
                            }
                        }
                    }
                }
            }
        }

        Cubes {
            cube_map,
            xmin,
            ymin,
            zmin,
            xmax,
            ymax,
            zmax,
        }
    }
}

pub fn run(lines: Vec<String>) {
    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<String>) {
    let mut cubes = Cubes::from_lines(&lines);

    for _ in 0..6 {
        cubes = cubes.tick();
    }

    println!("{}", cubes.cube_map.len());
}

fn part2(lines: &Vec<String>) {
    let mut cubes = HyperCubes::from_lines(&lines);

    for _ in 0..6 {
        cubes = cubes.tick();
    }

    println!("{}", cubes.cube_map.len());
}

#[derive(Clone)]
struct HyperCubes {
    // (w, x, y, z) => true
    cube_map: HashMap<(i32, i32, i32, i32), bool>,
    wmin: i32,
    xmin: i32,
    ymin: i32,
    zmin: i32,
    wmax: i32,
    xmax: i32,
    ymax: i32,
    zmax: i32,
}

impl HyperCubes {
    fn from_lines(lines: &Vec<String>) -> HyperCubes {
        let mut cube_map: HashMap<(i32, i32, i32, i32), bool> = HashMap::new();

        let mut x = 0;
        let mut y = 0;
        let z = 0;
        let w: i32 = 0;

        let mut wmin: i32 = 0;
        let mut xmin: i32 = 0;
        let mut ymin: i32 = 0;
        let mut zmin: i32 = 0;
        let mut wmax: i32 = 0;
        let mut xmax: i32 = 0;
        let mut ymax: i32 = 0;
        let mut zmax: i32 = 0;

        for line in lines {
            for c in line.chars() {
                if c == '#' {
                    cube_map.insert((w, x, y, z), true);
                    if w < wmin {
                        wmin = w;
                    }
                    if w < wmin {
                        xmin = x;
                    }
                    if y < ymin {
                        ymin = y;
                    }
                    if z < zmin {
                        zmin = z;
                    }

                    if w > wmax {
                        wmax = w;
                    }
                    if x > xmax {
                        xmax = x;
                    }
                    if y > ymax {
                        ymax = y;
                    }
                    if z > zmax {
                        zmax = z;
                    }
                }
                x += 1;
            }
            y -= 1;
            x = 0;
        }

        HyperCubes {
            cube_map,
            wmin,
            xmin,
            ymin,
            zmin,
            wmax,
            xmax,
            ymax,
            zmax,
        }
    }

    fn tick(&self) -> HyperCubes {
        let mut cube_map: HashMap<(i32, i32, i32, i32), bool> = HashMap::new();

        let mut wmin: i32 = self.wmin;
        let mut xmin: i32 = self.xmin;
        let mut ymin: i32 = self.ymin;
        let mut zmin: i32 = self.zmin;
        let mut wmax: i32 = self.wmax;
        let mut xmax: i32 = self.xmax;
        let mut ymax: i32 = self.ymax;
        let mut zmax: i32 = self.zmax;

        for w in (wmin - 1)..=(wmax + 1) {
            for x in (xmin - 1)..=(xmax + 1) {
                for y in (ymin - 1)..=(ymax + 1) {
                    for z in (zmin - 1)..=(zmax + 1) {
                        let mut neighbors: usize = 0;

                        for dw in -1..=1 {
                            for dx in -1..=1 {
                                for dy in -1..=1 {
                                    for dz in -1..=1 {
                                        if dw == 0 && dx == 0 && dy == 0 && dz == 0 {
                                            continue;
                                        }
                                        if None
                                            != self.cube_map.get(&(w + dw, x + dx, y + dy, z + dz))
                                        {
                                            neighbors += 1;
                                        }
                                    }
                                }
                            }
                        }

                        match self.cube_map.get(&(w, x, y, z)) {
                            None => {
                                if neighbors == 3 {
                                    cube_map.insert((w, x, y, z), true);

                                    if w < wmin {
                                        wmin = w;
                                    }
                                    if x < xmin {
                                        xmin = x;
                                    }
                                    if y < ymin {
                                        ymin = y;
                                    }
                                    if z < zmin {
                                        zmin = z;
                                    }
                                    if w > wmax {
                                        wmax = w;
                                    }
                                    if x > xmax {
                                        xmax = x;
                                    }
                                    if y > ymax {
                                        ymax = y;
                                    }
                                    if z > zmax {
                                        zmax = z;
                                    }
                                }
                            }
                            Some(_) => {
                                if neighbors == 2 || neighbors == 3 {
                                    cube_map.insert((w, x, y, z), true);
                                    if w < wmin {
                                        wmin = w;
                                    }
                                    if x < xmin {
                                        xmin = x;
                                    }
                                    if y < ymin {
                                        ymin = y;
                                    }
                                    if z < zmin {
                                        zmin = z;
                                    }
                                    if w > wmax {
                                        wmax = w;
                                    }
                                    if x > xmax {
                                        xmax = x;
                                    }
                                    if y > ymax {
                                        ymax = y;
                                    }
                                    if z > zmax {
                                        zmax = z;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        HyperCubes {
            cube_map,
            wmin,
            xmin,
            ymin,
            zmin,
            wmax,
            xmax,
            ymax,
            zmax,
        }
    }
}
