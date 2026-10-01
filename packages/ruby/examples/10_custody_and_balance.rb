# frozen_string_literal: true

# Balance + custody (sub-partner, deposit with payment, transfer)
# Run: ruby examples/10_custody_and_balance.rb
# Env: NOWPAYMENTS_API_KEY, EMAIL, PASSWORD, SUB_PARTNER_ID (for create_sub_partner_payment)

require_relative "../lib/nowpayments"

api_key = ENV["NOWPAYMENTS_API_KEY"] || "your_api_key"
np = NowPayments::Client.new(api_key: api_key, sandbox: true)

# Balance
balance = np.get_balance
puts "Balance: #{balance}"

# Sub-partners (if using custody)
partners = np.get_sub_partners
puts "Sub-partners: #{partners}"

# Deposit with payment – top up sub-partner via crypto (requires JWT)
email = ENV["EMAIL"]
password = ENV["PASSWORD"]
sub_partner_id = ENV["SUB_PARTNER_ID"]

if email && password && sub_partner_id
  auth = np.get_auth_token(email, password)
  token = auth["token"]
  result = np.create_sub_partner_payment(
    { currency: "trx", amount: 50, sub_partner_id: sub_partner_id },
    token
  )
  r = result["result"] || result
  puts "Deposit payment: #{r["pay_address"]} #{r["pay_amount"]} #{r["pay_currency"]}"
end
