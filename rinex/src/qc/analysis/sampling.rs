use hifitime::Unit;

use super::QcOpts;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QcSamplingAnalysis {
    /// First [`Epoch`] identified in time
    pub first_epoch: Option<Epoch>,
    /// Last [`Epoch`] identified in time
    pub last_epoch: Option<Epoch>,
    /// Time span of this RINEX context
    pub duration: Option<Duration>,
    /// File [`Header`] sample rate
    pub sample_rate: Option<Duration>,
    /// Dominant sample rate
    pub dominant_sample_rate: Option<Duration>,
    /// Unusual data gaps
    pub gaps: Vec<(Epoch, Duration)>,
    /// Epoch anomalies such as
    /// possible receiver loss of lock, bad conditions..
    #[cfg(feature = "obs")]
    #[cfg_attr(docrs, doc(cfg(feature = "obs")))]
    pub anomalies: Vec<(Epoch, EpochFlag)>,
}

impl QcSamplingAnalysis {
    pub fn new(rnx: &Rinex, opts: &QcOpts) -> Self {
        Self {
            first_epoch: rnx.first_epoch(),
            last_epoch: rnx.last_epoch(),
            duration: rnx.duration(),
            sample_rate: rnx.sample_rate(),
            dominant_sample_rate: rnx.dominant_sample_rate(),
            gaps: rnx.data_gaps(opts.gap_tolerance).collect(),
            #[cfg(feature = "obs")]
            anomalies: rnx.epoch_anomalies().collect(),
        }
    }
}

use crate::qc::HtmlReport;
use horrorshow::RenderBox;

impl HtmlReport for QcSamplingAnalysis {
    fn to_html(&self) -> String {
        todo!()
    }
    fn to_inline_html(&self) -> Box<dyn RenderBox + '_> {
        box_html! {
            tr {
                th {
                    : "Start"
                }
                th {
                    : "End"
                }
                th {
                    : "Span"
                }
            }
            tr {
                @ if let Some(epoch) = self.first_epoch {
                    td {
                        : epoch.to_string()
                    }
                } else {
                    td {
                        : "Unknown"
                    }
                }
                @ if let Some(epoch) = self.last_epoch {
                    td {
                        : epoch.to_string()
                    }
                } else {
                    td {
                        : "Unknown"
                    }
                }
                @ if let Some(duration) = self.duration {
                    td {
                        : duration.to_string()
                    }
                } else {
                    td {
                        : "Unknown"
                    }
                }
            }
            tr {
                th {
                    : "Sample rate"
                }
                @ if let Some(rate) = self.sample_rate {
                    td {
                        : format!("{} ({:.3} Hz)", rate, 1.0 / rate.to_unit(Unit::Second))
                    }
                } else {
                    td {
                        : "Unknown"
                    }
                }
            }
            div(class="table-container") {
                table(class="table is-bordered") {
                    thead {
                        th {
                            : "Gap analysis"
                        }
                    }
                    tbody {
                        @ for (_epoch, _dt) in &self.gaps {

                        }
                    }
                }
            }//table: gap analysis
        }
    }
}
