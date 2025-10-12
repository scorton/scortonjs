
## Installation

```bash
python -m pip install -r requirements.txt
```

## Usage

The `scorton` script exposes a single `scan` sub‑command and a helper `help` command.

### Show help

```bash
./scorton help
./scorton -h          # global help
./scorton scan -h     # help for the scan command
```

### Call a security route

```bash
./scorton scan dns_enum https://example.com --api http://127.0.0.1:8000 --token YOUR_JWT
```


Supported tools:

```
cookie_scan   -> /cookie_scan
dir_scan      -> /dir_scan
dns_enum      -> /dns_enum
headers_check -> /headers_check
methods_scan  -> /methods_scan
port_scan     -> /port_scan
reverse_dns   -> /reverse_dns
score         -> /score
ssl_scan      -> /ssl_scan
url_analyze   -> /url_analyze
whois_scan    -> /whois_scan
xss_scan      -> /xss_scan
```

External services can also be used:

```
virustotal  -> https://www.virustotal.com/api/v3/domains/<domain>
shodan      -> https://api.shodan.io/shodan/host/<ip>
urlscan     -> https://urlscan.io/api/v1/scan/
crowdsec    -> https://api.crowdsec.net/v1/alerts
```

## Environment variables

Instead of passing `--api` and `--token` each time you can define:

```bash
export SCORTON_API_URL=http://127.0.0.1:8000
export SCORTON_TOKEN=YOUR_TOKEN
```

These values become the defaults for all commands.

API keys for the external services can be provided via command‑line flags or environment variables:

```
export VT_API_KEY=YOUR_VT_KEY
export SHODAN_API_KEY=YOUR_SHODAN_KEY
export URLSCAN_API_KEY=YOUR_URLSCAN_KEY
export CROWDSEC_API_KEY=YOUR_CROWDSEC_KEY
```

### Examples

```bash
./scorton scan virustotal https://example.com --vt-key YOUR_VT_KEY
./scorton scan shodan 8.8.8.8 --shodan-key YOUR_SHODAN_KEY
```

## Development

The codebase is small and self‑contained. Run a quick syntax check with:

```bash
python3 -m py_compile scorton.py main.py
```

