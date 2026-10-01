/**
 * NOWPayments API Type Definitions
 */

/** Payment status values – use PAYMENT_STATUSES for iteration. API may return "sending" or "spending". */
export type PaymentStatus =
  | 'waiting'
  | 'confirming'
  | 'confirmed'
  | 'spending'
  | 'sending'  /* API docs variation */
  | 'partially_paid'
  | 'finished'
  | 'failed'
  | 'refunded'
  | 'expired';

/** All possible payment statuses (useful for validation) */
export const PAYMENT_STATUSES: readonly PaymentStatus[] = [
  'waiting',
  'confirming',
  'confirmed',
  'spending',
  'sending',
  'partially_paid',
  'finished',
  'failed',
  'refunded',
  'expired',
] as const;

/** Statuses that mean payment is done (success or terminal) */
export const PAYMENT_DONE_STATUSES: PaymentStatus[] = [
  'finished',
  'failed',
  'refunded',
  'expired',
];

/** Statuses that mean customer should still pay */
export const PAYMENT_PENDING_STATUSES: PaymentStatus[] = [
  'waiting',
  'confirming',
  'confirmed',
  'spending',
  'sending',
  'partially_paid',
];

/** Client configuration options */
export interface NowPaymentsConfig {
  /** Your NOWPayments API key (required) */
  apiKey: string;
  /** Use sandbox API (api-sandbox.nowpayments.io) */
  sandbox?: boolean;
  /** Custom base URL (overrides sandbox) */
  baseUrl?: string;
  /** Request timeout in ms */
  timeout?: number;
  /** IPN secret for verifying webhook callbacks */
  ipnSecret?: string;
}

/** Create payment request */
export interface CreatePaymentParams {
  price_amount: number;
  price_currency: string;
  pay_currency: string;
  pay_amount?: number;
  ipn_callback_url?: string;
  order_id?: string;
  order_description?: string;
  purchase_id?: string;
  payout_address?: string;
  payout_currency?: string;
  payout_extra_id?: string;
  /** @deprecated Use is_fixed_rate */
  fixed_rate?: boolean;
  is_fixed_rate?: boolean;
  is_fee_paid_by_user?: boolean;
  /** Customer IP for fiat2crypto payments — sent as `origin-ip` header, not in body */
  origin_ip?: string;
}

/** Create invoice request */
export interface CreateInvoiceParams {
  price_amount: number;
  price_currency: string;
  pay_currency?: string;
  ipn_callback_url?: string;
  order_id?: string;
  order_description?: string;
  success_url?: string;
  cancel_url?: string;
  partially_paid_url?: string;
  is_fixed_rate?: boolean;
  is_fee_paid_by_user?: boolean;
  /** Customer IP for fiat2crypto payments — sent as `origin-ip` header, not in body */
  origin_ip?: string;
}

/** Full currency details from GET /v1/full-currencies */
export interface FullCurrency {
  id: number;
  code: string;
  name: string;
  enable: boolean;
  wallet_regex?: string;
  priority?: number;
  extra_id_exists?: boolean;
  extra_id_regex?: string | null;
  logo_url?: string;
  track?: boolean;
  cg_id?: string;
  is_maxlimit?: boolean;
  network?: string;
  smart_contract?: string | null;
  network_precision?: number | null;
  [key: string]: unknown;
}

/** Fiat payout crypto currency option */
export interface FiatPayoutCryptoCurrency {
  provider: string;
  currencyCode: string;
  currencyNetwork: string;
  enabled: boolean;
}

/** Fiat payout payment method */
export interface FiatPayoutPaymentMethod {
  name: string;
  paymentCode: string;
  fields: Array<{ name: string; type: string; mandatory: boolean; description?: string }>;
  provider: string;
}

/** Fiat payout record */
export interface FiatPayoutRecord {
  id: string;
  provider: string;
  requestId: string;
  status: string;
  fiatCurrencyCode?: string;
  fiatAmount?: string;
  cryptoCurrencyCode?: string;
  cryptoCurrencyAmount?: string;
  fiatAccountCode?: string;
  fiatAccountNumber?: string;
  payoutDescription?: string | null;
  error?: string | null;
  createdAt?: string;
  updatedAt?: string;
}

/** List subscription plans params */
export interface GetSubscriptionPlansParams {
  limit?: number;
  offset?: number;
}

/** List subscriptions params */
export interface GetSubscriptionsParams {
  status?: string;
  subscription_plan_id?: string | number;
  is_active?: boolean;
  limit?: number;
  offset?: number;
}

/** List fiat payouts params */
export interface GetFiatPayoutsParams {
  id?: string;
  provider?: string;
  requestId?: string;
  fiatCurrency?: string;
  cryptoCurrency?: string;
  status?: string;
  filter?: string;
  provider_payout_id?: string;
  limit?: number;
  page?: number;
  orderBy?: string;
  sortBy?: string;
  dateFrom?: string;
  dateTo?: string;
}

/** Payment object from API */
export interface Payment {
  /** API may return number or string depending on endpoint */
  payment_id: number | string;
  payment_status: PaymentStatus;
  pay_address: string;
  pay_amount: number;
  pay_currency: string;
  price_amount: number;
  price_currency: string;
  actually_paid?: number;
  outcome_amount?: number;
  outcome_currency?: string;
  order_id?: string;
  order_description?: string;
  purchase_id?: string;
  created_at?: string;
  updated_at?: string;
}

/** Paginated payments list */
export interface PaymentsListResponse {
  data: Payment[];
  limit: number;
  page: number;
  pagesCount: number;
  total: number;
}

/** Estimate price response */
export interface EstimatePriceResponse {
  amount_from: number;
  currency_from: string;
  currency_to: string;
  estimated_amount: number;
}

/** Minimum amount response */
export interface MinAmountResponse {
  currency_from: string;
  currency_to: string;
  min_amount: number;
  fiat_equivalent?: number;
}

/** List payments query params */
export interface ListPaymentsParams {
  limit?: number;
  page?: number;
  sortBy?: string;
  orderBy?: 'asc' | 'desc';
  dateFrom?: string;
  dateTo?: string;
}

/** Get estimate params */
export interface EstimateParams {
  amount: number;
  currency_from: string;
  currency_to: string;
}

/** Get min amount params */
export interface MinAmountParams {
  currency_from: string;
  currency_to: string;
  /** Fiat currency code for equivalent amount in response (e.g. 'usd') */
  fiat_equivalent?: string;
  is_fixed_rate?: boolean;
  is_fee_paid_by_user?: boolean;
}

/** Invoice response */
export interface InvoiceResponse {
  id: string;
  invoice_id?: string;
  invoice_url: string;
  price_amount: number;
  price_currency: string;
  pay_currency?: string;
  order_id?: string;
  order_description?: string;
  [key: string]: unknown;
}

/** Subscription plan */
export interface SubscriptionPlan {
  id: string;
  amount: number;
  currency: string;
  interval_day: string;
  title: string;
  created_at?: string;
  updated_at?: string;
}

/** Recurring payment/subscription */
export interface RecurringPayment {
  id: string;
  subscription_plan_id: string;
  status: string;
  is_active: boolean;
  created_at?: string;
  updated_at?: string;
  expire_date?: string;
  subscriber?: { email?: string; sub_partner_id?: string };
}

/** Sub-partner balance */
export interface SubPartnerBalance {
  subPartnerId: string;
  balances: Record<string, { amount: number; pendingAmount: number }>;
}

/** Payout withdrawal item for createPayout */
export interface PayoutWithdrawal {
  address: string;
  currency: string;
  amount: number;
  extra_id?: string;
  ipn_callback_url?: string;
  payout_description?: string;
  unique_external_id?: string;
  fiat_amount?: number;
  fiat_currency?: string;
  /** ISO date string for scheduled payout (e.g. "2024-01-15T10:00:00Z"). Cancel via cancelPayout with individual payout id. */
  execute_at?: string;
}

/** Create payout request body */
export interface CreatePayoutParams {
  ipn_callback_url?: string;
  payout_description?: string;
  withdrawals: PayoutWithdrawal[];
}

/** Create payout response (batch) */
export interface CreatePayoutResponse {
  id: string;
  withdrawals: Array<{
    id: string;
    address: string;
    currency: string;
    amount: string;
    status: string;
    batch_withdrawal_id: string;
    [key: string]: unknown;
  }>;
}

/** Auth response (JWT token for payouts, etc.) */
export interface AuthResponse {
  token: string;
}

/** Validate address params */
export interface ValidateAddressParams {
  address: string;
  currency: string;
  extra_id?: string;
}

/** Create sub-partner deposit payment (top up sub-partner balance) */
export interface CreateSubPartnerPaymentParams {
  currency: string;
  amount: number;
  sub_partner_id: string | number;
  fixed_rate?: boolean;
  /** Alias mapped to fixed_rate for API */
  is_fixed_rate?: boolean;
}

/** Sub-partner payment response (wrapped in result). API may return string ids. */
export interface SubPartnerPaymentResponse {
  result: Omit<Payment, 'payment_id'> & {
    payment_id: number | string;
    amount_received?: number;
    ipn_callback_url?: string | null;
    smart_contract?: string | null;
    network?: string;
    network_precision?: number | null;
    time_limit?: number | null;
    burning_percent?: number | null;
    expiration_estimate_date?: string;
  };
}

/** Create payment for existing invoice */
export interface CreateInvoicePaymentParams {
  iid: number | string;
  pay_currency?: string;
  purchase_id?: string;
  order_description?: string;
  customer_email?: string;
  payout_address?: string;
  payout_extra_id?: string;
  payout_currency?: string;
}

/** API error response */
export interface ApiErrorResponse {
  message?: string;
  code?: string;
  status?: boolean;
  statusCode?: number;
}

/** API status response (GET /v1/status) */
export interface ApiStatusResponse {
  status?: string;
  message?: string;
  [key: string]: unknown;
}
