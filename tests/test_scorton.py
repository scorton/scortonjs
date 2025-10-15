#!/usr/bin/env python3
"""
Test suite for ScortonJS - Production-ready security auditing tool.

This module contains comprehensive tests for all CLI functionality,
including API client, error handling, and command execution.
"""

import json
import os
import pytest  # pyright: ignore[reportMissingImports]
import sys
from unittest.mock import Mock, patch, MagicMock
from requests.exceptions import RequestException, Timeout, ConnectionError, HTTPError

# Add the current directory to the path so we can import scorton
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from scorton import (
    ScortonClient,
    ScortonError,
    APIError,
    ConfigurationError,
    validate_url,
    get_auth_config,
    run_scan,
    run_score,
    run_audit,
    run_config,
    ENDPOINTS,
    DEFAULT_CONFIG,
)


class TestScortonClient:
    """Test cases for ScortonClient class."""

    def test_client_initialization(self):
        """Test ScortonClient initialization."""
        client = ScortonClient("http://localhost:8000", "test-token")
        assert client.api_base == "http://localhost:8000"
        assert client.token == "test-token"
        assert client.timeout == 30

    def test_client_initialization_with_custom_params(self):
        """Test ScortonClient initialization with custom parameters."""
        client = ScortonClient("http://localhost:8000", "test-token", timeout=60, max_retries=5)
        assert client.timeout == 60

    @patch('scorton.requests.Session.post')
    def test_successful_api_call(self, mock_post):
        """Test successful API call."""
        # Mock successful response
        mock_response = Mock()
        mock_response.json.return_value = {"status": "success", "data": "test"}
        mock_response.raise_for_status.return_value = None
        mock_post.return_value = mock_response

        client = ScortonClient("http://localhost:8000", "test-token")
        result = client.call_api("test_endpoint", "example.com")

        assert result == {"status": "success", "data": "test"}
        mock_post.assert_called_once()

    @patch('scorton.requests.Session.post')
    def test_api_call_with_mode(self, mock_post):
        """Test API call with mode parameter."""
        mock_response = Mock()
        mock_response.json.return_value = {"status": "success"}
        mock_response.raise_for_status.return_value = None
        mock_post.return_value = mock_response

        client = ScortonClient("http://localhost:8000", "test-token")
        result = client.call_api("test_endpoint", "example.com", mode="deep")

        assert result == {"status": "success"}
        call_args = mock_post.call_args
        assert call_args[1]['json']['mode'] == "deep"

    @patch('scorton.requests.Session.post')
    def test_api_call_timeout_error(self, mock_post):
        """Test API call timeout error handling."""
        mock_post.side_effect = Timeout("Request timeout")

        client = ScortonClient("http://localhost:8000", "test-token")
        
        with pytest.raises(APIError) as exc_info:
            client.call_api("test_endpoint", "example.com")
        
        assert "Request timeout" in str(exc_info.value)

    @patch('scorton.requests.Session.post')
    def test_api_call_connection_error(self, mock_post):
        """Test API call connection error handling."""
        mock_post.side_effect = ConnectionError("Connection failed")

        client = ScortonClient("http://localhost:8000", "test-token")
        
        with pytest.raises(APIError) as exc_info:
            client.call_api("test_endpoint", "example.com")
        
        assert "Connection error" in str(exc_info.value)

    @patch('scorton.requests.Session.post')
    def test_api_call_http_error(self, mock_post):
        """Test API call HTTP error handling."""
        mock_response = Mock()
        mock_response.status_code = 404
        mock_response.text = "Not Found"
        mock_post.side_effect = HTTPError(response=mock_response)

        client = ScortonClient("http://localhost:8000", "test-token")
        
        with pytest.raises(APIError) as exc_info:
            client.call_api("test_endpoint", "example.com")
        
        assert "HTTP error 404" in str(exc_info.value)

    @patch('scorton.requests.Session.post')
    def test_api_call_invalid_json(self, mock_post):
        """Test API call with invalid JSON response."""
        mock_response = Mock()
        mock_response.json.side_effect = ValueError("Invalid JSON")
        mock_response.raise_for_status.return_value = None
        mock_post.return_value = mock_response

        client = ScortonClient("http://localhost:8000", "test-token")
        
        with pytest.raises(APIError) as exc_info:
            client.call_api("test_endpoint", "example.com")
        
        assert "Invalid JSON response" in str(exc_info.value)

    @patch('scorton.requests.Session.post')
    def test_api_call_list_response(self, mock_post):
        """Test API call with list response normalization."""
        mock_response = Mock()
        mock_response.json.return_value = ["item1", "item2"]
        mock_response.raise_for_status.return_value = None
        mock_post.return_value = mock_response

        client = ScortonClient("http://localhost:8000", "test-token")
        result = client.call_api("test_endpoint", "example.com")

        assert result == {"status": "ok", "data": ["item1", "item2"]}

    @patch('scorton.requests.Session.post')
    def test_api_call_unknown_response(self, mock_post):
        """Test API call with unknown response type."""
        mock_response = Mock()
        mock_response.json.return_value = "string response"
        mock_response.raise_for_status.return_value = None
        mock_post.return_value = mock_response

        client = ScortonClient("http://localhost:8000", "test-token")
        result = client.call_api("test_endpoint", "example.com")

        assert result == {"status": "unknown", "raw": "string response"}


class TestUtilityFunctions:
    """Test cases for utility functions."""

    def test_validate_url_valid(self):
        """Test URL validation with valid URLs."""
        assert validate_url("http://example.com") is True
        assert validate_url("https://example.com") is True
        assert validate_url("http://localhost:8000") is True
        assert validate_url("https://api.example.com/v1") is True

    def test_validate_url_invalid(self):
        """Test URL validation with invalid URLs."""
        assert validate_url("not-a-url") is False
        assert validate_url("ftp://example.com") is False
        assert validate_url("") is False
        assert validate_url("http://") is False

    @patch.dict(os.environ, {"SCORTON_TOKEN": "test-token", "SCORTON_API_URL": "http://test.com"})
    def test_get_auth_config_from_env(self):
        """Test getting auth config from environment variables."""
        args = Mock()
        args.token = None
        args.api = None

        api_base, token = get_auth_config(args)
        assert api_base == "http://test.com"
        assert token == "test-token"

    def test_get_auth_config_from_args(self):
        """Test getting auth config from command line arguments."""
        args = Mock()
        args.token = "arg-token"
        args.api = "http://arg.com"

        api_base, token = get_auth_config(args)
        assert api_base == "http://arg.com"
        assert token == "arg-token"

    def test_get_auth_config_missing_token(self):
        """Test getting auth config with missing token."""
        args = Mock()
        args.token = None
        args.api = None

        with patch.dict(os.environ, {}, clear=True):
            with pytest.raises(ConfigurationError) as exc_info:
                get_auth_config(args)
            assert "Authentication token is required" in str(exc_info.value)

    def test_get_auth_config_invalid_url(self):
        """Test getting auth config with invalid URL."""
        args = Mock()
        args.token = "test-token"
        args.api = "invalid-url"

        with pytest.raises(ConfigurationError) as exc_info:
            get_auth_config(args)
        assert "Invalid API URL" in str(exc_info.value)


class TestCommandFunctions:
    """Test cases for command functions."""

    @patch('scorton.ScortonClient')
    @patch('scorton.get_auth_config')
    def test_run_scan_success(self, mock_get_auth, mock_client_class):
        """Test successful scan command execution."""
        mock_get_auth.return_value = ("http://localhost:8000", "test-token")
        mock_client = Mock()
        mock_client.call_api.return_value = {"status": "success"}
        mock_client_class.return_value = mock_client

        args = Mock()
        args.tool = "dns_enum"
        args.target = "example.com"
        args.token = None
        args.api = None
        args.timeout = 30

        with patch('builtins.print') as mock_print:
            run_scan(args)
            mock_print.assert_called_once()
            printed_data = json.loads(mock_print.call_args[0][0])
            assert printed_data == {"status": "success"}

    def test_run_scan_unknown_tool(self):
        """Test scan command with unknown tool."""
        args = Mock()
        args.tool = "unknown_tool"
        args.target = "example.com"

        with pytest.raises(ScortonError) as exc_info:
            run_scan(args)
        assert "Unknown tool 'unknown_tool'" in str(exc_info.value)

    @patch('scorton.ScortonClient')
    @patch('scorton.get_auth_config')
    def test_run_score_success(self, mock_get_auth, mock_client_class):
        """Test successful score command execution."""
        mock_get_auth.return_value = ("http://localhost:8000", "test-token")
        mock_client = Mock()
        mock_client.call_api.return_value = {"score": 85.5}
        mock_client_class.return_value = mock_client

        args = Mock()
        args.target = "example.com"
        args.token = None
        args.api = None
        args.timeout = 30

        with patch('builtins.print') as mock_print:
            run_score(args)
            mock_print.assert_called_once()

    @patch('scorton.ScortonClient')
    @patch('scorton.get_auth_config')
    def test_run_audit_success(self, mock_get_auth, mock_client_class):
        """Test successful audit command execution."""
        mock_get_auth.return_value = ("http://localhost:8000", "test-token")
        mock_client = Mock()
        mock_client.call_api.return_value = {"audit": "completed"}
        mock_client_class.return_value = mock_client

        args = Mock()
        args.target = "example.com"
        args.token = None
        args.api = None
        args.timeout = 30

        with patch('builtins.print') as mock_print:
            run_audit(args)
            mock_print.assert_called_once()

    def test_run_config_show(self):
        """Test config command showing current configuration."""
        args = Mock()
        args.set = None

        with patch.dict(os.environ, {"SCORTON_TOKEN": "test-token"}, clear=True):
            with patch('builtins.print') as mock_print:
                run_config(args)
                assert mock_print.call_count >= 2  # At least config header and values

    def test_run_config_set(self):
        """Test config command with --set option."""
        args = Mock()
        args.set = "SCORTON_API_URL=http://new-api.com"

        with patch('builtins.print') as mock_print:
            run_config(args)
            mock_print.assert_called()
            # Check that export command was printed
            calls = [call[0][0] for call in mock_print.call_args_list]
            assert any("export SCORTON_API_URL=http://new-api.com" in call for call in calls)

    def test_run_config_set_invalid_format(self):
        """Test config command with invalid --set format."""
        args = Mock()
        args.set = "invalid-format"

        with pytest.raises(ScortonError) as exc_info:
            run_config(args)
        assert "Invalid format for --set" in str(exc_info.value)


class TestConstants:
    """Test cases for constants and configuration."""

    def test_endpoints_contains_expected_tools(self):
        """Test that ENDPOINTS contains expected security tools."""
        expected_tools = [
            "cookie_scan", "dir_scan", "dns_enum", "headers_check",
            "methods_scan", "port_scan", "reverse_dns", "ssl_scan",
            "url_analyze", "whois_scan", "xss_scan"
        ]
        
        for tool in expected_tools:
            assert tool in ENDPOINTS
            assert ENDPOINTS[tool] == tool

    def test_default_config_values(self):
        """Test DEFAULT_CONFIG contains expected values."""
        assert DEFAULT_CONFIG["api_url"] == "http://localhost:8000"
        assert DEFAULT_CONFIG["timeout"] == 30
        assert DEFAULT_CONFIG["max_retries"] == 3
        assert DEFAULT_CONFIG["retry_backoff"] == 0.3


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
