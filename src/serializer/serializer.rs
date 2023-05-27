pub trait Serializer<T> {
    fn serialize(&mut self, value: &T) -> std::io::Result<()>;
}
