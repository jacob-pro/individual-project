use crate::virt_util::devices::DeviceXML;
use crate::virt_util::os::Os;
use crate::virt_util::xml::{write_text_element, write_wrapped_element, WriteXML};
use anyhow::Context;
use xml::writer::EventWriter;
use xml::EmitterConfig;

// https://libvirt.org/formatdomain.html
pub struct DomainXml {
    name: String,
    cpus: u32,
    os: Os,
    memory: u32,
    devices: Vec<DeviceXML>,
}

impl DomainXml {
    pub fn builder() -> DomainXmlBuilder {
        DomainXmlBuilder::default()
    }

    pub fn to_xml(&self) -> String {
        let config = EmitterConfig::new()
            .line_separator("\n")
            .perform_indent(true);
        let mut writer = EventWriter::new_with_config(Vec::new(), config);
        write_wrapped_element(&mut writer, "domain", vec![("type", "kvm")], |x| {
            self.write_xml(x)
        });
        String::from_utf8(writer.into_inner()).unwrap()
    }
}

impl<W: std::io::Write> WriteXML<W> for DomainXml {
    fn write_xml(&self, w: &mut EventWriter<W>) {
        let cpus = self.cpus.to_string();
        let mem = self.memory.to_string();
        write_text_element(w, "name", vec![], self.name.as_str());
        write_text_element(w, "vcpu", vec![], cpus.as_str());
        write_text_element(w, "memory", vec![("unit", "MiB")], mem.as_str());
        write_wrapped_element(w, "os", vec![], |w| self.os.write_xml(w));
        write_wrapped_element(w, "devices", vec![], |w| {
            for d in &self.devices {
                d.write_xml(w);
            }
        });
    }
}

#[derive(Default, Clone, Debug)]
pub struct DomainXmlBuilder {
    name: Option<String>,
    cpus: Option<u32>,
    memory: Option<u32>,
    devices: Vec<DeviceXML>,
}

impl DomainXmlBuilder {
    pub fn build(self) -> anyhow::Result<DomainXml> {
        Ok(DomainXml {
            name: self.name.with_context(|| "Missing name")?,
            cpus: self.cpus.unwrap_or(1),
            os: Os {},
            memory: self.memory.unwrap_or(512),
            devices: self.devices,
        })
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn cpus(mut self, cpus: Option<u32>) -> Self {
        self.cpus = cpus;
        self
    }

    pub fn memory(mut self, memory: Option<u32>) -> Self {
        self.memory = memory;
        self
    }

    pub fn device(mut self, device: DeviceXML) -> Self {
        self.devices.push(device);
        self
    }

    pub fn devices(mut self, devices: Vec<DeviceXML>) -> Self {
        self.devices = devices;
        self
    }
}
