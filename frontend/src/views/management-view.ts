import type { CourtesyReport, RevenueReport } from "../models/reports";

export class ManagementView {
  showRevenueReport(report: RevenueReport): void {}

  showCourtesyReport(report: CourtesyReport): void {}

  showError(message: string): void {}
}

