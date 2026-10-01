package nowpayments

import (
	"fmt"
	"net/url"
	"strings"
)

// GetSubscriptions returns list of recurring payments.
func (c *Client) GetSubscriptions(params *GetSubscriptionsParams) (*SubscriptionsResponse, error) {
	q := url.Values{}
	if params != nil {
		if params.Status != "" {
			q.Set("status", params.Status)
		}
		if params.SubscriptionPlanID != nil {
			q.Set("subscription_plan_id", fmt.Sprintf("%v", params.SubscriptionPlanID))
		}
		if params.IsActive != nil {
			q.Set("is_active", boolStr(*params.IsActive))
		}
		if params.Limit > 0 {
			q.Set("limit", fmt.Sprintf("%d", params.Limit))
		}
		if params.Offset > 0 {
			q.Set("offset", fmt.Sprintf("%d", params.Offset))
		}
	}
	path := "/v1/subscriptions"
	if len(q) > 0 {
		path += "?" + q.Encode()
	}
	var result SubscriptionsResponse
	if err := c.get(path, nil, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// SubscriptionsResponse from get subscriptions.
type SubscriptionsResponse struct {
	Count  int               `json:"count"`
	Result []RecurringPayment `json:"result"`
}

// GetSubscription returns single recurring payment.
func (c *Client) GetSubscription(id string) (*SubscriptionResponse, error) {
	var result SubscriptionResponse
	if err := c.get("/v1/subscriptions/"+url.PathEscape(id), nil, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// SubscriptionResponse from get subscription.
type SubscriptionResponse struct {
	Result RecurringPayment `json:"result"`
}

// DeleteSubscription cancels recurring payment. JWT optional per API docs.
func (c *Client) DeleteSubscription(id, jwtToken string) (*DeleteSubscriptionResponse, error) {
	var result DeleteSubscriptionResponse
	if err := c.delete("/v1/subscriptions/"+url.PathEscape(id), &result, jwtToken); err != nil {
		return nil, err
	}
	return &result, nil
}

// DeleteSubscriptionResponse from delete subscription.
type DeleteSubscriptionResponse struct {
	Result string `json:"result"`
}

// GetSubscriptionPlans returns list of subscription plans.
func (c *Client) GetSubscriptionPlans(params *GetSubscriptionPlansParams) (*SubscriptionPlansResponse, error) {
	q := url.Values{}
	if params != nil {
		if params.Limit > 0 {
			q.Set("limit", fmt.Sprintf("%d", params.Limit))
		}
		if params.Offset > 0 {
			q.Set("offset", fmt.Sprintf("%d", params.Offset))
		}
	}
	path := "/v1/subscriptions/plans"
	if len(q) > 0 {
		path += "?" + q.Encode()
	}
	var result SubscriptionPlansResponse
	if err := c.get(path, nil, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// SubscriptionPlansResponse from get subscription plans.
type SubscriptionPlansResponse struct {
	Count  int                `json:"count"`
	Result []SubscriptionPlan `json:"result"`
}

// GetSubscriptionPlan returns single subscription plan.
func (c *Client) GetSubscriptionPlan(id string) (*SubscriptionPlanResponse, error) {
	var result SubscriptionPlanResponse
	if err := c.get("/v1/subscriptions/plans/"+url.PathEscape(id), nil, &result, ""); err != nil {
		return nil, err
	}
	return &result, nil
}

// SubscriptionPlanResponse from get subscription plan.
type SubscriptionPlanResponse struct {
	Result SubscriptionPlan `json:"result"`
}

// UpdateSubscriptionPlan updates subscription plan.
func (c *Client) UpdateSubscriptionPlan(id string, updates map[string]interface{}) (interface{}, error) {
	var result interface{}
	if err := c.patch("/v1/subscriptions/plans/"+url.PathEscape(id), updates, &result, ""); err != nil {
		return nil, err
	}
	return result, nil
}

// CreateSubscriptionParams for creating subscription.
type CreateSubscriptionParams struct {
	SubscriptionPlanID interface{} `json:"subscription_plan_id"`
	SubPartnerID      interface{} `json:"sub_partner_id,omitempty"`
	Email             string     `json:"email,omitempty"`
}

// CreateSubscription creates subscription. Requires JWT.
func (c *Client) CreateSubscription(params CreateSubscriptionParams, jwtToken string) (*CreateSubscriptionResponse, error) {
	if strings.TrimSpace(jwtToken) == "" {
		return nil, &NowPaymentsError{Message: "JWT token is required for CreateSubscription. Call GetAuthToken first."}
	}
	var result CreateSubscriptionResponse
	if err := c.post("/v1/subscriptions", params, &result, jwtToken); err != nil {
		return nil, err
	}
	return &result, nil
}

// CreateSubscriptionResponse from create subscription.
type CreateSubscriptionResponse struct {
	Result RecurringPayment `json:"result"`
}
