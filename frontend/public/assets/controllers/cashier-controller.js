export class CashierController {
    parkingApi;
    constructor(parkingApi) {
        this.parkingApi = parkingApi;
    }
    async calculateCharge(ticketCode) { }
    async registerPayment(ticketId, amountReceived) { }
}
