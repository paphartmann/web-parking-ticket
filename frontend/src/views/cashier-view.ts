import type { ChargeQuote } from "../models/parking-ticket";

export class CashierView {
  showChargeQuote(quote: ChargeQuote): void {}

  showPaymentConfirmation(): void {}

  showError(message: string): void {}
}

