# OpsGenie Prometheus exporter

This binary scans the Opsgenie API and exports the information discovered to Prometheus.

## Configuration

Configuration can be loaded as follows:

- By default, an `./.env` file will be attempted to be read. The path can be overridden with the
  `--env-file` CLI argument.
- If there is no `.env` file at the destination, the configuration will be loaded from the
  environment variables.

Right now, the following configuration options are supported:

```
OPSGENIE_BASE_URL=https://api.eu.opsgenie.com # Check the Opsgenie API docs to find the right URL for you.
OPSGENIE_API_KEY=<your key> # API key. Can be created in the Opsgenie settings.
PROMETHEUS_PORT=8432 # Prometheus exporter will run on this port
LOG_FORMAT=plain # Can be `json`
```
