export class ManagementController {
    managementApi;
    constructor(managementApi) {
        this.managementApi = managementApi;
    }
    async loadDailyRevenue(day) { }
    async loadMonthlyRevenue(year, month) { }
    async loadDailyCourtesyCount(day) { }
    async loadMonthlyCourtesyCount(year, month) { }
}
