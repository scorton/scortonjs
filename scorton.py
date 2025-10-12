#!/usr/bin/env python3
"""Scorton CLI entry point."""

import argparse
import json
import os
import sys
from typing import Any, Callable, Dict
from urllib.parse import urlparse

import requests


# Mapping of CLI tool names to API endpoints.
ENDPOINTS = {
    "cookie_scan": "cookie_scan",
    "dir_scan": "dir_scan",
    "dns_enum": "dns_enum",
    "headers_check": "headers_check",
    "methods_scan": "methods_scan",
    "port_scan": "port_scan",
    "reverse_dns": "reverse_dns",
    "score": "score",
    "ssl_scan": "ssl_scan",
    "url_analyze": "url_analyze",
    "whois_scan": "whois_scan",
    "xss_scan": "xss_scan",
}

# External API dispatch table. Each tool maps to a function that will be
# invoked with (target, api_key).
EXTERNAL_APIS: Dict[str, Callable[[str, str], None]] = {}


def post_to_api(api_base: str, endpoint: str, target: str, token: str) -> None:
    url = f"{api_base.rstrip('/')}/{endpoint}"
    headers = {"Content-Type": "application/json", "AUTH": token}
    payload = {"target": target}
    try:
        resp = requests.post(url, headers=headers, json=payload, timeout=30)
    except requests.RequestException as exc:  # network error
        print(f"Failed to call {url}: {exc}")
        sys.exit(1)

    if not resp.ok:
        body_preview = resp.text[:400]
        print(
            f"Failed to call {url}: {resp.status_code} {resp.reason} :: {body_preview}"
        )
        sys.exit(1)

    try:
        data: Any = resp.json()
    except ValueError:
        print(resp.text)
        sys.exit(1)

    if isinstance(data, dict):
        output = data
    elif isinstance(data, list):
        output = {"status": "ok", "data": data}
    else:
        output = {"status": "unknown", "raw": data}

    print(json.dumps(output, indent=2, ensure_ascii=False))


def handle_response(resp: requests.Response, url: str) -> None:
    if not resp.ok:
        body_preview = resp.text[:400]
        print(
            f"Failed to call {url}: {resp.status_code} {resp.reason} :: {body_preview}"
        )
        sys.exit(1)

    try:
        data: Any = resp.json()
    except ValueError:
        print(resp.text)
        sys.exit(1)

    if isinstance(data, dict):
        output = data
    elif isinstance(data, list):
        output = {"status": "ok", "data": data}
    else:
        output = {"status": "unknown", "raw": data}

    print(json.dumps(output, indent=2, ensure_ascii=False))


def call_virustotal(target: str, api_key: str) -> None:
    if not api_key:
        print("VirusTotal API key required. Use --vt-key or set VT_API_KEY.")
        sys.exit(1)
    domain = urlparse(target).netloc or target
    url = f"https://www.virustotal.com/api/v3/domains/{domain}"
    headers = {"x-apikey": api_key}
    try:
        resp = requests.get(url, headers=headers, timeout=30)
    except requests.RequestException as exc:
        print(f"Failed to call {url}: {exc}")
        sys.exit(1)
    handle_response(resp, url)


def call_shodan(target: str, api_key: str) -> None:
    if not api_key:
        print("Shodan API key required. Use --shodan-key or set SHODAN_API_KEY.")
        sys.exit(1)
    url = f"https://api.shodan.io/shodan/host/{target}?key={api_key}"
    try:
        resp = requests.get(url, timeout=30)
    except requests.RequestException as exc:
        print(f"Failed to call {url}: {exc}")
        sys.exit(1)
    handle_response(resp, url)


def call_urlscan(target: str, api_key: str) -> None:
    if not api_key:
        print("urlscan.io API key required. Use --urlscan-key or set URLSCAN_API_KEY.")
        sys.exit(1)
    url = "https://urlscan.io/api/v1/scan/"
    headers = {"API-Key": api_key, "Content-Type": "application/json"}
    payload = {"url": target}
    try:
        resp = requests.post(url, headers=headers, json=payload, timeout=30)
    except requests.RequestException as exc:
        print(f"Failed to call {url}: {exc}")
        sys.exit(1)
    handle_response(resp, url)


def call_crowdsec(target: str, api_key: str) -> None:
    if not api_key:
        print("CrowdSec API key required. Use --crowdsec-key or set CROWDSEC_API_KEY.")
        sys.exit(1)
    url = f"https://api.crowdsec.net/v1/alerts?ip={target}"
    headers = {"X-Api-Key": api_key}
    try:
        resp = requests.get(url, headers=headers, timeout=30)
    except requests.RequestException as exc:
        print(f"Failed to call {url}: {exc}")
        sys.exit(1)
    handle_response(resp, url)


# Populate the external API dispatch table.
EXTERNAL_APIS.update(
    {
        "virustotal": call_virustotal,
        "shodan": call_shodan,
        "urlscan": call_urlscan,
        "crowdsec": call_crowdsec,
    }
)


def validate_token(token: str) -> None:
    if token.count(".") != 2:
        print("Provided token does not look like a JWT (expected three dot-separated parts)")
        sys.exit(1)


def run_scan(args: argparse.Namespace) -> None:
    # First check if the tool is handled by the local API
    if args.tool in ENDPOINTS:
        token = args.token or os.environ.get("SCORTON_TOKEN")
        if not token:
            print("An AUTH token is required. Provide --token or set SCORTON_TOKEN.")
            sys.exit(1)
        validate_token(token)
        api_base = args.api or os.environ.get("SCORTON_API_URL", "http://localhost:8000")
        post_to_api(api_base, ENDPOINTS[args.tool], args.target, token)
        return

    # External APIs
    if args.tool in EXTERNAL_APIS:
        if args.tool == "virustotal":
            key = args.vt_key or os.environ.get("VT_API_KEY")
        elif args.tool == "shodan":
            key = args.shodan_key or os.environ.get("SHODAN_API_KEY")
        elif args.tool == "urlscan":
            key = args.urlscan_key or os.environ.get("URLSCAN_API_KEY")
        else:  # crowdsec
            key = args.crowdsec_key or os.environ.get("CROWDSEC_API_KEY")

        EXTERNAL_APIS[args.tool](args.target, key)
        return

    print(
        "Unknown tool '{}'. Available: {}".format(
            args.tool, ", ".join(sorted(list(ENDPOINTS) + list(EXTERNAL_APIS)))
        )
    )
    sys.exit(1)


def main(argv=None) -> None:
    parser = argparse.ArgumentParser(prog="scorton", description="Scorton security auditing CLI")
    subparsers = parser.add_subparsers(dest="command")

    scan_parser = subparsers.add_parser("scan", help="Call a security tool for a target")
    scan_parser.add_argument("tool", help="Tool to invoke, e.g. dns_enum")
    scan_parser.add_argument("target", help="Domain or URL to analyze")
    scan_parser.add_argument(
        "--api",
        default=os.environ.get("SCORTON_API_URL"),
        help="Base URL of the API",
    )
    scan_parser.add_argument(
        "--token",
        default=os.environ.get("SCORTON_TOKEN"),
        help="JWT for AUTH header",
    )
    scan_parser.add_argument("--vt-key", help="API key for VirusTotal")
    scan_parser.add_argument("--shodan-key", help="API key for Shodan")
    scan_parser.add_argument("--urlscan-key", help="API key for urlscan.io")
    scan_parser.add_argument("--crowdsec-key", help="API key for CrowdSec")
    scan_parser.set_defaults(func=run_scan)

    subparsers.add_parser("help", help="Show this help message")

    args = parser.parse_args(argv)
    if args.command in (None, "help"):
        parser.print_help()
        return
    args.func(args)


if __name__ == "__main__":
    main()

