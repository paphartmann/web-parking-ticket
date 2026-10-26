import { ManagementApi } from "../services/management-api";

export class ManagementController {
  constructor(private readonly managementApi: ManagementApi) {}

  async loadDailyRevenue(day: string) {}

  async loadMonthlyRevenue(year: number, month: number) {}

  async loadDailyCourtesyCount(day: string) {}

  async loadMonthlyCourtesyCount(year: number, month: number) {}
}

