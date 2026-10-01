# frozen_string_literal: true

lib = File.expand_path("lib", __dir__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require "nowpayments/version"

Gem::Specification.new do |spec|
  spec.name          = "nowpayments-ruby"
  spec.version       = NowPayments::VERSION
  spec.authors       = ["Foisalislambd"]
  spec.email         = [""]

  spec.summary       = "Full-featured Ruby SDK for NOWPayments cryptocurrency payment API"
  spec.description   = "Accept 300+ cryptocurrencies with auto-conversion. Payments, invoices, payouts, subscriptions, custody, IPN webhooks."
  spec.homepage      = "https://github.com/Foisalislambd/nowpayments-ruby"
  spec.license       = "MIT"
  spec.required_ruby_version = ">= 3.1.0"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/Foisalislambd/nowpayments-ruby"
  spec.metadata["changelog_uri"] = "https://github.com/Foisalislambd/nowpayments-ruby/blob/main/CHANGELOG.md"

  spec.files = Dir["lib/**/*", "README.md", "LICENSE"]
  spec.require_paths = ["lib"]

  spec.add_dependency "faraday", ">= 2.12", "< 3"
end
