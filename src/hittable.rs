use crate::{Point3, Ray, Vector3, dot, Interval};

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord::default()
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3) {
        // Sets the hit record normal vector.
        // NOTE: The paramter `outward_normal` is assumed to have unit length.
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec!(),
        }
    }

    pub fn build<T: Hittable + 'static>(object: T) -> Self {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::build(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}