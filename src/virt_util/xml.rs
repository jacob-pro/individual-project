use xml::writer::{EventWriter, XmlEvent};

pub fn write_text_element<'a, W, Y>(writer: &mut EventWriter<W>, start: Y, text: &str)
where
    W: std::io::Write,
    Y: Into<XmlEvent<'a>>,
{
    writer.write::<XmlEvent>(start.into()).unwrap();
    writer
        .write::<XmlEvent>(XmlEvent::characters(text).into())
        .unwrap();
    writer
        .write::<XmlEvent>(XmlEvent::end_element().into())
        .unwrap();
}

pub fn write_wrapped_element<'a, W, Y, T>(writer: &mut EventWriter<W>, start: Y, inner: T)
where
    W: std::io::Write,
    Y: Into<XmlEvent<'a>>,
    T: Fn(&mut EventWriter<W>),
{
    writer.write(start).unwrap();
    inner(writer);
    writer
        .write::<XmlEvent>(XmlEvent::end_element().into())
        .unwrap();
}
