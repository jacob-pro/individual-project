use crate::virt_util::xml_tools::*;
use crate::virt_util::*;
use xml::EventWriter;

#[derive(Clone, Debug, new)]
pub struct DiskXml {
    driver_type: DiskDriverType,
    source: String,
    device_type: DiskDeviceType,
    readonly: bool,
    target_dev: String,
    target_bus: TargetBus,
}

#[derive(Clone, Debug, new)]
pub struct GraphicsXml {
    gtype: String,
    port: String,
    autoport: String,
}

#[derive(Clone, Debug)]
pub enum DeviceXML {
    Disk(DiskXml),
    Graphics(GraphicsXml),
}

impl<W: std::io::Write> WriteXML<W> for DeviceXML {
    fn write_xml(&self, w: &mut EventWriter<W>) {
        match self {
            DeviceXML::Disk(d) => {
                let device = serde_plain::to_string(&d.device_type).unwrap();
                let target_bus = serde_plain::to_string(&d.target_bus).unwrap();
                write_wrapped_element(
                    w,
                    "disk",
                    vec![("type", "file"), ("device", &device)],
                    |w| {
                        let driver_type = serde_plain::to_string(&d.driver_type).unwrap();
                        write_text_element(
                            w,
                            "driver",
                            vec![("name", "qemu"), ("type", &driver_type)],
                            "",
                        );
                        write_text_element(w, "source", vec![("file", &d.source)], "");
                        if d.readonly {
                            write_text_element(w, "readonly", vec![], "");
                        }
                        write_text_element(
                            w,
                            "target",
                            vec![("dev", &d.target_dev), ("bus", &target_bus)],
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
