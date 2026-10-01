/**
 * Example: Subscription plans and create subscription
 * Run: npx tsx examples/08-subscription.ts
 * Env: NOWPAYMENTS_API_KEY, EMAIL, PASSWORD (for createSubscription)
 */
import { NowPayments } from '../src';

const np = new NowPayments({
  apiKey: process.env.NOWPAYMENTS_API_KEY!,
  sandbox: true,
});

async function main() {
  // List plans (no auth)
  const plans = await np.getSubscriptionPlans();
  console.log('Plans:', plans.result?.length ?? 0);

  const plansList = plans.result ?? [];
  if (plansList.length > 0) {
    const plan = plansList[0];
    const planId = plan.id;
    console.log('Plan:', plan);

    // Create subscription (requires JWT)
    const email = process.env.EMAIL;
    const password = process.env.PASSWORD;
    if (email && password && planId) {
      const { token } = await np.getAuthToken(email, password);
      const sub = await np.createSubscription(
        {
          subscription_plan_id: planId,
          email: 'customer@example.com',
        },
        token
      );
      console.log('Subscription:', sub.result);
    } else {
      console.log('Set EMAIL and PASSWORD to create subscription');
    }
  }
}

main().catch(console.error);
