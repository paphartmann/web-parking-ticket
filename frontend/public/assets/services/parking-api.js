export class ParkingApi {
    async issueEntryTicket(request) {
        const response = await fetch("/api/barrier/entry-tickets", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(request),
        });
        if (!response.ok)
            throw new Error(response.status === 422 ? "Informe uma placa válida." : "Não foi possível emitir o ticket.");
        return response.json();
    }
    async validateExit(request) { }
    async calculateCharge(ticketCode) { }
    async registerPayment(ticketId, amountReceived) { }
}
