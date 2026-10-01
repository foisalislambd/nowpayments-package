# frozen_string_literal: true

# List payments with filters
# Run: ruby examples/03_list_payments.rb

require_relative "../lib/nowpayments"

api_key = ENV["NOWPAYMENTS_API_KEY"] || "your_api_key"
np = NowPayments::Client.new(api_key: api_key, sandbox: true)

result = np.get_payments(
  limit: 5,
  page: 0,
  sortBy: "created_at",
  orderBy: "desc",
  dateFrom: "2024-01-01",
  dateTo: "2024-12-31"
)

puts "Total: #{result["total"]}"
puts "Page: #{result["page"] + 1} of #{result["pagesCount"]}"
puts "\nPayments:"
(result["data"] || []).each_with_index do |p, i|
  puts "  #{i + 1}. #{p["payment_id"]} | #{p["payment_status"]} | #{p["price_amount"]} #{p["price_currency"]}"
end
