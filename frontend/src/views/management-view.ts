import type { CourtesyReport, RevenueReport } from "../models/reports";

export class ManagementView {
  showRevenueReport(report: RevenueReport): void {
    const target = document.querySelector<HTMLElement>("#revenue-result");
    if (target) target.textContent = `Período: ${new Date(report.start).toLocaleDateString("pt-BR")} a ${new Date(report.end).toLocaleDateString("pt-BR")} · Total recebido: ${new Intl.NumberFormat("pt-BR", { style: "currency", currency: "BRL" }).format(Number(report.totalReceived))}`;
  }

  showCourtesyReport(report: CourtesyReport): void {
    const target = document.querySelector<HTMLElement>("#courtesy-result");
    if (target) target.textContent = `Período: ${new Date(report.start).toLocaleDateString("pt-BR")} a ${new Date(report.end).toLocaleDateString("pt-BR")} · Tickets de cortesia: ${report.ticketCount}`;
  }

  showError(message: string): void { const target = document.querySelector<HTMLElement>("#management-error"); if (target) target.textContent = message; }
}

