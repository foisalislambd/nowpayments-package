/**
 * NOWPayments Node.js SDK
 * Full-featured TypeScript client for the NOWPayments cryptocurrency payment API
 *
 * @see https://documenter.getpostman.com/view/7907941/2s93JusNJt
 */

import type { AxiosInstance } from 'axios';
import type {
  NowPaymentsConfig,
  CreatePaymentParams,
  CreateInvoiceParams,
  CreateInvoicePaymentParams,
  CreatePayoutParams,
  CreatePayoutResponse,
  ValidateAddressParams,
  CreateSubPartnerPaymentParams,
  SubPartnerPaymentResponse,
  GetSubscriptionPlansParams,
  GetSubscriptionsParams,
  GetFiatPayoutsParams,
  Payment,
  PaymentsListResponse,
  EstimatePriceResponse,
  MinAmountResponse,
  ListPaymentsParams,
  EstimateParams,
  MinAmountParams,
  InvoiceResponse,
  ApiStatusResponse,
  AuthResponse,
  SubscriptionPlan,
  RecurringPayment,
  SubPartnerBalance,
  FullCurrency,
  FiatPayoutCryptoCurrency,
  FiatPayoutPaymentMethod,
  FiatPayoutRecord,
} from './types';
import { createHttpClient, NowPaymentsError } from './utils/http';
import { verifyIpnSignature, createIpnSignature } from './utils/ipn';
import {
  isPaymentComplete,
  isPaymentPending,
  getStatusLabel,
  getPaymentSummary,
  PAYMENT_STATUS_LABELS,
} from './utils/helpers';

export { NowPaymentsError, verifyIpnSignature, createIpnSignature };
export {
  isPaymentComplete,
  isPaymentPending,
  getStatusLabel,
  getPaymentSummary,
  PAYMENT_STATUS_LABELS,
} from './utils/helpers';
export {
  PAYMENT_STATUSES,
  PAYMENT_DONE_STATUSES,
  PAYMENT_PENDING_STATUSES,
} from './types';
export * from './types';

export class NowPayments {
  private readonly client: AxiosInstance;
  private readonly config: NowPaymentsConfig;

  constructor(config: NowPaymentsConfig) {
    if (!config.apiKey?.trim()) {
      throw new Error(
        'NOWPayments API key is required. Get yours at https://account.nowpayments.io'
      );
    }
    this.config = config;
    this.client = createHttpClient(config);
  }

  /** Check if API is up and available */
  async getStatus(): Promise<ApiStatusResponse> {
    const { data } = await this.client.get<ApiStatusResponse>('/v1/status');
    return data;
  }

  /** Get list of available crypto currencies (e.g. btc, eth, usdt) */
  async getCurrencies(fixedRate?: boolean): Promise<{ currencies: string[] }> {
    const { data } = await this.client.get<{ currencies: string[] }>(
      '/v1/currencies',
      { params: fixedRate != null ? { fixed_rate: fixedRate } : {} }
    );
    return data;
  }

  /** Get full currency details (id, code, name, wallet_regex, network, etc.) */
  async getFullCurrencies(): Promise<{ currencies: FullCurrency[] }> {
    const { data } = await this.client.get<{ currencies: FullCurrency[] }>('/v1/full-currencies');
    return data;
  }

  /** Get merchant checked currencies (from coins settings) */
  async getMerchantCoins(fixedRate?: boolean): Promise<{ currencies: string[] }> {
    const { data } = await this.client.get<{ currencies: string[] }>(
      '/v1/merchant/coins',
      { params: fixedRate != null ? { fixed_rate: fixedRate } : {} }
    );
    return data;
  }

  /**
   * Get JWT token (required for payouts, custody, etc.).
   * Token expires in 5 minutes. Never log email/password.
   *
   * @example
   * const { token } = await np.getAuthToken(email, password);
   * await np.createPayout(params, token);
   */
  async getAuthToken(email: string, password: string): Promise<AuthResponse> {
    const { data } = await this.client.post<AuthResponse>('/v1/auth', {
      email,
      password,
    });
    return data;
  }

  /** Get single currency details (limits, etc.) */
  async getCurrency(currency: string): Promise<unknown> {
    const code = currency?.trim();
    if (!code) {
      throw new Error('Currency code is required (e.g. "btc", "eth")');
    }
    const { data } = await this.client.get(`/v1/currencies/${encodeURIComponent(code)}`);
    return data;
  }

  /** Get estimated price in crypto for a fiat amount */
  async getEstimatePrice(params: EstimateParams): Promise<EstimatePriceResponse> {
    const { data } = await this.client.get<EstimatePriceResponse>('/v1/estimate', {
      params: {
        amount: params.amount,
        currency_from: params.currency_from,
        currency_to: params.currency_to,
      },
    });
    return data;
  }

  /** Get minimum payment amount for currency pair */
  async getMinAmount(params: MinAmountParams): Promise<MinAmountResponse> {
    const query: Record<string, string | boolean> = {
      currency_from: params.currency_from,
      currency_to: params.currency_to,
    };
    if (params.fiat_equivalent != null) query.fiat_equivalent = params.fiat_equivalent;
    if (params.is_fixed_rate != null) query.is_fixed_rate = params.is_fixed_rate;
    if (params.is_fee_paid_by_user != null) query.is_fee_paid_by_user = params.is_fee_paid_by_user;

    const { data } = await this.client.get<MinAmountResponse>('/v1/min-amount', { params: query });
    return data;
  }

  /**
   * Create a new payment. Returns address + amount for customer to pay.
   *
   * @example
   * const p = await np.createPayment({
   *   price_amount: 29.99,
   *   price_currency: 'usd',
   *   pay_currency: 'btc',
   *   order_id: 'order-123',
   * });
   * // Show: Pay p.pay_amount BTC to p.pay_address
   */
  async createPayment(params: CreatePaymentParams): Promise<Payment> {
    const { origin_ip, ...rest } = params;
    const body = { ...rest } as Record<string, unknown>;
    if (body.fixed_rate != null && body.is_fixed_rate == null) {
      body.is_fixed_rate = body.fixed_rate;
    }
    delete body.fixed_rate;
    const headers = origin_ip ? { 'origin-ip': origin_ip } : {};
    const { data } = await this.client.post<Payment>('/v1/payment', body, { headers });
    return data;
  }

  /** Get payment status by ID */
  async getPaymentStatus(paymentId: number | string): Promise<Payment> {
    if (paymentId == null || String(paymentId).trim() === '') {
      throw new Error('Payment ID is required');
    }
    const { data } = await this.client.get<Payment>(
      `/v1/payment/${encodeURIComponent(String(paymentId))}`
    );
    return data;
  }

  /** Get paginated list of payments. JWT recommended per API docs. */
  async getPayments(
    params?: ListPaymentsParams,
    jwtToken?: string
  ): Promise<PaymentsListResponse> {
    const headers = jwtToken ? { Authorization: `Bearer ${jwtToken}` } : {};
    const { data } = await this.client.get<PaymentsListResponse>('/v1/payment/', {
      params: params ?? {},
      headers,
    });
    return data;
  }

  /** Update payment estimate (call before expiration) */
  async updatePaymentEstimate(
    paymentId: number | string
  ): Promise<{ pay_amount: number; expiration_estimate_date: string; id: string; token_id: string }> {
    if (paymentId == null || String(paymentId).trim() === '') {
      throw new Error('Payment ID is required');
    }
    const { data } = await this.client.post(
      `/v1/payment/${encodeURIComponent(String(paymentId))}/update-merchant-estimate`
    );
    return data;
  }

  /** Create an invoice (redirect flow) */
  async createInvoice(params: CreateInvoiceParams): Promise<InvoiceResponse> {
    const { origin_ip, ...body } = params;
    const headers = origin_ip ? { 'origin-ip': origin_ip } : {};
    const { data } = await this.client.post<InvoiceResponse>('/v1/invoice', body, { headers });
    return data;
  }

  /** Create payment for existing invoice */
  async createInvoicePayment(params: CreateInvoicePaymentParams): Promise<Payment> {
    const { data } = await this.client.post<Payment>('/v1/invoice-payment', params);
    return data;
  }

  // --- Recurring Payments (Subscriptions) ---

  /** List all recurring payments */
  async getSubscriptions(params?: GetSubscriptionsParams): Promise<{ count: number; result: RecurringPayment[] }> {
    const { data } = await this.client.get<{
      count: number;
      result: RecurringPayment[];
    }>('/v1/subscriptions', { params: params ?? {} });
    return data;
  }

  /** Get single recurring payment */
  async getSubscription(id: string): Promise<{ result: RecurringPayment }> {
    const { data } = await this.client.get<{ result: RecurringPayment }>(
      `/v1/subscriptions/${id}`
    );
    return data;
  }

  /** Cancel recurring payment. JWT required per API docs. */
  async deleteSubscription(id: string, jwtToken?: string): Promise<{ result: string }> {
    const headers = jwtToken ? { Authorization: `Bearer ${jwtToken}` } : {};
    const { data } = await this.client.delete<{ result: string }>(
      `/v1/subscriptions/${id}`,
      { headers }
    );
    return data;
  }

  /** List subscription plans */
  async getSubscriptionPlans(params?: GetSubscriptionPlansParams): Promise<{
    count: number;
    result: SubscriptionPlan[];
  }> {
    const { data } = await this.client.get<{
      count: number;
      result: SubscriptionPlan[];
    }>('/v1/subscriptions/plans', { params: params ?? {} });
    return data;
  }

  /** Get single subscription plan */
  async getSubscriptionPlan(
    id: string
  ): Promise<{ result: SubscriptionPlan }> {
    const { data } = await this.client.get<{ result: SubscriptionPlan }>(
      `/v1/subscriptions/plans/${id}`
    );
    return data;
  }

  /** Update subscription plan */
  async updateSubscriptionPlan(
    id: string,
    updates: Partial<SubscriptionPlan>
  ): Promise<unknown> {
    const { data } = await this.client.patch(
      `/v1/subscriptions/plans/${id}`,
      updates
    );
    return data;
  }

  // --- Sub-Partner / Customer Management ---

  /** List sub-partners (users). JWT required for custody API. */
  async getSubPartners(
    params?: { id?: number | number[]; offset?: number; limit?: number; order?: 'ASC' | 'DESC' },
    jwtToken?: string
  ): Promise<unknown> {
    const { data } = await this.client.get('/v1/sub-partner', {
      params: params ?? {},
      headers: jwtToken ? { Authorization: `Bearer ${jwtToken}` } : {},
    });
    return data;
  }

  /** Get sub-partner balance */
  async getSubPartnerBalance(
    subPartnerId: string
  ): Promise<{ result: SubPartnerBalance }> {
    const { data } = await this.client.get<{ result: SubPartnerBalance }>(
      `/v1/sub-partner/balance/${subPartnerId}`
    );
    return data;
  }

  /** List transfers. JWT required for custody. */
  async getTransfers(
    params?: {
      id?: number | number[];
      status?: string | string[];
      limit?: number;
      offset?: number;
      order?: 'ASC' | 'DESC';
    },
    jwtToken?: string
  ): Promise<unknown> {
    const { data } = await this.client.get('/v1/sub-partner/transfers', {
      params: params ?? {},
      headers: jwtToken ? { Authorization: `Bearer ${jwtToken}` } : {},
    });
    return data;
  }

  /** Get single transfer. JWT required for custody. */
  async getTransfer(
    id: string,
    jwtToken?: string
  ): Promise<unknown> {
    const { data } = await this.client.get(
      `/v1/sub-partner/transfer/${id}`,
      { headers: jwtToken ? { Authorization: `Bearer ${jwtToken}` } : {} }
    );
    return data;
  }

  /**
   * Create mass payout. Requires JWT (call getAuthToken first).
   */
  async createPayout(
    params: CreatePayoutParams,
    jwtToken: string
  ): Promise<CreatePayoutResponse> {
    if (!jwtToken?.trim()) {
      throw new Error('JWT token is required for createPayout. Call getAuthToken first.');
    }
    const { data } = await this.client.post<CreatePayoutResponse>('/v1/payout', params, {
      headers: { Authorization: `Bearer ${jwtToken}` },
    });
    return data;
  }

  /** Verify payout with 2FA code. Requires JWT. */
  async verifyPayout(
    payoutId: string,
    verificationCode: string,
    jwtToken: string
  ): Promise<string> {
    if (!jwtToken?.trim()) {
      throw new Error('JWT token is required for verifyPayout. Call getAuthToken first.');
    }
    const { data } = await this.client.post<string>(
      `/v1/payout/${payoutId}/verify`,
      { verification_code: verificationCode },
      { headers: { Authorization: `Bearer ${jwtToken}` } }
    );
    return data as string;
  }

  /** Get payout status */
  async getPayoutStatus(
    payoutId: string,
    jwtToken?: string
  ): Promise<unknown> {
    const config = jwtToken
      ? { headers: { Authorization: `Bearer ${jwtToken}` } }
      : {};
    const { data } = await this.client.get(`/v1/payout/${payoutId}`, config);
    return data;
  }

  /** List payouts */
  async getPayouts(params?: {
    batch_id?: string;
    status?: string;
    order_by?: string;
    order?: 'asc' | 'desc';
    date_from?: string;
    date_to?: string;
    limit?: number;
    page?: number;
  }): Promise<unknown> {
    const { data } = await this.client.get('/v1/payout', {
      params: params ?? {},
    });
    return data;
  }

  /** Validate payout address before creating payout */
  async validatePayoutAddress(
    params: ValidateAddressParams
  ): Promise<unknown> {
    const { data } = await this.client.post(
      '/v1/payout/validate-address',
      params
    );
    return data;
  }

  /** Estimate network fee for a payout */
  async getPayoutFee(currency: string, amount: number): Promise<unknown> {
    const code = currency?.trim();
    if (!code) {
      throw new Error('Currency is required (e.g. "btc", "eth")');
    }
    if (amount == null || Number.isNaN(amount)) {
      throw new Error('Amount is required');
    }
    const { data } = await this.client.get('/v1/payout/fee', {
      params: { currency: code, amount },
    });
    return data;
  }

  /** Cancel a scheduled payout (created with execute_at). Use individual payout id, not batch id. Requires JWT. */
  async cancelPayout(payoutId: string, jwtToken: string): Promise<void> {
    if (!jwtToken?.trim()) {
      throw new Error('JWT token is required for cancelPayout. Call getAuthToken first.');
    }
    await this.client.post(
      '/v1/payout/w_id/cancel',
      { payout_id: payoutId },
      { headers: { Authorization: `Bearer ${jwtToken}` } }
    );
  }

  // --- Fiat Payouts API (JWT required) ---

  /** Get crypto currencies available for fiat cashout. Requires JWT. */
  async getFiatPayoutsCryptoCurrencies(
    params?: { provider?: string; currency?: string },
    jwtToken?: string
  ): Promise<{ result: FiatPayoutCryptoCurrency[] }> {
    const headers = jwtToken ? { Authorization: `Bearer ${jwtToken}` } : {};
    const { data } = await this.client.get('/v1/fiat-payouts/crypto-currencies', {
      params: params ?? {},
      headers,
    });
    return data as { result: FiatPayoutCryptoCurrency[] };
  }

  /** Get payment methods for fiat payout (provider + currency). Requires JWT. */
  async getFiatPayoutsPaymentMethods(
    params?: { provider?: string; currency?: string },
    jwtToken?: string
  ): Promise<{ result: FiatPayoutPaymentMethod[] }> {
    const headers = jwtToken ? { Authorization: `Bearer ${jwtToken}` } : {};
    const { data } = await this.client.get('/v1/fiat-payouts/payment-methods', {
      params: params ?? {},
      headers,
    });
    return data as { result: FiatPayoutPaymentMethod[] };
  }

  /** List fiat payouts with filters. Requires JWT. */
  async getFiatPayouts(
    params?: GetFiatPayoutsParams,
    jwtToken?: string
  ): Promise<{ result: { rows: FiatPayoutRecord[] } }> {
    const headers = jwtToken ? { Authorization: `Bearer ${jwtToken}` } : {};
    const { data } = await this.client.get('/v1/fiat-payouts', {
      params: params ?? {},
      headers,
    });
    return data as { result: { rows: FiatPayoutRecord[] } };
  }

  /** Get custody balance (currencies + amount). Needs JWT for some setups. */
  async getBalance(jwtToken?: string): Promise<unknown> {
    const config = jwtToken
      ? { headers: { Authorization: `Bearer ${jwtToken}` } }
      : {};
    const { data } = await this.client.get('/v1/balance', config);
    return data;
  }

  // --- Custody / Sub-Partner (requires JWT) ---

  /** Create new user account (sub-partner) */
  async createSubPartner(
    name: string,
    jwtToken: string
  ): Promise<{ result: { id: string; name: string; created_at: string; updated_at: string } }> {
    if (!jwtToken?.trim()) {
      throw new Error('JWT token is required for createSubPartner. Call getAuthToken first.');
    }
    const { data } = await this.client.post(
      '/v1/sub-partner/balance',
      { name },
      { headers: { Authorization: `Bearer ${jwtToken}` } }
    );
    return data as { result: { id: string; name: string; created_at: string; updated_at: string } };
  }

  /**
   * Deposit with payment – top up a sub-partner's balance via crypto payment.
   * Customer pays to the returned address; funds go to sub-partner's custody.
   * Requires JWT.
   *
   * @example
   * const { token } = await np.getAuthToken(email, password);
   * const { result } = await np.createSubPartnerPayment(
   *   { currency: 'trx', amount: 50, sub_partner_id: '1631380403' },
   *   token
   * );
   * // Show customer: Pay result.pay_amount TRX to result.pay_address
   */
  async createSubPartnerPayment(
    params: CreateSubPartnerPaymentParams,
    jwtToken: string
  ): Promise<SubPartnerPaymentResponse> {
    if (!jwtToken?.trim()) {
      throw new Error('JWT token is required for createSubPartnerPayment. Call getAuthToken first.');
    }
    const body = { ...params } as Record<string, unknown>;
    if (body.is_fixed_rate != null && body.fixed_rate == null) {
      body.fixed_rate = body.is_fixed_rate;
      delete body.is_fixed_rate;
    }
    const { data } = await this.client.post<SubPartnerPaymentResponse>(
      '/v1/sub-partner/payment',
      body,
      { headers: { Authorization: `Bearer ${jwtToken}` } }
    );
    return data;
  }

  /** Create subscription (email or custody). Requires JWT. */
  async createSubscription(
    params: {
      subscription_plan_id: number | string;
      sub_partner_id?: number | string;
      email?: string;
    },
    jwtToken: string
  ): Promise<{ result: RecurringPayment }> {
    if (!jwtToken?.trim()) {
      throw new Error('JWT token is required for createSubscription. Call getAuthToken first.');
    }
    const { data } = await this.client.post<{ result: RecurringPayment }>(
      '/v1/subscriptions',
      params,
      { headers: { Authorization: `Bearer ${jwtToken}` } }
    );
    return data;
  }

  /** Transfer between user accounts. Requires JWT. */
  async createTransfer(
    params: { currency: string; amount: number; from_id: number | string; to_id: number | string },
    jwtToken: string
  ): Promise<unknown> {
    if (!jwtToken?.trim()) {
      throw new Error('JWT token is required for createTransfer. Call getAuthToken first.');
    }
    const { data } = await this.client.post(
      '/v1/sub-partner/transfer',
      params,
      { headers: { Authorization: `Bearer ${jwtToken}` } }
    );
    return data;
  }

  /** Write off from user to master account. Requires JWT. */
  async writeOff(
    params: { currency: string; amount: number; sub_partner_id: string | number },
    jwtToken: string
  ): Promise<unknown> {
    if (!jwtToken?.trim()) {
      throw new Error('JWT token is required for writeOff. Call getAuthToken first.');
    }
    const { data } = await this.client.post(
      '/v1/sub-partner/write-off',
      params,
      { headers: { Authorization: `Bearer ${jwtToken}` } }
    );
    return data;
  }

  /** Deposit from master to user account. Requires JWT. */
  async deposit(
    params: { currency: string; amount: number; sub_partner_id: string | number },
    jwtToken: string
  ): Promise<unknown> {
    if (!jwtToken?.trim()) {
      throw new Error('JWT token is required for deposit. Call getAuthToken first.');
    }
    const { data } = await this.client.post(
      '/v1/sub-partner/deposit',
      params,
      { headers: { Authorization: `Bearer ${jwtToken}` } }
    );
    return data;
  }

  // --- Conversions (custody, requires JWT) ---

  /** Create conversion within custody account */
  async createConversion(
    params: { amount: number; from_currency: string; to_currency: string },
    jwtToken: string
  ): Promise<unknown> {
    if (!jwtToken?.trim()) {
      throw new Error('JWT token is required for createConversion. Call getAuthToken first.');
    }
    const { data } = await this.client.post(
      '/v1/conversion',
      params,
      { headers: { Authorization: `Bearer ${jwtToken}` } }
    );
    return data;
  }

  /** Get conversion status */
  async getConversionStatus(
    conversionId: string,
    jwtToken: string
  ): Promise<unknown> {
    if (!jwtToken?.trim()) {
      throw new Error('JWT token is required for getConversionStatus. Call getAuthToken first.');
    }
    const { data } = await this.client.get(
      `/v1/conversion/${conversionId}`,
      { headers: { Authorization: `Bearer ${jwtToken}` } }
    );
    return data;
  }

  /** List conversions */
  async getConversions(
    params?: {
      id?: number | number[];
      status?: string | string[];
      from_currency?: string;
      to_currency?: string;
      created_at_from?: string;
      created_at_to?: string;
      limit?: number;
      offset?: number;
      order?: 'ASC' | 'DESC';
    },
    jwtToken?: string
  ): Promise<unknown> {
    const { data } = await this.client.get('/v1/conversion', {
      params: params ?? {},
      headers: jwtToken ? { Authorization: `Bearer ${jwtToken}` } : {},
    });
    return data;
  }

  /**
   * Verify IPN webhook signature. Use ipnSecret in config.
   *
   * @example
   * // Express: if (np.verifyIpn(req.body, req.headers['x-nowpayments-sig']))
   * // Fastify: if (np.verifyIpn(req.body, req.headers['x-nowpayments-sig']))
   */
  verifyIpn(payload: string | Record<string, unknown>, signature: string): boolean {
    const secret = this.config.ipnSecret;
    if (!secret) {
      throw new Error(
        'IPN secret not configured. Pass ipnSecret in constructor or use verifyIpnSignature() with explicit secret.'
      );
    }
    return verifyIpnSignature(payload, signature, secret);
  }
}

export default NowPayments;
