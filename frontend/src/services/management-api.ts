import type { CourtesyReport, RevenueReport } from "../models/reports.js";

export class ManagementApi {
  private async get<T>(query: URLSearchParams, report: "revenue" | "courtesy"): Promise<T> {
    const response = await fetch(`/api/management/reports/${report}?${query}`);
    if (!response.ok) throw new Error("Não foi possível carregar o relatório.");
    return response.json() as Promise<T>;
  }

  async dailyRevenue(day: string): Promise<RevenueReport> {
    return this.get(new URLSearchParams({ day }), "revenue");
  }

  async monthlyRevenue(year: number, month: number): Promise<RevenueReport> {
    return this.get(new URLSearchParams({ year: String(year), month: String(month) }), "revenue");
  }

  async yearlyRevenue(year: number): Promise<RevenueReport> {
    return this.get(new URLSearchParams({ year: String(year) }), "revenue");
  }

  async dailyCourtesyCount(day: string): Promise<CourtesyReport> {
    return this.get(new URLSearchParams({ day }), "courtesy");
  }

  async monthlyCourtesyCount(year: number, month: number): Promise<CourtesyReport> {
    return this.get(new URLSearchParams({ year: String(year), month: String(month) }), "courtesy");
  }

  async yearlyCourtesyCount(year: number): Promise<CourtesyReport> {
    return this.get(new URLSearchParams({ year: String(year) }), "courtesy");
  }
}

