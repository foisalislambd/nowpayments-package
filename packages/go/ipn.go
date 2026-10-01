package nowpayments

import (
	"bytes"
	"crypto/hmac"
	"crypto/sha512"
	"encoding/hex"
	"encoding/json"
	"regexp"
	"sort"
	"strings"
)

var ipnSigHexStrip = regexp.MustCompile(`[^a-fA-F0-9]`)

// VerifyIPNSignature verifies the IPN callback signature from NOWPayments.
// Prefer passing the raw HTTP body string. Parsed maps can change number/string
// types and break verification. Signature is from x-nowpayments-sig header.
func VerifyIPNSignature(payload interface{}, signature, ipnSecret string) bool {
	if strings.TrimSpace(signature) == "" || strings.TrimSpace(ipnSecret) == "" {
		return false
	}
	var obj map[string]interface{}
	switch v := payload.(type) {
	case string:
		if err := json.Unmarshal([]byte(v), &obj); err != nil {
			return false
		}
	case map[string]interface{}:
		obj = v
	default:
		return false
	}
	jsonStr := jsonEncodeSorted(obj)
	mac := hmac.New(sha512.New, []byte(strings.TrimSpace(ipnSecret)))
	mac.Write([]byte(jsonStr))
	computed := hex.EncodeToString(mac.Sum(nil))
	sigHex := strings.ToLower(strings.TrimSpace(signature))
	if i := strings.LastIndex(sigHex, "="); i >= 0 {
		sigHex = sigHex[i+1:]
	}
	sigHex = strings.ToLower(ipnSigHexStrip.ReplaceAllString(sigHex, ""))
	sigBytes, err := hex.DecodeString(sigHex)
	if err != nil {
		return false
	}
	computedBytes, err := hex.DecodeString(computed)
	if err != nil {
		return false
	}
	if len(sigBytes) != len(computedBytes) {
		return false
	}
	return hmac.Equal(sigBytes, computedBytes)
}

// CreateIPNSignature creates an IPN signature for testing (e.g., mocking callbacks).
func CreateIPNSignature(payload map[string]interface{}, ipnSecret string) string {
	jsonStr := jsonEncodeSorted(payload)
	mac := hmac.New(sha512.New, []byte(strings.TrimSpace(ipnSecret)))
	mac.Write([]byte(jsonStr))
	return hex.EncodeToString(mac.Sum(nil))
}

// VerifyIPN verifies IPN using the client's configured IPNSecret.
func (c *Client) VerifyIPN(payload interface{}, signature string) (bool, error) {
	if c.config.IPNSecret == "" {
		return false, &NowPaymentsError{Message: "IPN secret not configured. Pass IPNSecret in Config or use VerifyIPNSignature with explicit secret."}
	}
	return VerifyIPNSignature(payload, signature, c.config.IPNSecret), nil
}

// sortObject recursively sorts map keys (matches NOWPayments IPN spec).
func sortObject(obj map[string]interface{}) map[string]interface{} {
	result := make(map[string]interface{})
	keys := make([]string, 0, len(obj))
	for k := range obj {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, k := range keys {
		val := obj[k]
		if val == nil {
			result[k] = val
			continue
		}
		if m, ok := val.(map[string]interface{}); ok {
			result[k] = sortObject(m)
		} else {
			result[k] = val
		}
	}
	return result
}

func jsonEncodeSorted(obj map[string]interface{}) string {
	sorted := sortObject(obj)
	var buf bytes.Buffer
	enc := json.NewEncoder(&buf)
	enc.SetEscapeHTML(false)
	_ = enc.Encode(sorted)
	s := buf.String()
	if len(s) > 0 && s[len(s)-1] == '\n' {
		s = s[:len(s)-1]
	}
	return s
}
