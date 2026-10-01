# frozen_string_literal: true

# Create a payment - show address to customer
# Run: ruby examples/01_create_payment.rb

require_relative "../lib/nowpayments"

api_key = ENV["NOWPAYMENTS_API_KEY"] || "your_api_key"
np = NowPayments::Client.new(api_key: api_key, sandbox: true)

payment = np.create_payment(
  price_amount: 29.99,
  price_currency: "usd",
  pay_currency: "btc",
  order_id: "order-#{Time.now.to_i}",
  order_description: "Premium Plan",
  ipn_callback_url: "https://yoursite.com/webhook"
)

puts "Payment created: #{payment["payment_id"]}"
puts "Pay #{payment["pay_amount"]} #{(payment["pay_currency"] || "").to_s.upcase} to:"
puts payment["pay_address"]
puts "\nStatus: #{payment["payment_status"]}"
