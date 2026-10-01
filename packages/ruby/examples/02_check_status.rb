# frozen_string_literal: true

# Check payment status
# Run: ruby examples/02_check_status.rb PAYMENT_ID

require_relative "../lib/nowpayments"

api_key = ENV["NOWPAYMENTS_API_KEY"] || "your_api_key"
payment_id = ARGV[0] || "PASTE_PAYMENT_ID_HERE"

np = NowPayments::Client.new(api_key: api_key, sandbox: true)
payment = np.get_payment_status(payment_id)

puts "Payment #{payment_id}:"
puts "  Status: #{payment["payment_status"]}"
puts "  Amount: #{payment["pay_amount"]} #{payment["pay_currency"]}"
puts "  Address: #{payment["pay_address"]}"

label = NowPayments::Helpers.status_label(payment["payment_status"])
puts "  Label: #{label}"

if NowPayments::Helpers.payment_complete?(payment["payment_status"])
  puts "\nPayment is complete (terminal state)"
elsif NowPayments::Helpers.payment_pending?(payment["payment_status"])
  puts "\nPayment is still pending"
end
