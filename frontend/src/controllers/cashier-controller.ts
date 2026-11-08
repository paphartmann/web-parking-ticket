import { ParkingApi } from "../services/parking-api";

export class CashierController {
  constructor(private readonly parkingApi: ParkingApi) {}

  async calculateCharge(ticketCode: string) {
    return this.parkingApi.calculateCharge(ticketCode);
  }

  async registerPayment(ticketId: string, amountReceived: string): Promise<void> {
    await this.parkingApi.registerPayment(ticketId, amountReceived);
  }
}
