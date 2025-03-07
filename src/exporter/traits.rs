use std::future::Future;

use crate::exporter::ExporterResult;

// Exporter charges for gathering data from sensors with is scheduler.
pub trait Exporter {
    type Chips;

    fn scrape(&self) -> impl Future<Output = ExporterResult<Self::Chips>> + Send;
}
