use super::SensorData;
use std::marker::PhantomData;

pub struct OwnedSensorData<T> {
    data: String,
    _phantom: PhantomData<T>,
}

impl<T: Sized + Send + Sync + 'static> OwnedSensorData<T> {
    pub fn new() -> Self {
        Self {
            data: "".to_string(),
            _phantom: PhantomData,
        }
    }
}

impl<T: Sized + Send + Sync + 'static> SensorData for OwnedSensorData<T> {
    fn name(&self) -> &str {
        "OwnedSensorData"
    }

    fn from_str(s: &str) -> Self {
        Self {
            data: s.to_string(),
            _phantom: PhantomData,
        }
    }
}
