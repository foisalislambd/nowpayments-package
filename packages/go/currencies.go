package nowpayments

import (
	"net/url"
	"strings"
)

// GetStatus checks if the API is up and available.
func (c *Client) GetStatus() (*ApiStatusResponse, error) {
	var result ApiStatusResponse
	if err := c.do("GET", "/v1/status", nil, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// GetCurrencies returns the list of available crypto currencies (e.g. btc, eth, usdt).
func (c *Client) GetCurrencies(fixedRate *bool) (map[string][]string, error) {
	params := url.Values{}
	if fixedRate != nil {
		params.Set("fixed_rate", boolStr(*fixedRate))
	}
	var result struct {
		Currencies []string `json:"currencies"`
	}
	path := "/v1/currencies"
	if len(params) > 0 {
		path += "?" + params.Encode()
	}
	if err := c.do("GET", path, nil, &result, ""); err != nil {
		return nil, err
	}
	return map[string][]string{"currencies": result.Currencies}, nil
}

// GetFullCurrencies returns full currency details (id, code, name, wallet_regex, network, etc.).
func (c *Client) GetFullCurrencies() (*struct {
	Currencies []FullCurrency `json:"currencies"`
}, error) {
	var result struct {
		Currencies []FullCurrency `json:"currencies"`
	}
	if err := c.do("GET", "/v1/full-currencies", nil, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// GetMerchantCoins returns merchant checked currencies (from coins settings).
func (c *Client) GetMerchantCoins(fixedRate *bool) (map[string][]string, error) {
	params := url.Values{}
	if fixedRate != nil {
		params.Set("fixed_rate", boolStr(*fixedRate))
	}
	var result struct {
		Currencies []string `json:"currencies"`
	}
	path := "/v1/merchant/coins"
	if len(params) > 0 {
		path += "?" + params.Encode()
	}
	if err := c.do("GET", path, nil, &result, ""); err != nil {
		return nil, err
	}
	return map[string][]string{"currencies": result.Currencies}, nil
}

// GetAuthToken returns a JWT token (required for payouts, custody, etc.). Token expires in 5 minutes.
func (c *Client) GetAuthToken(email, password string) (*AuthResponse, error) {
	body := map[string]string{"email": email, "password": password}
	var result AuthResponse
	if err := c.do("POST", "/v1/auth", body, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// GetCurrency returns single currency details (limits, etc.).
func (c *Client) GetCurrency(currency string) (interface{}, error) {
	code := strings.TrimSpace(currency)
	if code == "" {
		return nil, &NowPaymentsError{Message: "Currency code is required (e.g. \"btc\", \"eth\")"}
	}
	var result interface{}
	if err := c.do("GET", "/v1/currencies/"+url.PathEscape(code), nil, &result, ""); err != nil {
		return nil, err
	}
	return result, nil
}

func boolStr(b bool) string {
	if b {
		return "true"
	}
	return "false"
}
