import { ManagementApi } from "../services/management-api";

export class ManagementController {
  constructor(private readonly managementApi: ManagementApi) {}

  async loadDailyRevenue(day: string) { return this.managementApi.dailyRevenue(day); }

  async loadMonthlyRevenue(year: number, month: number) { return this.managementApi.monthlyRevenue(year, month); }

  async loadYearlyRevenue(year: number) { return this.managementApi.yearlyRevenue(year); }

  async loadDailyCourtesyCount(day: string) { return this.managementApi.dailyCourtesyCount(day); }

  async loadMonthlyCourtesyCount(year: number, month: number) { return this.managementApi.monthlyCourtesyCount(year, month); }

  async loadYearlyCourtesyCount(year: number) { return this.managementApi.yearlyCourtesyCount(year); }
}

