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

  async validateExit(request: ValidateExitRequest) {}

  async calculateCharge(ticketCode: string) {}

  async registerPayment(ticketId: string, amountReceived: string): Promise<void> {}
}

