use ggez::{GameResult, Context, graphics::Image, audio::Source};

pub struct Assets {
    jacc_in_the_box: Image,
    jacc_out_of_box: Image,
    // jacc_sound: Source
}

impl Assets {
    pub fn new(ctx: &Context) -> GameResult<Assets> {
        // Initialize all assets ...
        Ok(
            Assets { 
                jacc_in_the_box: Image::from_path(ctx, "/JaccInTheBox.png").unwrap(), 
                jacc_out_of_box: Image::from_path(ctx, "/JaccOutOfBox.png").unwrap(), 
                // jacc_sound: Source::new(ctx, "/pewpew.ogg").unwrap() 
            }
        )
    }

    pub fn get_jacc_in_the_box_image(&self) -> &Image { &self.jacc_in_the_box }
    pub fn get_jacc_out_of_box_image(&self) -> &Image { &self.jacc_out_of_box }
    // pub fn get_jacc_sound_source(&self) -> &Source { &self.jacc_sound }
}