use crate::virt_util::xml::*;
use serde::Serialize;
use xml::EventWriter;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DiskDevice {
    Disk,
    CdRom,
}

#[derive(Clone, Debug, new)]
pub struct Disk {
    driver_type: String,
    source: String,
    device: DiskDevice,
    readonly: bool,
    target_dev: String,
    target_bus: String,
}

#[derive(Clone, Debug, new)]
pub struct Graphics {
    gtype: String,
    port: String,
    autoport: String,
}

#[derive(Clone, Debug)]
pub enum DeviceXML {
    Disk(Disk),
    Graphics(Graphics),
}

impl<W: std::io::Write> WriteXML<W> for DeviceXML {
    fn write_xml(&self, w: &mut EventWriter<W>) {
        match self {
            DeviceXML::Disk(d) => {
                let device = serde_plain::to_string(&d.device).unwrap();
                write_wrapped_element(
                    w,
                    "disk",
                    vec![("type", "file"), ("device", &device)],
                    |w| {
                        write_text_element(
                            w,
                            "driver",
                            vec![("name", "qemu"), ("type", &d.driver_type)],
                            "",
                        );
                        write_text_element(w, "source", vec![("file", &d.source)], "");
                        if d.readonly {
                            write_text_element(w, "readonly", vec![], "");
                        }
                        write_text_element(
                            w,
                            "target",
                            vec![("dev", &d.target_dev), ("bus", &d.target_bus)],
                            "",
                        );
                    },
                );
            }
            DeviceXML::Graphics(g) => {
                write_text_element(
                    w,
                    "graphics",
                    vec![
                        ("type", &g.gtype),
                        ("port", &g.port),
                        ("autoport", &g.autoport),
                    ],
                    "",
                );
            }
        }
    }
}
