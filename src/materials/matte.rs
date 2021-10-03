use std::sync::Arc;

use crate::core::interaction::SurfaceInteraction;
use crate::core::spectrum::Spectrum;
use crate::core::texture::Texture;
use crate::ray::Ray;

pub struct MatteMaterial {
    pub kd: Arc<dyn Texture<Spectrum> + Sync + Send>,
    pub sigma: Arc<dyn Texture<f64> + Sync + Send>
}

impl MatteMaterial{
    fn scatter(_ray: &Ray, surface: &SurfaceInteraction) {

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
    }
}