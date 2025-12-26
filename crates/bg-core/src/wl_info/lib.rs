use std::fmt::Display;
use std::collections::HashMap;
use wayland_client::{
    backend::ObjectId, protocol::{wl_output::{self},
                                  wl_registry::{self},
                                  wl_seat::{self}}, Connection, Dispatch, Proxy, QueueHandle, WEnum
};


#[derive(Debug)]
pub struct SeatInfo {
    protocol_id: u32,
    name: String,
    capabilities: Vec<String>,
}

impl Display for SeatInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "id: {}", self.protocol_id)?;
        writeln!(f, "name: {}", self.name)?;
        writeln!(f, "capabilities: {:?}", self.capabilities)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct OutputInfo {
    protocol_id: u32,
    pub name: String,
    description: String,
    x: i32,
    y: i32,
    scale: i32,
    physical_width: i32,
    physical_height: i32,
    make: String,
    model: String,
    subpixel_orientation: String,
    output_transform: String,
    modes: Vec<OutputMode>,
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

#[derive(Debug)]
pub struct OutputMode {
    width: i32,
    height: i32,
    refresh: i32,
    flags: Vec<String>,
}

struct State {
    seat: HashMap<ObjectId, SeatInfo>,
    outputs: HashMap<ObjectId, OutputInfo>,
}

impl State {
    fn new() -> Self {
        Self {
            seat: HashMap::new(),
            outputs: HashMap::new(),
        }
    }
}


impl Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        _: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global { name, interface, version } = event {
            match interface.as_str() {
                "wl_output" => {
                    registry.bind::<wl_output::WlOutput, _, _>(
                        name,
                        version.min(4),
                        qh,
                        (),
                    );
                }
                "wl_seat" => {
                    registry.bind::<wl_seat::WlSeat, _, _>(
                        name,
                        version.min(7),
                        qh,
                        (),
                    );
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<wl_output::WlOutput, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &wl_output::WlOutput,
        event: wl_output::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let id = proxy.id();

        let info = state.outputs.entry(id.clone()).or_insert_with(|| OutputInfo {
            protocol_id: id.protocol_id(),
            name: String::new(),
            description: String::new(),
            x: 0,
            y: 0,
            scale: 1,
            physical_width: 0,
            physical_height: 0,
            make: String::new(),
            model: String::new(),
            subpixel_orientation: String::new(),
            output_transform: String::new(),
            modes: Vec::new(),
        });

        match event {
            wl_output::Event::Name { name } => {
                info.name = name;
            }
            wl_output::Event::Description { description } => {
                info.description = description;
            }
            wl_output::Event::Geometry {
                x,
                y,
                physical_width,
                physical_height,
                subpixel,
                make,
                model,
                transform,
            } => {
                info.x = x;
                info.y = y;
                info.physical_width = physical_width;
                info.physical_height = physical_height;
                info.make = make;
                info.model = model;
                info.subpixel_orientation = format!("{:?}", subpixel);
                info.output_transform = format!("{:?}", transform);
            }
            wl_output::Event::Scale { factor } => {
                info.scale = factor;
            }
            wl_output::Event::Mode {
                width,
                height,
                refresh,
                flags,
            } => {
                let mut fs = Vec::new();
                match flags {
                    WEnum::Value(mode) => {
                        if mode == wl_output::Mode::Current {
                            fs.push("current".into());
                        }
                        if mode == wl_output::Mode::Preferred {
                            fs.push("preferred".into());
                        }
                    }
                    WEnum::Unknown(_) => {}
                }


                info.modes.push(OutputMode {
                    width,
                    height,
                    refresh,
                    flags: fs,
                });
            }
            wl_output::Event::Done => {}
            _ => {}
        }
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &wl_seat::WlSeat,
        event: wl_seat::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let id = proxy.id();

        let info = state.seat.entry(id.clone()).or_insert_with(|| SeatInfo {
            protocol_id: id.protocol_id(),
            name: String::new(),
            capabilities: Vec::new(),
        });

        match event {
            wl_seat::Event::Name { name } => {
                info.name = name;
            }
            wl_seat::Event::Capabilities { capabilities } => {
                info.capabilities.clear();

                match capabilities {
                    WEnum::Value(caps) => {
                        if caps.contains(wl_seat::Capability::Keyboard)  {
                            info.capabilities.push("keyboard".into());
                        }
                        if caps.contains(wl_seat::Capability::Pointer) {
                            info.capabilities.push("pointer".into());
                        }
                        if caps.contains(wl_seat::Capability::Touch) {
                            info.capabilities.push("touch".into());
                        }
                    }
                    WEnum::Unknown(_) => {
                        // compositor 发了你不认识的 capability
                    }
                }
                    }
            _ => {},
        }
    }
}

pub fn get_info() -> (Vec<OutputInfo>, Vec<SeatInfo>) {
    let conn = Connection::connect_to_env()
        .expect("Failed to connect to Wayland");

    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let display = conn.display();
    let mut state = State::new();

    display.get_registry(&qh, ());

    // registry → bind
    event_queue.roundtrip(&mut state).unwrap();
    // output / seat → 吐事件
    event_queue.roundtrip(&mut state).unwrap();

    (
        state.outputs.into_values().collect(),
        state.seat.into_values().collect(),
    )
}

pub fn get_output_by_name(name: &str) -> Option<OutputInfo> {
    get_info().0.into_iter().find(|o| {o.name == name})
}
