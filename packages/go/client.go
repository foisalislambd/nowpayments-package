package nowpayments

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
	"time"
)

const (
	// Version is the SDK release version.
	Version = "1.0.4"

	ProductionURL = "https://api.nowpayments.io"
	SandboxURL    = "https://api-sandbox.nowpayments.io"
)

// Config holds the client configuration.
type Config struct {
	// APIKey is your NOWPayments API key (required).
	APIKey string
	// Sandbox uses the sandbox API (api-sandbox.nowpayments.io).
	Sandbox bool
	// BaseURL overrides the default URL (takes precedence over Sandbox).
	BaseURL string
	// Timeout is the HTTP request timeout (default 30s).
	Timeout time.Duration
	// IPNSecret is used for verifying webhook callbacks.
	IPNSecret string
}

// NowPaymentsError represents an API or network error.
type NowPaymentsError struct {
	Message    string
	StatusCode int
	Code       string
	Response   interface{}
}

func (e *NowPaymentsError) Error() string {
	parts := []string{e.Message}
	if e.StatusCode > 0 {
		parts = append(parts, fmt.Sprintf("(status: %d)", e.StatusCode))
	}
	if e.Code != "" {
		parts = append(parts, fmt.Sprintf("[%s]", e.Code))
	}
	return strings.Join(parts, " ")
}

// Client is the NOWPayments API client.
type Client struct {
	httpClient *http.Client
	baseURL    string
	apiKey     string
	config     Config
}

// New creates a new NOWPayments client.
func New(config Config) (*Client, error) {
	if strings.TrimSpace(config.APIKey) == "" {
		return nil, fmt.Errorf("NOWPayments API key is required. Get yours at https://account.nowpayments.io")
	}

	baseURL := config.BaseURL
	if baseURL == "" {
		if config.Sandbox {
			baseURL = SandboxURL
		} else {
			baseURL = ProductionURL
		}
	}
	baseURL = strings.TrimSuffix(baseURL, "/")

	timeout := config.Timeout
	if timeout == 0 {
		timeout = 30 * time.Second
	}

	return &Client{
		httpClient: &http.Client{Timeout: timeout},
		baseURL:    baseURL,
		apiKey:     config.APIKey,
		config:     config,
	}, nil
}

// do performs an HTTP request and decodes the response.
func (c *Client) do(method, path string, body interface{}, result interface{}, jwtToken string) error {
	var bodyReader io.Reader
	if body != nil {
		jsonBody, err := json.Marshal(body)
		if err != nil {
			return err
		}
		bodyReader = bytes.NewReader(jsonBody)
	}

	req, err := http.NewRequest(method, c.baseURL+path, bodyReader)
	if err != nil {
		return err
	}

	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("x-api-key", strings.TrimSpace(c.apiKey))
	if jwtToken != "" {
		req.Header.Set("Authorization", "Bearer "+strings.TrimSpace(jwtToken))
	}

	resp, err := c.httpClient.Do(req)
	if err != nil {
		if strings.Contains(err.Error(), "timeout") || strings.Contains(err.Error(), "deadline") {
			return &NowPaymentsError{Message: "Request timed out. Check your connection or try again."}
		}
		return &NowPaymentsError{Message: err.Error(), Code: "NETWORK_ERROR"}
	}
	defer resp.Body.Close()

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	if resp.StatusCode >= 400 {
		var errData struct {
			Message string `json:"message"`
			Msg     string `json:"msg"`
			Error   string `json:"error"`
			Code    string `json:"code"`
		}
		_ = json.Unmarshal(respBody, &errData)
		msg := errData.Message
		if msg == "" {
			msg = errData.Msg
		}
		if msg == "" {
			msg = errData.Error
		}
		if msg == "" {
			msg = string(respBody)
		}
		if msg == "" {
			msg = "Request failed"
		}
		return &NowPaymentsError{
			Message:    msg,
			StatusCode: resp.StatusCode,
			Code:       errData.Code,
			Response:   respBody,
		}
	}

	if result != nil && len(respBody) > 0 {
		if err := json.Unmarshal(respBody, result); err != nil {
			// Non-JSON success bodies (e.g. verify payout returns plain "OK")
			if ptr, ok := result.(*interface{}); ok {
				*ptr = strings.TrimSpace(string(respBody))
				return nil
			}
			if ptr, ok := result.(*string); ok {
				*ptr = strings.TrimSpace(string(respBody))
				return nil
			}
			return &NowPaymentsError{
				Message:    fmt.Sprintf("Failed to parse response: %v", err),
				StatusCode: resp.StatusCode,
				Response:   string(respBody),
			}
		}
	}
	return nil
}

// get performs a GET request. Pass nil for params if no query params.
func (c *Client) get(path string, params url.Values, result interface{}, jwtToken string) error {
	if len(params) > 0 {
		path += "?" + params.Encode()
	}
	return c.do(http.MethodGet, path, nil, result, jwtToken)
}

// post performs a POST request.
func (c *Client) post(path string, body interface{}, result interface{}, jwtToken string) error {
	return c.do(http.MethodPost, path, body, result, jwtToken)
}

// patch performs a PATCH request.
func (c *Client) patch(path string, body interface{}, result interface{}, jwtToken string) error {
	return c.do(http.MethodPatch, path, body, result, jwtToken)
}

// delete performs a DELETE request.
func (c *Client) delete(path string, result interface{}, jwtToken string) error {
	return c.do(http.MethodDelete, path, nil, result, jwtToken)
}
