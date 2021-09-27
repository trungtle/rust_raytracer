use crate::core::scene::SurfaceInteraction;
use crate::ray::Ray;

pub struct MatteMaterial {

}

impl super::Material for MatteMaterial {
    fn scatter(_ray: &Ray, surface: &SurfaceInteraction) -> super::ScatterResult {

        // Scatter toward a random point inside a unit sphere tangent to the point of intersection.
		// vec3 newTarget = intersect.P + intersect.N + Sampler::RandomSampleInUnitSphere();
		// scatterRay = Ray(intersect.P, newTarget - intersect.P, ray.time);
		// pdf = dot(intersect.N, scatterRay.direction) / M_PI;

        // ONB uvw;
		// uvw.BuildFromW(intersect.N);
		// vec3 direction = uvw.Local(Sampler::RandomCosineDirection());
		// scatterRay = Ray(intersect.P, direction, ray.time);

        // let uvw: super::ONB = ;
        // uvw.build_from(&surface.hit_normal);
        // let direction = uvw.local(&Sampler::sample_cosine_direction());

        super::ScatterResult(true, Ray::new(surface.hit_point, crate::math::Vec3::from(1.)))
    }
}