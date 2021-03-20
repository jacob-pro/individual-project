use crate::virt_util::xml::write_text_element;
use xml::writer::XmlEvent;
use xml::EventWriter;

pub struct Os {}

impl Os {
    pub(crate) fn xml_events<W: std::io::Write>(&self, w: &mut EventWriter<W>) {
        write_text_element(w, XmlEvent::start_element("type"), "hvm");
    }
}
