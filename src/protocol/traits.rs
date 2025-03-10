pub trait Protocol {
    fn add_item(&mut self, v: &str);
    fn get_item(&self) -> &str;
}
