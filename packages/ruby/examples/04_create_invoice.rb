# frozen_string_literal: true

# Create invoice (redirect customer to URL)
# Run: ruby examples/04_create_invoice.rb

require_relative "../lib/nowpayments"

api_key = ENV["NOWPAYMENTS_API_KEY"] || "your_api_key"
np = NowPayments::Client.new(api_key: api_key, sandbox: true)

invoice = np.create_invoice(
  price_amount: 49.99,
  price_currency: "usd",
  pay_currency: "btc",
  order_id: "inv-#{Time.now.to_i}",
  order_description: "Premium subscription",
  success_url: "https://yoursite.com/success",
  cancel_url: "https://yoursite.com/cancel"
)

puts "Invoice created!"
puts "Redirect customer to: #{invoice["invoice_url"]}"
