use xml::writer::{EventWriter, XmlEvent};

pub fn write_text_element<'a, W>(
    writer: &mut EventWriter<W>,
    tag: &str,
    attributes: Vec<(&str, &str)>,
    content: &str,
) where
    W: std::io::Write,
{
    let mut start = XmlEvent::start_element(tag);
    for (x, y) in attributes {
        start = start.attr(x, y);
    }
    writer.write::<XmlEvent>(start.into()).unwrap();
    writer
        .write::<XmlEvent>(XmlEvent::characters(content).into())
        .unwrap();
    writer
        .write::<XmlEvent>(XmlEvent::end_element().into())
        .unwrap();
}

pub fn write_wrapped_element<'a, W, T>(
    writer: &mut EventWriter<W>,
    tag: &str,
    attributes: Vec<(&str, &str)>,
    inner: T,
) where
    W: std::io::Write,
    T: Fn(&mut EventWriter<W>),
{
    let mut start = XmlEvent::start_element(tag);
    for (x, y) in attributes {
        start = start.attr(x, y);
    }
    writer.write::<XmlEvent>(start.into()).unwrap();
    inner(writer);
    writer
        .write::<XmlEvent>(XmlEvent::end_element().into())
        .unwrap();
}

pub trait WriteXML<W> {
    fn write_xml(&self, w: &mut EventWriter<W>);
}
