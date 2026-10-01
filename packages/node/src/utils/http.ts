/**
 * HTTP client for NOWPayments API
 */

import axios, { AxiosInstance, AxiosError } from 'axios';
import type { NowPaymentsConfig } from '../types';

const PRODUCTION_URL = 'https://api.nowpayments.io';
const SANDBOX_URL = 'https://api-sandbox.nowpayments.io';

export class NowPaymentsError extends Error {
  constructor(
    message: string,
    public statusCode?: number,
    public code?: string,
    public response?: unknown
  ) {
    super(message);
    this.name = 'NowPaymentsError';
    Object.setPrototypeOf(this, NowPaymentsError.prototype);
  }

  /** Developer-friendly string for logs */
  override toString(): string {
    const parts = [this.message];
    if (this.statusCode) parts.push(`(status: ${this.statusCode})`);
    if (this.code) parts.push(`[${this.code}]`);
    return parts.join(' ');
  }
}

export function createHttpClient(config: NowPaymentsConfig): AxiosInstance {
  const baseURL =
    config.baseUrl ||
    (config.sandbox ? SANDBOX_URL : PRODUCTION_URL);

  const client = axios.create({
    baseURL,
    timeout: config.timeout ?? 30000,
    headers: {
      'Content-Type': 'application/json',
      'x-api-key': config.apiKey.trim(),
    },
  });

  client.interceptors.response.use(
    (response) => response,
    (error: AxiosError) => {
      if (error.response) {
        const data = error.response.data as
          | { message?: string; msg?: string; code?: string; error?: string }
          | undefined;
        const message =
          typeof data?.message === 'string'
            ? data.message
            : typeof data?.msg === 'string'
              ? data.msg
              : typeof data?.error === 'string'
                ? data.error
                : error.message || 'Request failed';
        throw new NowPaymentsError(
          message,
          error.response.status,
          data?.code,
          error.response.data
        );
      }
      const msg =
        error.code === 'ECONNABORTED'
          ? 'Request timed out. Check your connection or try again.'
          : error.message || 'Network error. Check your connection.';
      throw new NowPaymentsError(msg, undefined, error.code, error);
    }
  );

  return client;
}
