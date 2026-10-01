# frozen_string_literal: true

module NowPayments
  PRODUCTION_URL = "https://api.nowpayments.io"
  SANDBOX_URL = "https://api-sandbox.nowpayments.io"

  PAYMENT_STATUSES = %w[
    waiting confirming confirmed spending sending
    partially_paid finished failed refunded expired
  ].freeze

  PAYMENT_DONE_STATUSES = %w[finished failed refunded expired].freeze
  PAYMENT_PENDING_STATUSES = %w[waiting confirming confirmed spending sending partially_paid].freeze

  PAYMENT_STATUS_LABELS = {
    "waiting" => "Awaiting payment",
    "confirming" => "Confirming",
    "confirmed" => "Confirmed",
    "spending" => "Processing",
    "sending" => "Sending to wallet",
    "partially_paid" => "Partially paid",
    "finished" => "Completed",
    "failed" => "Failed",
    "refunded" => "Refunded",
    "expired" => "Expired"
  }.freeze
end
