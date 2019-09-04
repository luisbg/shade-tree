use crate::material::Material;
use crate::ray::Ray;
use crate::visible::{HitRecord, Visible};

#[derive(Default)]
pub struct World {
    obj_list: Vec<Box<dyn Visible>>,
}

impl World {
    pub fn add(&mut self, obj: Box<dyn Visible>) {
        self.obj_list.push(obj);
    }
}

impl Visible for World {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest = t_max;
        for obj in self.obj_list.iter() {
            let mut tmp_rec = HitRecord::default();

            if obj.hit(ray, t_min, t_max, &mut tmp_rec) && tmp_rec.t < closest {
                hit_anything = true;
                closest = tmp_rec.t;
                rec.t = tmp_rec.t;
                rec.p = tmp_rec.p;
                rec.normal = tmp_rec.normal;
                rec.material = tmp_rec.material;
            }
        }

        hit_anything
    }

    fn set_material(&mut self, _m: Material) {
        println!("ERROR: Don't call set_material on the World.");
        unreachable!();
    }
}
