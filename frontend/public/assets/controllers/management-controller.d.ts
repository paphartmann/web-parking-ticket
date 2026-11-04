import { ManagementApi } from "../services/management-api";
export declare class ManagementController {
    private readonly managementApi;
    constructor(managementApi: ManagementApi);
    loadDailyRevenue(day: string): Promise<void>;
    loadMonthlyRevenue(year: number, month: number): Promise<void>;
    loadDailyCourtesyCount(day: string): Promise<void>;
    loadMonthlyCourtesyCount(year: number, month: number): Promise<void>;
}
