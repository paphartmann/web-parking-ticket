export declare class ManagementApi {
    dailyRevenue(day: string): Promise<void>;
    monthlyRevenue(year: number, month: number): Promise<void>;
    dailyCourtesyCount(day: string): Promise<void>;
    monthlyCourtesyCount(year: number, month: number): Promise<void>;
}
