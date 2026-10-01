package nowpayments

import (
	"fmt"
	"net/url"
)

// appendQueryParam encodes scalar or slice values for NOWPayments list filters (id, status).
func appendQueryParam(q url.Values, key string, val interface{}) {
	if val == nil {
		return
	}
	switch v := val.(type) {
	case []int:
		for _, n := range v {
			q.Add(key, fmt.Sprintf("%d", n))
		}
	case []int64:
		for _, n := range v {
			q.Add(key, fmt.Sprintf("%d", n))
		}
	case []string:
		for _, s := range v {
			q.Add(key, s)
		}
	case []interface{}:
		for _, item := range v {
			appendQueryParam(q, key, item)
		}
	case int:
		q.Add(key, fmt.Sprintf("%d", v))
	case int64:
		q.Add(key, fmt.Sprintf("%d", v))
	case float64:
		q.Add(key, fmt.Sprintf("%v", v))
	case string:
		q.Add(key, v)
	default:
		q.Add(key, fmt.Sprintf("%v", v))
	}
}
