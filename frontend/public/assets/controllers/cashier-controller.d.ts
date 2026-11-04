import { ParkingApi } from "../services/parking-api";
export declare class CashierController {
    private readonly parkingApi;
    constructor(parkingApi: ParkingApi);
    calculateCharge(ticketCode: string): Promise<void>;
    registerPayment(ticketId: string, amountReceived: string): Promise<void>;
}
