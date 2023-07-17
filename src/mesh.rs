use std::f64::consts::PI;


pub struct Mesh2D {
    vertices: Vec<(f64, f64)>,
    polygons: Vec<Vec<usize>>,  // a polygon is a vector of vertex indices.
    mid_z: Vec<(usize, usize)>, // middle Z segments.
}

impl Mesh2D {
    pub fn regular_polygon(sides: usize) -> Mesh2D {
        let mut vertices: Vec<(f64, f64)> = Vec::new();
        let mut polygons: Vec<Vec<usize>> = Vec::new();
        let mut polygon: Vec<usize> = Vec::new();
        let mid_z: Vec<(usize, usize)> = Vec::new();

        let delta: f64 = 2.0*PI / (sides as f64);
        for n in 0..sides {
            let theta = delta * (n as f64);
            let x = theta.cos();
            let y = theta.sin();
            vertices.push((x, y));
            polygon.push(n);
        }
        polygons.push(polygon);

        Mesh2D {vertices, polygons, mid_z}
    }

    pub fn num_polygons(&self) -> usize {
        self.polygons.len()
    }
}