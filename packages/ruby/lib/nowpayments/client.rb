# frozen_string_literal: true

require_relative 'http'
require_relative 'ipn'
require_relative 'constants'

module NowPayments
  # Main client for NOWPayments API
  # @see https://documenter.getpostman.com/view/7907941/2s93JusNJt
  class Client
    def initialize(api_key:, sandbox: false, base_url: nil, timeout: 30_000, ipn_secret: nil)
      raise ArgumentError, 'NOWPayments API key is required. Get yours at https://account.nowpayments.io' if api_key.to_s.strip.empty?

      # Accept seconds (<=100) or milliseconds (>100)
      t = timeout || 30_000
      timeout_sec = t.to_f > 100 ? t.to_f / 1000.0 : t.to_f

      @config = {
        api_key: api_key.to_s.strip,
        sandbox: sandbox,
        base_url: base_url,
        timeout: timeout_sec,
        ipn_secret: ipn_secret
      }
      @http = Http.new(@config)
    end

    # --- Status & Auth ---

    def get_status
      @http.get('/v1/status')
    end

    def get_auth_token(email, password)
      @http.post('/v1/auth', { email: email, password: password })
    end

    # --- Currencies ---

    def get_currencies(fixed_rate = nil)
      params = fixed_rate.nil? ? {} : { fixed_rate: fixed_rate }
      @http.get('/v1/currencies', params)
    end

    def get_full_currencies
      @http.get('/v1/full-currencies')
    end

    def get_merchant_coins(fixed_rate = nil)
      params = fixed_rate.nil? ? {} : { fixed_rate: fixed_rate }
      @http.get('/v1/merchant/coins', params)
    end

    def get_currency(currency)
      code = currency.to_s.strip
      raise ArgumentError, 'Currency code is required (e.g. "btc", "eth")' if code.empty?

      require 'uri'
      @http.get("/v1/currencies/#{URI.encode_www_form_component(code.to_s)}")
    end

    # --- Estimate & Min Amount ---

    def get_estimate_price(amount:, currency_from:, currency_to:)
      @http.get('/v1/estimate', {
        amount: amount,
        currency_from: currency_from,
        currency_to: currency_to
      })
    end

    def get_min_amount(currency_from:, currency_to:, fiat_equivalent: nil, is_fixed_rate: nil, is_fee_paid_by_user: nil)
      params = { currency_from: currency_from, currency_to: currency_to }
      params[:fiat_equivalent] = fiat_equivalent unless fiat_equivalent.nil?
      params[:is_fixed_rate] = is_fixed_rate unless is_fixed_rate.nil?
      params[:is_fee_paid_by_user] = is_fee_paid_by_user unless is_fee_paid_by_user.nil?
      @http.get('/v1/min-amount', params)
    end

    # --- Payments ---

    def create_payment(params)
      body = params.dup
      # Alias fixed_rate → is_fixed_rate (Node.js compatibility, supports both symbol and string keys)
      fixed_val = body[:fixed_rate] || body["fixed_rate"]
      has_is_fixed = body.key?(:is_fixed_rate) || body.key?("is_fixed_rate")
      if !fixed_val.nil? && !has_is_fixed
        body[:is_fixed_rate] = fixed_val
        body.delete(:fixed_rate)
        body.delete("fixed_rate")
      end
      @http.post('/v1/payment', body)
    end

    def get_payment_status(payment_id)
      raise ArgumentError, 'Payment ID is required' if payment_id.nil? || payment_id.to_s.strip.empty?

      @http.get("/v1/payment/#{payment_id}")
    end

    def get_payments(params = {}, jwt_token = nil)
      @http.get('/v1/payment/', params, jwt_token)
    end

    def update_payment_estimate(payment_id)
      raise ArgumentError, 'Payment ID is required' if payment_id.to_s.strip.empty?

      @http.post("/v1/payment/#{payment_id}/update-merchant-estimate", nil)
    end

    # --- Invoices ---

    def create_invoice(params)
      @http.post('/v1/invoice', params)
    end

    def create_invoice_payment(params)
      @http.post('/v1/invoice-payment', params)
    end

    # --- Payouts ---

    def validate_payout_address(params)
      @http.post('/v1/payout/validate-address', params)
    end

    def create_payout(params, jwt_token)
      raise ArgumentError, 'JWT token is required for create_payout. Call get_auth_token first.' if jwt_token.to_s.strip.empty?

      @http.post('/v1/payout', params, jwt_token)
    end

    def verify_payout(payout_id, verification_code, jwt_token)
      raise ArgumentError, 'JWT token is required for verify_payout. Call get_auth_token first.' if jwt_token.to_s.strip.empty?

      @http.post("/v1/payout/#{payout_id}/verify", { verification_code: verification_code }, jwt_token)
    end

    def cancel_payout(payout_id, jwt_token)
      raise ArgumentError, 'JWT token is required for cancel_payout. Call get_auth_token first.' if jwt_token.to_s.strip.empty?

      @http.post('/v1/payout/w_id/cancel', { payout_id: payout_id }, jwt_token)
    end

    def get_payout_fee(currency, amount)
      raise ArgumentError, 'Currency is required (e.g. "btc", "eth")' if currency.to_s.strip.empty?
      raise ArgumentError, 'Amount is required' if amount.nil?

      @http.get('/v1/payout/fee', { currency: currency, amount: amount })
    end

    def get_payout_status(payout_id, jwt_token = nil)
      @http.get("/v1/payout/#{payout_id}", {}, jwt_token)
    end

    def get_payouts(params = {})
      @http.get('/v1/payout', params)
    end

    # --- Fiat Payouts ---

    def get_fiat_payouts_crypto_currencies(params = {}, jwt_token = nil)
      @http.get('/v1/fiat-payouts/crypto-currencies', params, jwt_token)
    end

    def get_fiat_payouts_payment_methods(params = {}, jwt_token = nil)
      @http.get('/v1/fiat-payouts/payment-methods', params, jwt_token)
    end

    def get_fiat_payouts(params = {}, jwt_token = nil)
      @http.get('/v1/fiat-payouts', params, jwt_token)
    end

    # --- Balance ---

    def get_balance(jwt_token = nil)
      @http.get('/v1/balance', {}, jwt_token)
    end

    # --- Subscriptions ---

    def get_subscriptions(params = {})
      @http.get('/v1/subscriptions', params)
    end

    def get_subscription(id)
      @http.get("/v1/subscriptions/#{id}")
    end

    def delete_subscription(id, jwt_token = nil)
      @http.delete("/v1/subscriptions/#{id}", jwt_token)
    end

    def get_subscription_plans(params = {})
      @http.get('/v1/subscriptions/plans', params)
    end

    def get_subscription_plan(id)
      @http.get("/v1/subscriptions/plans/#{id}")
    end

    def update_subscription_plan(id, updates)
      @http.patch("/v1/subscriptions/plans/#{id}", updates)
    end

    def create_subscription(params, jwt_token)
      raise ArgumentError, 'JWT token is required for create_subscription. Call get_auth_token first.' if jwt_token.to_s.strip.empty?

      @http.post('/v1/subscriptions', params, jwt_token)
    end

    # --- Sub-Partners / Custody ---

    def create_sub_partner(name, jwt_token)
      raise ArgumentError, 'JWT token is required for create_sub_partner. Call get_auth_token first.' if jwt_token.to_s.strip.empty?

      @http.post('/v1/sub-partner/balance', { name: name }, jwt_token)
    end

    def create_sub_partner_payment(params, jwt_token)
      raise ArgumentError, 'JWT token is required for create_sub_partner_payment. Call get_auth_token first.' if jwt_token.to_s.strip.empty?

      body = params.dup
      is_fixed_val = body[:is_fixed_rate] || body['is_fixed_rate']
      has_fixed = body.key?(:fixed_rate) || body.key?('fixed_rate')
      if !is_fixed_val.nil? && !has_fixed
        body[:fixed_rate] = is_fixed_val
        body.delete(:is_fixed_rate)
        body.delete('is_fixed_rate')
      end
      @http.post('/v1/sub-partner/payment', body, jwt_token)
    end

    def get_sub_partners(params = {}, jwt_token = nil)
      @http.get('/v1/sub-partner', params, jwt_token)
    end

    def get_sub_partner_balance(sub_partner_id)
      @http.get("/v1/sub-partner/balance/#{sub_partner_id}")
    end

    def get_transfers(params = {}, jwt_token = nil)
      @http.get('/v1/sub-partner/transfers', params, jwt_token)
    end

    def get_transfer(id, jwt_token = nil)
      @http.get("/v1/sub-partner/transfer/#{id}", {}, jwt_token)
    end

    def create_transfer(params, jwt_token)
      raise ArgumentError, 'JWT token is required for create_transfer. Call get_auth_token first.' if jwt_token.to_s.strip.empty?

      @http.post('/v1/sub-partner/transfer', params, jwt_token)
    end

    def deposit(params, jwt_token)
      raise ArgumentError, 'JWT token is required for deposit. Call get_auth_token first.' if jwt_token.to_s.strip.empty?

      @http.post('/v1/sub-partner/deposit', params, jwt_token)
    end

    def write_off(params, jwt_token)
      raise ArgumentError, 'JWT token is required for write_off. Call get_auth_token first.' if jwt_token.to_s.strip.empty?

      @http.post('/v1/sub-partner/write-off', params, jwt_token)
    end

    # --- Conversions ---

    def create_conversion(params, jwt_token)
      raise ArgumentError, 'JWT token is required for create_conversion. Call get_auth_token first.' if jwt_token.to_s.strip.empty?

      @http.post('/v1/conversion', params, jwt_token)
    end

    def get_conversion_status(conversion_id, jwt_token)
      raise ArgumentError, 'JWT token is required for get_conversion_status. Call get_auth_token first.' if jwt_token.to_s.strip.empty?

      @http.get("/v1/conversion/#{conversion_id}", {}, jwt_token)
    end

    def get_conversions(params = {}, jwt_token = nil)
      @http.get('/v1/conversion', params, jwt_token)
    end

    # --- IPN ---

    def verify_ipn(payload, signature)
      secret = @config[:ipn_secret]
      raise ArgumentError, 'IPN secret not configured. Pass ipn_secret in constructor or use verify_ipn_signature with explicit secret.' if secret.to_s.strip.empty?

      IPN.verify_signature(payload, signature, secret)
    end
  end
end
