use crate::virt_util::xml::*;
use xml::EventWriter;

pub struct Os {}

impl<W: std::io::Write> WriteXML<W> for Os {
    fn write_xml(&self, w: &mut EventWriter<W>) {
        write_text_element(w, "type", vec![("arch", "x86_64")], "hvm");
    }
}
