pub mod creatures;
mod pixel_image;
mod growth_stages;
mod shape_adjustment;
mod pixel_vector;

pub use pixel_vector::PixelVectorShape;
pub use pixel_image::PixelImage;
pub use growth_stages::GrowthStageShapes;
pub use shape_adjustment::move_pixel_image;
