use parry2d::{math::Real, bounding_volume::BoundingVolume, na::ComplexField};

// use num_traits::identities::Zero;
fn main() {
    let len = parry2d::math::Vector::new(1.0, 1.0).scale(2.0).len();
    println!("len: {}", len);
    let len = (glam::vec2(1.0, 1.0) * 2.0).length();

    println!("len: {}", len);
    // parry2d::bounding_volume::Aabb::new_invalid().contains();
    ComplexField::powf(10.0, 2.0);
}
