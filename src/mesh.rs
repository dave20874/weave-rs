use std::{f32::consts::PI, collections::HashMap};

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

    pub fn find_poly_center(&self, poly: &Vec<usize>) -> (f32, f32) {
        let mut cx: f32 = 0.0;
        let mut cy: f32 = 0.0;
        let mut n = 0;

        for pt_id in poly {
            let p = self.vertices[*pt_id];
            cx += p.x;
            cy += p.y;
            n += 1;
        }
        assert!( n > 0 );
        cx /= n as f32;
        cy /= n as f32;

        (cx, cy)
    }

    pub fn get_mid_pts(points: (usize, usize), 
                       vertices: &mut Vec<Point>, 
                       expanded_edges: &mut HashMap<(usize, usize), (usize, usize)>) 
                       -> (usize, usize) {
        println!("Getting mid points of points {} and {}", points.0, points.1);

        // Keep track of whether we reversed the input points
        let mut reversed = false;

        let (p1, p2) = if points.0 <= points.1 {
            // don't reverse inputs
            (points.0, points.1)
        }
        else {
            // reverse inputs
            reversed = true;
            (points.1, points.0)
        };

        let (p3, p4) = if expanded_edges.contains_key(&(p1, p2)) {
            // We already generated intermediate points for this edge
            *expanded_edges.get(&(p1, p2)).unwrap()
        }
        else {
            // These points haven't been generated yet so generate them
            let p1_point = vertices[p1];
            let p2_point = vertices[p2];
            let r0 = (p2_point.x - p1_point.x).hypot(p2_point.y-p1_point.y);
            let theta0 = (p2_point.y-p1_point.y).atan2(p2_point.x-p1_point.x);
            let r_segment = r0 * 0.377964473;  // 1/sqrt(7)

            let dx = r_segment*(theta0+0.33347).cos();
            let dy = r_segment*(theta0+0.33347).sin();

            let p3x = p1_point.x + dx;
            let p3y = p1_point.y + dy;
            let p4x = p2_point.x - dx;
            let p4y = p2_point.y - dy;
            let p3 = vertices.len();
            let p4 = p3+1;

            let new_ids = (vertices.len());

            vertices.push(Point{x:p3x, y:p3y});
            vertices.push(Point{x:p4x, y:p4y});
            expanded_edges.insert((p1, p2), (p3, p4));

            (p3, p4)
        };

        if reversed {
            (p4, p3)
        }
        else {
            (p3, p4)
        }
    }

    pub fn penta_decomp(&self) -> Mesh2D {
        let mut vertices: Vec<Point> = Vec::new();
        let mut polygons: Vec<Vec<usize>> = Vec::new();
        let mut mid_z: Vec<(usize, usize)> = Vec::new();

        // To start, all the vertices of this polygon go forward into the next one
        for v in &self.vertices {
            vertices.push(v.clone());
        }

        // when we split up edges, we'll keep track of them here.
        let mut expanded_edges: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        // For each polygon in this mesh, create new polygons from it's edges
        for poly in &self.polygons {
            let poly_len = poly.len();

            // Find center point, register it as a new vertex
            let (cx, cy) = self.find_poly_center(&poly);
            vertices.push(Point { x: cx, y: cy } );
            let a = vertices.len()-1;

            // For each corner of this polygon
            for corner in 0..poly.len() {
                // get index of next CCW corner
                let corner_pt = poly[corner];
                let cw = poly[(corner + poly_len-1) % (poly.len())];
                let ccw = poly[(corner +1) % (poly.len())];
                // get indexes of points from CW to this corner
                let (b, c) = Mesh2D::get_mid_pts((cw, corner_pt), &mut vertices, &mut expanded_edges);
                let (e, f) = Mesh2D::get_mid_pts((corner_pt, ccw), &mut vertices, &mut expanded_edges);

                // record polygon from a (center) to b to c to d (corner) to e to a (center)
                polygons.push(vec![a, b, c, corner_pt, e]);

                // record mid_z segment from b to c
                mid_z.push((b, c));
            }
        }

        Mesh2D {vertices, polygons, mid_z}
    }
}