"""
IPN (Instant Payment Notification) verification utilities.
Matches official docs: sort keys recursively, then HMAC-SHA512.
@see https://nowpayments.io/help/payments/api
"""

import hmac
import json
import hashlib
import re
from typing import Any


def _sort_object(obj: dict[str, Any]) -> dict[str, Any]:
    """Recursively sort object keys (matches NOWPayments IPN spec)."""
    result: dict[str, Any] = {}
    for key in sorted(obj.keys()):
        val = obj[key]
        if val is not None and isinstance(val, dict):
            result[key] = _sort_object(val)
        else:
            result[key] = val
    return result


def verify_ipn_signature(
    payload: str | dict[str, Any],
    signature: str,
    ipn_secret: str,
) -> bool:
    """
    Verify IPN callback signature from NOWPayments.
    Safe to call – handles invalid input gracefully.

    Prefer the raw HTTP body string when possible. Framework-parsed dicts can
    coerce number/string types and break verification.

    Args:
        payload: Raw request body (string recommended) or parsed dict
        signature: Value from x-nowpayments-sig header
        ipn_secret: Your IPN Secret from Dashboard → Store Settings

    Returns:
        True if signature is valid, False otherwise
    """
    if not signature or not signature.strip() or not ipn_secret or not ipn_secret.strip():
        return False

    try:
        if isinstance(payload, str):
            obj = json.loads(payload)
        elif isinstance(payload, dict):
            obj = payload
        else:
            return False

        sorted_obj = _sort_object(obj)
        # Official Python docs: separators=(',', ':') — default ensure_ascii=True
        json_string = json.dumps(sorted_obj, separators=(",", ":"))

        computed_sig = hmac.new(
            ipn_secret.strip().encode("utf-8"),
            json_string.encode("utf-8"),
            hashlib.sha512,
        ).hexdigest()

        sig_norm = signature.strip().lower()
        if "=" in sig_norm:
            sig_norm = sig_norm.split("=", 1)[-1]
        sig_norm = re.sub(r"[^a-f0-9]", "", sig_norm)
        computed_norm = computed_sig.lower()
        if len(sig_norm) != len(computed_norm) or not sig_norm:
            return False
        return hmac.compare_digest(sig_norm, computed_norm)
    except (json.JSONDecodeError, TypeError, ValueError):
        return False


def create_ipn_signature(payload: dict[str, Any], ipn_secret: str) -> str:
    """Create IPN signature for testing (e.g., mocking callbacks)."""
    json_string = json.dumps(_sort_object(payload), separators=(",", ":"))
    return hmac.new(
        ipn_secret.strip().encode("utf-8"),
        json_string.encode("utf-8"),
        hashlib.sha512,
    ).hexdigest()
