package nowpayments

// CreateInvoice creates an invoice (redirect flow).
func (c *Client) CreateInvoice(params CreateInvoiceParams) (*InvoiceResponse, error) {
	var result InvoiceResponse
	if err := c.do("POST", "/v1/invoice", params, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// CreateInvoicePayment creates payment for existing invoice.
func (c *Client) CreateInvoicePayment(params CreateInvoicePaymentParams) (*Payment, error) {
	var result Payment
	if err := c.do("POST", "/v1/invoice-payment", params, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}
