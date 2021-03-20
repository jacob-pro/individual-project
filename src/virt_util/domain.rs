use crate::virt_util::os::Os;
use crate::virt_util::xml::{write_text_element, write_wrapped_element};
use anyhow::Context;
use xml::writer::{EventWriter, XmlEvent};
use xml::EmitterConfig;

// https://libvirt.org/formatdomain.html
pub struct DomainXml {
    name: String,
    cpus: u32,
    os: Os,
    memory: u32,
}

impl DomainXml {
    pub fn builder() -> DomainXmlBuilder {
        DomainXmlBuilder::default()
    }

    fn xml_events<W: std::io::Write>(&self, w: &mut EventWriter<W>) {
        let cpus = self.cpus.to_string();
        let mem = self.memory.to_string();
        write_text_element(w, XmlEvent::start_element("name"), self.name.as_str());
        write_text_element(w, XmlEvent::start_element("vcpu"), cpus.as_str());
        write_text_element(
            w,
            XmlEvent::start_element("memory").attr("unit", "MiB"),
            mem.as_str(),
        );
        write_wrapped_element(w, XmlEvent::start_element("os"), |x| self.os.xml_events(x));
    }

    pub fn to_xml(&self) -> String {
        let config = EmitterConfig::new()
            .line_separator("\n")
            .perform_indent(true);
        let mut writer = EventWriter::new_with_config(Vec::new(), config);
        write_wrapped_element(
            &mut writer,
            XmlEvent::start_element("domain").attr("type", "kvm"),
            |x| self.xml_events(x),
        );
        String::from_utf8(writer.into_inner()).unwrap()
    }
}

#[derive(Default, Clone, Debug)]
pub struct DomainXmlBuilder {
    name: Option<String>,
    cpus: Option<u32>,
    memory: Option<u32>,
}

impl DomainXmlBuilder {
    pub fn build(self) -> anyhow::Result<DomainXml> {
        Ok(DomainXml {
            name: self.name.with_context(|| "Missing name")?,
            cpus: self.cpus.unwrap_or(1),
            os: Os {},
            memory: self.memory.unwrap_or(512),
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
}
