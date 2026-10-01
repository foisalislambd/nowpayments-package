# frozen_string_literal: true

module NowPayments
  # Human-friendly helpers for payment status and display
  module Helpers
    # Check if payment is complete (success or terminal state)
    #
    # @param status [String] Payment status
    # @return [Boolean]
    def self.payment_complete?(status)
      PAYMENT_DONE_STATUSES.include?(status.to_s)
    end

    # Check if payment is still pending (customer should pay)
    #
    # @param status [String] Payment status
    # @return [Boolean]
    def self.payment_pending?(status)
      PAYMENT_PENDING_STATUSES.include?(status.to_s)
    end

    # Get human-readable status label
    #
    # @param status [String] Payment status
    # @return [String]
    def self.status_label(status)
      PAYMENT_STATUS_LABELS[status.to_s] || status.to_s
    end

    # Build a short summary for displaying to users
    # e.g. "Awaiting payment: 0.001234 BTC → bc1q..."
    #
    # @param payment [Hash] Payment object with pay_amount, pay_currency, pay_address, payment_status
    # @return [String]
    def self.payment_summary(payment)
      pay_amount = payment["pay_amount"] || payment[:pay_amount]
      pay_currency = (payment["pay_currency"] || payment[:pay_currency] || "").to_s.upcase
      pay_address = payment["pay_address"] || payment[:pay_address] || "…"
      payment_status = payment["payment_status"] || payment[:payment_status]
      label = PAYMENT_STATUS_LABELS[payment_status.to_s] || payment_status.to_s
      "#{label}: #{pay_amount} #{pay_currency} → #{pay_address}"
    end
  end
end
