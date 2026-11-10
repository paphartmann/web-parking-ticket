export interface IssueEntryTicketRequest {
  vehiclePlate: string;
}

export interface ValidateExitRequest {
  ticketCode?: string;
  vehiclePlate?: string;
}

export interface ExitValidationResponse {
  ticketId: string;
  mayOpenBarrier: boolean;
}

export interface EntryTicketResponse {
  ticketId: string;
  ticketCode: string;
  vehiclePlate: string;
  enteredAt: string;
  status: string;
}

export interface ParkingTicketResponse extends EntryTicketResponse {
  id?: string;
  code?: string;
  vehiclePlate: string;
  paidAt: string | null;
  releasedAt: string | null;
  status: string;
}

export interface ChargeQuoteResponse {
  ticketId: string;
  amount: string;
  courtesy: boolean;
}

export interface PaymentResponse {
  paymentId: string;
  ticketId: string;
  amount: string;
  paidAt: string;
}

export class ParkingApi {
  async issueEntryTicket(request: IssueEntryTicketRequest): Promise<EntryTicketResponse> {
    const response = await fetch("/api/barrier/entry-tickets", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(request),
    });
    if (!response.ok) throw new Error(response.status === 422 ? "Informe uma placa válida." : "Não foi possível emitir o ticket.");
    return response.json() as Promise<EntryTicketResponse>;
  }

  async listTickets(): Promise<ParkingTicketResponse[]> {
    const response = await fetch("/api/tickets");
    if (!response.ok) throw new Error("Não foi possível carregar os tickets.");
    return response.json() as Promise<ParkingTicketResponse[]>;
  }

  async validateExit(request: ValidateExitRequest): Promise<ExitValidationResponse> {
    const response = await fetch("/api/barrier/exit-validation", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(request),
    });
    if (!response.ok) throw new Error("Não foi possível validar a saída.");
    return response.json() as Promise<ExitValidationResponse>;
  }

  async openGate(request: ValidateExitRequest): Promise<string> {
    const response = await fetch("/api/barrier/open_gate", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(request),
    });
    if (!response.ok) throw new Error("Não foi possível abrir a cancela.");
    return response.json() as Promise<string>;
  }

  async calculateCharge(ticketCode: string): Promise<ChargeQuoteResponse> {
    const response = await fetch(`/api/cashier/tickets/${encodeURIComponent(ticketCode)}/charge`);
    if (!response.ok) throw new Error("Não foi possível calcular o valor do ticket.");
    return response.json() as Promise<ChargeQuoteResponse>;
  }

  async registerPayment(ticketId: string, amountReceived: string): Promise<PaymentResponse> {
    const response = await fetch("/api/cashier/payments", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ ticketId, amountReceived }),
    });
    if (!response.ok) throw new Error("Não foi possível registrar o pagamento.");
    return response.json() as Promise<PaymentResponse>;
  }
}

