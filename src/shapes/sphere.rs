use crate::{HitRecord, Hittable, Point3, Ray, dot};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    /// Creates a new sphere at the specified location with the specified radius.
    /// 
    /// # Examples
    /// ```
    /// use raytracer::{Sphere,Point3};
    /// let s = Sphere::new(Point3::new(1.0, 2.0, 3.0), 5.0);
    /// 
    /// assert_eq!(s.center, Point3::new(1.0, 2.0, 3.0));
    /// assert_eq!(s.radius, 5.0);
    /// ```
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { 
            center,
            radius: f64::max(radius, 0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminent = h * h - a * c;
        if discriminent < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminent);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}
