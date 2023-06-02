pub struct SerializedEvent<'a, T> {
    pub event: &'a str,
    pub(crate) deserialize: fn(&str) -> T,
}

impl <T> SerializedEvent<'_, T> {
    pub fn deserialize(&self) -> T {
        (self.deserialize)(&self.event)
    }
}

