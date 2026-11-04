import type { CourtesyReport, RevenueReport } from "../models/reports";
export declare class ManagementView {
    showRevenueReport(report: RevenueReport): void;
    showCourtesyReport(report: CourtesyReport): void;
    showError(message: string): void;
}
