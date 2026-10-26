export type TicketStatus = "active" | "courtesy" | "paid" | "released";

export interface ParkingTicket {
  id: string;
  code: string;
  vehiclePlate: string;
  enteredAt: string;
  paidAt: string | null;
  releasedAt: string | null;
  status: TicketStatus;
}

export interface ChargeQuote {
  ticketId: string;
  amount: string;
  courtesy: boolean;
}

