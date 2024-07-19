use vise::{traits::GaugeValue, Gauge, LabeledFamily, Metrics};

#[derive(Debug)]
#[repr(u64)]
pub(crate) enum OnCallStatus {
    OnCall = 1,
    NotOnCall = 0,
}

#[derive(Debug, Metrics)]
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
}

#[vise::register]
pub(crate) static METRICS: vise::Global<OpsgenieMetrics> = vise::Global::new();
