import type { ChargeQuote } from "../models/parking-ticket";

export class CashierView {
  showChargeQuote(quote: ChargeQuote): void {
    const result = document.querySelector<HTMLElement>("#charge-result");
    if (result) result.textContent = `Valor: ${new Intl.NumberFormat("pt-BR", { style: "currency", currency: "BRL" }).format(Number(quote.amount))}${quote.courtesy ? " (cortesia)" : ""}`;
    const id = document.querySelector<HTMLInputElement>("#payment-ticket-id");
    const amount = document.querySelector<HTMLInputElement>("#payment-amount");
    if (id) id.value = quote.ticketId;
    if (amount) amount.value = quote.amount;
  }

  showPaymentConfirmation(): void { const message = document.querySelector<HTMLElement>("#payment-result"); if (message) message.textContent = "Pagamento registrado."; }

  showError(message: string): void { const target = document.querySelector<HTMLElement>("#cashier-error"); if (target) target.textContent = message; }
}

