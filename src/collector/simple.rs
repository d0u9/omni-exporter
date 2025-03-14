use crate::collector::types::Sensor;
use crate::error::Result;
use crate::sensor::InternalSensorReader;
use crate::types::Metric;
use crate::types::MetricLabels;
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

    pub async fn collect(&self) -> Result<Vec<Metric<Vec<(String, String)>>>> {
        let mut metrics = Vec::new();
        for sensor in &self.sensors {
            match sensor {
                Sensor::System(sensor) => {
                    let m = sensor.update().await?;
                    metrics.extend(m.into_iter().map(|metric| Metric {
                        mtype: metric.mtype,
                        value: metric.value,
                        labels: metric.labels.labels().collect(),
                    }));
                }
                Sensor::External(sensor) => {
                    let m = sensor.update().await?;
                    metrics.extend(m.into_iter().map(|metric| Metric {
                        mtype: metric.mtype,
                        value: metric.value,
                        labels: metric.labels.labels().collect(),
                    }));
                }
            }
        }
        Ok(metrics)
    }
}
