# frozen_string_literal: true

module NowPayments
  class NowPaymentsError < StandardError
    attr_reader :status_code, :code, :response

    def initialize(message, status_code = nil, code = nil, response = nil)
      super(message)
      @status_code = status_code
      @code = code
      @response = response
    end

    def to_s
      parts = [message]
      parts << "(status: #{status_code})" if status_code
      parts << "[#{code}]" if code
      parts.join(" ")
    end
  end
end
