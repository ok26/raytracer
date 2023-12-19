use crate::objects::{triangle::Triangle, material::Material};
use super::vector::Vec3;

pub fn parse_obj(data: &str) -> Vec<Triangle> {
    let mut vertices: Vec<Vec3> = vec![];
    let mut triangles: Vec<Triangle> = vec![];

    for line in data.lines() {

        if line.starts_with("v ") {
            let mut coords = line.split_whitespace().skip(1);
            let x = coords.next().unwrap().parse::<f64>().unwrap();
            let y = coords.next().unwrap().parse::<f64>().unwrap();
            let z = coords.next().unwrap().parse::<f64>().unwrap();
            vertices.push(Vec3::new([x, y, z]));
        }

        else if line.starts_with("f ") {
            let mut coords = line.split_whitespace().skip(1);
            let a = coords.next().unwrap().split("/").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
            let b = coords.next().unwrap().split("/").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
            let c = coords.next().unwrap().split("/").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
            triangles.push(Triangle::new(
                vertices[a - 1],
                vertices[b - 1],
                vertices[c - 1],
                Material::blank()
            ))
        }
    }
    println!("{}", triangles.len());
    triangles
}

pub fn parse_vex(data: &str) -> Vec<Triangle> {
    let mut points: Vec<Vec3> = vec![];
    for line in data.lines() {
        let mut coords = line.split_whitespace();
        let x = coords.next().unwrap().parse::<f64>().unwrap();
        let y = coords.next().unwrap().parse::<f64>().unwrap();
        let z = coords.next().unwrap().parse::<f64>().unwrap();
        points.push(Vec3::new([x, y, z]));
    }

    let mut triangles: Vec<Triangle> = vec![];
    for i in (2..points.len()).step_by(3) {
        triangles.push(Triangle::new(
            points[i - 2],
            points[i - 1],
            points[i],
            Material::blank()
        ))
    }
    println!("{}", triangles.len());
    triangles
}