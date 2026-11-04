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
export declare class ParkingApi {
    issueEntryTicket(request: IssueEntryTicketRequest): Promise<EntryTicketResponse>;
    validateExit(request: ValidateExitRequest): Promise<void>;
    calculateCharge(ticketCode: string): Promise<void>;
    registerPayment(ticketId: string, amountReceived: string): Promise<void>;
}
