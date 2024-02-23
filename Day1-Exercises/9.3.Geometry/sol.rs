fn magnitude(v: &[f64; 3]) -> f64 {
   (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt()
}

fn normalize(v: &mut [f64; 3]) {
   let m = magnitude(v);
   v[0] /= m;
   v[1] /= m;
   v[2] /= m;
}

fn main() {
   println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

   let mut v = [1.0, 2.0, 9.0];
   println!("Magnitude of {:?}: {}", v, magnitude(&v));
   normalize(&mut v);
   println!("Magnitude of {:?} after normalization: {}", v, magnitude(&v));
}