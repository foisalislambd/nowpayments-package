# frozen_string_literal: true

require "json"
require "openssl"

module NowPayments
  # IPN (Instant Payment Notification) verification utilities
  # Matches official docs: sort keys recursively, then HMAC-SHA512
  # @see https://nowpayments.io/help/payments/api
  module IPN
    # Recursively sort object keys (matches NOWPayments IPN spec).
    # Normalizes to string keys for consistent JSON output (matches Node.js JSON.parse behavior).
    def self.sort_object(obj)
      return obj unless obj.is_a?(Hash)

      normalized = obj.transform_keys(&:to_s)
      normalized.keys.sort.each_with_object({}) do |key, result|
        val = normalized[key]
        result[key] = val.is_a?(Hash) ? sort_object(val) : val
        result
      end
    end

    # Verify IPN callback signature from NOWPayments.
    # Safe to call – handles invalid input gracefully.
    #
    # Prefer the raw HTTP body string when possible. Framework-parsed hashes can
    # coerce number/string types and break verification.
    #
    # @param payload [String, Hash] Raw request body (string recommended) or parsed object
    # @param signature [String] Value from x-nowpayments-sig header
    # @param ipn_secret [String] Your IPN Secret from Dashboard → Store Settings
    # @return [Boolean] true if signature is valid, false otherwise
    def self.verify_signature(payload, signature, ipn_secret)
      return false if signature.nil? || signature.to_s.strip.empty?
      return false if ipn_secret.nil? || ipn_secret.to_s.strip.empty?

      obj = case payload
            when String
              JSON.parse(payload)
            when Hash
              payload
            else
              return false
            end

      json_string = JSON.generate(sort_object(obj))
      computed_sig = OpenSSL::HMAC.hexdigest('SHA512', ipn_secret.strip, json_string)

      sig_hex = signature.to_s.strip.downcase
      sig_hex = sig_hex.split("=", 2).last if sig_hex.include?("=")
      sig_hex = sig_hex.gsub(/[^a-f0-9]/, "")
      sig_bytes = [sig_hex].pack('H*')
      computed_bytes = [computed_sig.downcase].pack('H*')
      return false if sig_bytes.bytesize != computed_bytes.bytesize || sig_bytes.empty?

      OpenSSL::fixed_length_secure_compare(sig_bytes, computed_bytes)
    rescue JSON::ParserError, ArgumentError
      false
    end

    # Create IPN signature for testing (e.g., mocking callbacks)
    #
    # @param payload [Hash] Payload to sign
    # @param ipn_secret [String] IPN secret
    # @return [String] Hex-encoded HMAC-SHA512 signature
    def self.create_signature(payload, ipn_secret)
      json_string = JSON.generate(sort_object(payload))
      OpenSSL::HMAC.hexdigest('SHA512', ipn_secret.strip, json_string)
    end
  end
end
