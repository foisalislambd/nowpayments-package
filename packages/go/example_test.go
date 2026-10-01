package nowpayments_test

import (
	"fmt"
	"log"

	"github.com/foisalislambd/nowpayments-go"
)

func ExampleNew() {
	client, err := nowpayments.New(nowpayments.Config{
		APIKey:  "your-api-key",
		Sandbox: true,
	})
	if err != nil {
		log.Fatal(err)
	}
	_ = client
}

func ExampleVerifyIPNSignature() {
	payload := `{"payment_id":123,"payment_status":"finished"}`
	signature := "abc123..."
	ipnSecret := "your-ipn-secret"
	valid := nowpayments.VerifyIPNSignature(payload, signature, ipnSecret)
	fmt.Println(valid)
}
