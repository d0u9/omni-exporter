use crate::collector::types::Sensor;
use crate::error::Result;
use crate::metric::Metrics;
use crate::sensor::SensorReader;

pub struct SimpleCollector {
    sensors: Vec<Sensor>,
}

impl SimpleCollector {
    pub fn new() -> Self {
        Self { sensors: vec![] }
    }

    pub fn add_sensor(&mut self, sensor: Sensor) {
        self.sensors.push(sensor);
    }

    pub async fn collect(&self) -> Result<Metrics> {
        let mut metrics = Metrics::new();
        for sensor in &self.sensors {
            match sensor {
                Sensor::System(sensor) => {
                    let m = sensor.read().await?;
                    metrics.extend(m);
                }
                Sensor::Sysctl(sensor) => {
                    let m = sensor.read().await?;
                    metrics.extend(m);
                }
                Sensor::External(sensor) => {
                    let m = sensor.read().await?;
                    metrics.extend(m);
                }
            }
        }
        Ok(metrics)
    }
}
