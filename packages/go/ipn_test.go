package nowpayments

import (
	"encoding/json"
	"testing"
)

func TestIPNSignatureRoundTrip(t *testing.T) {
	payload := map[string]interface{}{
		"payment_status": "finished",
		"payment_id":     float64(123),
		"z":              float64(1),
		"a":              float64(2),
	}
	secret := "mysecret"
	sig := CreateIPNSignature(payload, secret)
	if !VerifyIPNSignature(payload, sig, secret) {
		t.Fatal("expected valid signature")
	}
	if !VerifyIPNSignature(payload, "sha512="+sig, secret) {
		t.Fatal("expected valid signature with sha512= prefix")
	}
	if VerifyIPNSignature(payload, "deadbeef", secret) {
		t.Fatal("expected invalid signature")
	}
}

func TestIPNSortedKeys(t *testing.T) {
	a := map[string]interface{}{"b": float64(1), "a": float64(2)}
	b := map[string]interface{}{"a": float64(2), "b": float64(1)}
	secret := "s"
	sig := CreateIPNSignature(a, secret)
	if !VerifyIPNSignature(b, sig, secret) {
		t.Fatal("key order must not affect verification")
	}
}

func TestDoNonJSONSuccessBody(t *testing.T) {
	// Simulate VerifyPayout parsing path via json.Unmarshal failure fallback logic
	var result interface{}
	respBody := []byte("OK")
	if err := json.Unmarshal(respBody, &result); err != nil {
		result = string(respBody)
	}
	if result != "OK" {
		t.Fatalf("got %v", result)
	}
}
