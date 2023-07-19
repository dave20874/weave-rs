use std::f32::consts::PI;

use iced::{widget::canvas::path::Builder, Point};


pub struct Mesh2D {
    vertices: Vec<Point>,
    polygons: Vec<Vec<usize>>,  // a polygon is a vector of vertex indices.
    mid_z: Vec<(usize, usize)>, // middle Z segments.
}

impl Mesh2D {
    pub fn regular_polygon(sides: usize) -> Mesh2D {
        let mut vertices: Vec<Point> = Vec::new();
        let mut polygons: Vec<Vec<usize>> = Vec::new();
        let mut polygon: Vec<usize> = Vec::new();
        let mid_z: Vec<(usize, usize)> = Vec::new();

        let delta: f32 = 2.0*PI / (sides as f32);
        for n in 0..sides {
            let theta = delta * (n as f32);
            let x = theta.cos();
            let y = theta.sin();
            vertices.push(Point{x, y});
            polygon.push(n);
        }
        polygons.push(polygon);

        Mesh2D {vertices, polygons, mid_z}
    }

    pub fn num_polygons(&self) -> usize {
        self.polygons.len()
    }

    pub fn build(&self, builder: &mut Builder, scale_x:f32, scale_y:f32, cx:f32, cy:f32) -> () {

        for polygon in &self.polygons {

            let start_x = self.vertices[polygon[0]].x * scale_x + cx;
            let start_y = self.vertices[polygon[0]].y * scale_y + cy;
            builder.move_to(Point{x: start_x, y: start_y});
            for n in 1..polygon.len() {
                let x = self.vertices[polygon[n]].x * scale_x + cx;
                let y = self.vertices[polygon[n]].y * scale_y + cy;
                builder.line_to(Point{x, y});
            }
            builder.close();
        }
    }
}