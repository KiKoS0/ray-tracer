use super::hit::HitRecord;
use super::hit::Hittable;
use crate::math::Ray;
use std::sync::Arc;

pub struct HittableList<T: Hittable + Send + Sync> {
    objects: Vec<Arc<T>>,
}

impl<T: Hittable +Send + Sync> HittableList<T> {
    pub fn add(&mut self, obj: Arc<T>) {
        self.objects.push(obj);
    }

    pub fn new() -> Self {
        HittableList{
            objects: Vec::<Arc<T>>::new()
        }
    }
}

impl<T: Hittable +Send + Sync> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let mut hit = HitRecord::Miss;
        let mut closest = t_max;
        for obj in &self.objects {
            // Ugly but no idea how to do it otherwise
            if let h @ HitRecord::Hit{..} =  obj.hit(ray, t_min, closest) {
                if let HitRecord::Hit{t, ..} = h {
                    closest = t;
                }
                hit = h;
            }
        }
        hit
    }
}
