use std::fmt::Display;
use log::info;
use crate::wl::OutputInfo;

#[allow(dead_code)]
fn select_outputs(
    existing: Vec<OutputInfo>,
    target: Option<Vec<String>>,
) -> Vec<OutputInfo> {
    match target {
        Some(names) => existing
            .into_iter()
            .filter(|o| names.contains(&o.name))
            .inspect(|o| info!("Found output {}...", o.name))
            .collect(),
        None => {
            info!("No output selected, defaulting to all outputs");
            existing
        }
    }
}

impl Display for OutputInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "id: {}", self.protocol_id)?;
        writeln!(f, "name: {}", self.name)?;
        writeln!(f, "description: {}", self.description)?;
        writeln!(f, "x: {}, y: {}, scale: {}", self.x, self.y, self.scale)?;
        writeln!(f, "physical_width: {}, physical_height: {}", self.physical_width, self.physical_height)?;
        writeln!(f, "make: {}", self.make)?;
        writeln!(f, "model: {}", self.model)?;
        writeln!(f, "subpixel_orientation: {}", self.subpixel_orientation)?;
        writeln!(f, "output_transform: {}", self.output_transform)?;
        writeln!(f, "modes: {:?}", self.modes)?;

        Ok(())
    }
}



pub fn get_output_by_name(name: &str) -> Option<OutputInfo> {
    crate::wl::get_info().0.into_iter().find(|o| {o.name == name})
}