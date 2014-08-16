pub trait Tag {
    fn get_tag(&self) -> &'static str;
}
