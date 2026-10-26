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

export class ParkingApi {
  async issueEntryTicket(request: IssueEntryTicketRequest) {}

  async validateExit(request: ValidateExitRequest) {}

  async calculateCharge(ticketCode: string) {}

  async registerPayment(ticketId: string, amountReceived: string): Promise<void> {}
}

