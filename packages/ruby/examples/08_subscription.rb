# frozen_string_literal: true

# Subscription plans and create subscription
# Run: ruby examples/08_subscription.rb
# Env: NOWPAYMENTS_API_KEY, EMAIL, PASSWORD (for create_subscription)

require_relative "../lib/nowpayments"

api_key = ENV["NOWPAYMENTS_API_KEY"] || "your_api_key"
np = NowPayments::Client.new(api_key: api_key, sandbox: true)

# List plans (no auth)
plans = np.get_subscription_plans
plans_list = plans["result"] || []
puts "Plans: #{plans_list.size}"

if plans_list.any?
  plan = plans_list[0]
  plan_id = plan["id"]
  puts "Plan: #{plan}"

  # Create subscription (requires JWT)
  email = ENV["EMAIL"]
  password = ENV["PASSWORD"]
  if email && password && plan_id
    auth = np.get_auth_token(email, password)
    token = auth["token"]
    sub = np.create_subscription(
      {
        subscription_plan_id: plan_id,
        email: "customer@example.com"
      },
      token
    )
    puts "Subscription: #{sub["result"]}"
  else
    puts "Set EMAIL and PASSWORD to create subscription"
  end
end
