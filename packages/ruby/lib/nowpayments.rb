# frozen_string_literal: true

require "nowpayments/version"
require "nowpayments/constants"
require "nowpayments/error"
require "nowpayments/http"
require "nowpayments/ipn"
require "nowpayments/helpers"
require "nowpayments/client"

module NowPayments
  # Convenience aliases (Node.js style)
  def self.verify_ipn_signature(payload, signature, ipn_secret)
    IPN.verify_signature(payload, signature, ipn_secret)
  end

  def self.create_ipn_signature(payload, ipn_secret)
    IPN.create_signature(payload, ipn_secret)
  end
end
