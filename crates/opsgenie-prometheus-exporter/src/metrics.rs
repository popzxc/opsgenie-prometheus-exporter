use std::time::Duration;

use vise::{Buckets, Gauge, Histogram, LabeledFamily, Metrics};

#[derive(Debug)]
#[repr(u64)]
pub(crate) enum OnCallStatus {
    OnCall = 1,
    NotOnCall = 0,
}

const MINUTE: f64 = 60.0;
const WEEK: f64 = 60.0 * 60.0 * 24.0 * 7.0;

#[derive(Debug, Metrics)]
#[metrics(prefix = "opsgenie")]
pub(crate) struct OpsgenieMetrics {
    /// Will export all team members on-call status.
    /// Value is `1` when the person is on-call, and `0` otherwise.
    #[metrics(labels = ["team", "schedule", "on_call"])]
    pub on_call: LabeledFamily<(String, String, String), Gauge<u64>, 3>,
    /// Number of alerts for each team.
    #[metrics(labels = ["team", "priority"])]
    pub alerts: LabeledFamily<(String, String), Gauge<u64>, 2>,
    /// Number of escalated alerts.
    #[metrics(labels = ["team", "priority"])]
    pub escalated_alerts: LabeledFamily<(String, String), Gauge<u64>, 2>,
    /// Alert live duration in seconds.
    #[metrics(buckets = Buckets::exponential(MINUTE..=WEEK, 4.0), labels = ["team", "priority"])]
    pub alert_duration: LabeledFamily<(String, String), Histogram<Duration>, 2>,
}

#[vise::register]
pub(crate) static METRICS: vise::Global<OpsgenieMetrics> = vise::Global::new();
