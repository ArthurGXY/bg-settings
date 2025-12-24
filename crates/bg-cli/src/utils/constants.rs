use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone)]
#[derive(PartialEq)]
pub enum ListTarget {
    // seats
    Seat,
    Seats,
    // outputs
    Output,
    Outputs,
    // backends
    Backend,
    Backends,
    // help
    Command,
    Commands,
    // static + animated
    Media,
    // static
    Static,
    Image,
    Images,
    // animated
    Animated,
    Video,
    Videos
}

pub const SEAT: [ListTarget; 2] = [
    ListTarget::Seat, ListTarget::Seats];

pub const OUTPUT: [ListTarget; 2] = [
    ListTarget::Output, ListTarget::Outputs];
pub const HELP: [ListTarget; 2] = [
    ListTarget::Command, ListTarget::Commands];

pub const BACKEND: [ListTarget; 2] = [
    ListTarget::Backend, ListTarget::Backends];
pub const STATIC_MEDIA: [ListTarget; 3] = [
    ListTarget::Static, ListTarget::Image, ListTarget::Images];
pub const ANIMATED_MEDIA: [ListTarget; 3] = [
    ListTarget::Animated, ListTarget::Video, ListTarget::Videos];


impl ListTarget {
    pub fn requires_media_path(&self) -> bool {
        matches!(
            self,
            // static + animated
            ListTarget::Media |
            // static
            ListTarget::Static | ListTarget::Image | ListTarget::Images |
            // animated
            ListTarget::Animated | ListTarget::Video | ListTarget::Videos
        )
    }
    pub fn is_in(&self, group: &[ListTarget]) -> bool {
        group.contains(self)
    }
}
