use super::ExporterResult;
use super::traits::Exporter;

// SimpleExporter schedules grabing data from sensors synchronously.
// Each time the scrape() method is called, it will read data from all sensors and return the result.
pub struct Simple {}

impl Simple {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn do_scrape(&self) -> ExporterResult<String> {
        Ok("Hello, World!".to_string())
    }
}

impl Exporter for Simple {
    type Chips = String;

    async fn scrape(&self) -> ExporterResult<Self::Chips> {
        self.do_scrape().await
    }
}
