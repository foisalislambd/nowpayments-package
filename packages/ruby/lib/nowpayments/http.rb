# frozen_string_literal: true

require "faraday"
require "faraday/json"

module NowPayments
  class Http
    def initialize(config)
      @config = config
      @conn = build_connection
    end

    def get(path, params = {}, jwt_token = nil)
      response = @conn.get(path) do |req|
        req.params = params unless params.empty?
        req.headers["Authorization"] = "Bearer #{jwt_token}" if jwt_token.to_s.strip != ""
      end
      handle_response(response)
    rescue Faraday::Error => e
      raise NowPaymentsError.new(
        e.is_a?(Faraday::TimeoutError) ? "Request timed out. Check your connection or try again." : (e.message || "Network error. Check your connection."),
        nil, e.class.name, e
      )
    end

    def post(path, body = nil, jwt_token = nil)
      response = @conn.post(path) do |req|
        req.body = body unless body.nil?
        req.headers["Authorization"] = "Bearer #{jwt_token}" if jwt_token.to_s.strip != ""
      end
      handle_response(response)
    rescue Faraday::Error => e
      raise NowPaymentsError.new(
        e.is_a?(Faraday::TimeoutError) ? "Request timed out. Check your connection or try again." : (e.message || "Network error. Check your connection."),
        nil, e.class.name, e
      )
    end

    def patch(path, body, jwt_token = nil)
      response = @conn.patch(path) do |req|
        req.body = body
        req.headers["Authorization"] = "Bearer #{jwt_token}" if jwt_token.to_s.strip != ""
      end
      handle_response(response)
    rescue Faraday::Error => e
      raise NowPaymentsError.new(
        e.is_a?(Faraday::TimeoutError) ? "Request timed out. Check your connection or try again." : (e.message || "Network error. Check your connection."),
        nil, e.class.name, e
      )
    end

    def delete(path, jwt_token = nil)
      response = @conn.delete(path) do |req|
        req.headers["Authorization"] = "Bearer #{jwt_token}" if jwt_token.to_s.strip != ""
      end
      handle_response(response)
    rescue Faraday::Error => e
      raise NowPaymentsError.new(
        e.is_a?(Faraday::TimeoutError) ? "Request timed out. Check your connection or try again." : (e.message || "Network error. Check your connection."),
        nil, e.class.name, e
      )
    end

    private

    def build_connection
      base_url = @config[:base_url] || (@config[:sandbox] ? SANDBOX_URL : PRODUCTION_URL)
      timeout = @config[:timeout] || 30.0

      Faraday.new(url: base_url) do |f|
        f.request :json
        f.response :json, content_type: /\bjson$/
        f.options.timeout = timeout
        f.headers["Content-Type"] = "application/json"
        f.headers["x-api-key"] = @config[:api_key]
        f.adapter Faraday.default_adapter
      end
    end

    def handle_response(response)
      return response.body if response.success?

      data = response.body
      message = data.is_a?(Hash) ? (data["message"] || data["msg"] || data["error"]) : nil
      message ||= response.reason_phrase || "Request failed"
      raise NowPaymentsError.new(
        message,
        response.status,
        data.is_a?(Hash) ? data["code"] : nil,
        data
      )
    end
  end
end
