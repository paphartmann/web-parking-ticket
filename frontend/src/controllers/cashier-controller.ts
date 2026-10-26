import { ParkingApi } from "../services/parking-api";

export class CashierController {
  constructor(private readonly parkingApi: ParkingApi) {}

  async calculateCharge(ticketCode: string) {}

  async registerPayment(ticketId: string, amountReceived: string): Promise<void> {}
}
