#[derive(Debug)]
pub struct AlertApi<'a>(pub(crate) &'a crate::OpsgenieClient);
