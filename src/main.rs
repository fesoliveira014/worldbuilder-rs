mod color;
mod canvas;

extern crate voronator;
extern crate skia_safe;

use voronator::delaunator::Point;
use rand::prelude::*;
use std::fs::File;
use std::io::Write;

fn get_points(n: i32, jitter: f64) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut points: Vec<Point> = vec![];
    for i in 0..n+1 {
        for j in 0..n+1 {
            points.push(Point{
                x: (i as f64) + jitter * (rng.gen::<f64>() - rng.gen::<f64>()), 
                y: (j as f64) + jitter * (rng.gen::<f64>() - rng.gen::<f64>()) 
            });
        }
    }
    
    points
}

fn draw_poly(canvas: &mut canvas::Canvas, poly: &[(f32, f32)]) {
    canvas.begin_path();
    canvas.move_to(poly[0].0, poly[0].1);
     for i in 1..poly.len() {
         canvas.line_to(poly[i].0, poly[i].1);
     }
     canvas.close_path();
     canvas.stroke();
}

fn main() {
    let mut canvas = canvas::Canvas::new(1000, 1000);
    // let spectral_colors = color::SpectralColors::new();
    let num_points = 50;

    let points: Vec<Point> = get_points(num_points, 0.6)
        .into_iter()
        .map(|p| Point{
            x: ((canvas.width() as f64) / 20. + p.x * (canvas.width() as f64)) / (num_points as f64), 
            y: ((canvas.height() as f64) / 20. + p.y * (canvas.height() as f64)) / (num_points as f64)
        }).collect();


    // let diagram = voronator::VoronoiDiagram::new(
    //         &Point{x: 0., y: 0.}, 
    //         &Point{x: canvas.width() as f64, y: canvas.height() as f64}, 
    //         &points
    //     )
    //     .unwrap();
    let diagram = voronator::CentroidDiagram::new(&points).unwrap();

    canvas.set_line_width(1.0);
    
    for (site, cell) in diagram.sites.iter().zip(diagram.cells.iter()) {
        let s = (site.x as f32, site.y as f32);
        let p: Vec<(f32, f32)> = cell.into_iter().map(|x| (x.x as f32, x.y as f32)).collect();
        
        for i in 0..p.len()-1 {
            draw_poly(&mut canvas, &[s, p[i], p[i+1]]);
        }

        draw_poly(&mut canvas, &[s, p[0], p[p.len()-1]]);
    }
        
    
    let d = canvas.data();
    let mut file = File::create("test.png").unwrap();
    let bytes = d.as_bytes();
    file.write_all(bytes).unwrap();
}
