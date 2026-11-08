import { ParkingApi } from "./services/parking-api.js";
import { ManagementApi } from "./services/management-api.js";
import { ManagementController } from "./controllers/management-controller.js";
import { ManagementView } from "./views/management-view.js";
import { CashierController } from "./controllers/cashier-controller.js";
import { CashierView } from "./views/cashier-view.js";
import QRCode from "qrcode";

const api = new ParkingApi();
const cashier = new CashierController(api);
const cashierView = new CashierView();
const management = new ManagementController(new ManagementApi());
const managementView = new ManagementView();
const form = document.querySelector<HTMLFormElement>("#entry-form")!;
const plateInput = document.querySelector<HTMLInputElement>("#vehicle-plate")!;
const submit = document.querySelector<HTMLButtonElement>("#issue-ticket")!;
const result = document.querySelector<HTMLElement>("#ticket-result")!;
const error = document.querySelector<HTMLElement>("#entry-error")!;
const ticketList = document.querySelector<HTMLTableSectionElement>("#tickets-body")!;
const listError = document.querySelector<HTMLElement>("#tickets-error")!;

const renderTickets = async () => {
  listError.textContent = "";
  try {
    const tickets = await api.listTickets();
    ticketList.replaceChildren(...tickets.map((ticket) => {
      const row = document.createElement("tr");
      const code = ticket.ticketCode ?? ticket.code ?? "";
      const cells = [code, ticket.vehiclePlate, new Date(ticket.enteredAt).toLocaleString("pt-BR"), ticket.status];
      for (const value of cells) {
        const cell = document.createElement("td");
        cell.textContent = value;
        row.append(cell);
      }
      return row;
    }));
  } catch (cause) {
    listError.textContent = cause instanceof Error ? cause.message : "Ocorreu um erro inesperado.";
  }
};

form.addEventListener("submit", async (event) => {
  event.preventDefault();
  error.textContent = "";
  result.hidden = true;
  submit.disabled = true;
  submit.textContent = "Emitindo…";
  try {
    const ticket = await api.issueEntryTicket({ vehiclePlate: plateInput.value.trim() });
    document.querySelector("#ticket-code")!.textContent = ticket.ticketCode;
    document.querySelector("#ticket-plate")!.textContent = ticket.vehiclePlate;
    document.querySelector("#ticket-time")!.textContent = new Date(ticket.enteredAt).toLocaleString("pt-BR");
    await QRCode.toCanvas(document.querySelector<HTMLCanvasElement>("#ticket-qr")!, ticket.ticketId, { width: 180, margin: 1 });
    result.hidden = false;
    form.reset();
    await renderTickets();
  } catch (cause) {
    error.textContent = cause instanceof Error ? cause.message : "Ocorreu um erro inesperado.";
  } finally {
    submit.disabled = false;
    submit.textContent = "Emitir ticket";
  }
});

document.querySelector<HTMLButtonElement>("#refresh-tickets")!.addEventListener("click", renderTickets);

const exitForm = document.querySelector<HTMLFormElement>("#exit-form")!;
exitForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  const output = document.querySelector<HTMLElement>("#exit-result")!;
  const code = document.querySelector<HTMLInputElement>("#exit-ticket-code")!.value.trim();
  output.textContent = "Validando…";
  try {
    const result = await api.validateExit({ ticketCode: code });
    output.textContent = result.mayOpenBarrier ? "Saída autorizada." : "Saída não autorizada. Verifique o ticket e o pagamento.";
    output.setAttribute("data-authorized", String(result.mayOpenBarrier));
  } catch (cause) {
    output.textContent = cause instanceof Error ? cause.message : "Ocorreu um erro inesperado.";
  }
});

const chargeForm = document.querySelector<HTMLFormElement>("#charge-form")!;
chargeForm.addEventListener("submit", async (event) => {
  event.preventDefault();
  const message = document.querySelector<HTMLElement>("#charge-result")!;
  const code = document.querySelector<HTMLInputElement>("#charge-ticket-code")!.value.trim();
  message.textContent = "Calculando…";
  try {
    const quote = await cashier.calculateCharge(code);
    cashierView.showChargeQuote(quote);
  } catch (cause) {
    cashierView.showError(cause instanceof Error ? cause.message : "Ocorreu um erro inesperado.");
  }
});

document.querySelector<HTMLFormElement>("#payment-form")!.addEventListener("submit", async (event) => {
  event.preventDefault();
  const message = document.querySelector<HTMLElement>("#payment-result")!;
  try {
    await cashier.registerPayment(
      document.querySelector<HTMLInputElement>("#payment-ticket-id")!.value,
      document.querySelector<HTMLInputElement>("#payment-amount")!.value,
    );
    cashierView.showPaymentConfirmation();
    await renderTickets();
  } catch (cause) {
    cashierView.showError(cause instanceof Error ? cause.message : "Ocorreu um erro inesperado.");
  }
});

document.querySelector<HTMLFormElement>("#revenue-form")!.addEventListener("submit", async (event) => {
  event.preventDefault();
  const [year, month] = document.querySelector<HTMLInputElement>("#revenue-period")!.value.split("-").map(Number);
  try { managementView.showRevenueReport(await management.loadMonthlyRevenue(year, month)); }
  catch (cause) { managementView.showError(cause instanceof Error ? cause.message : "Ocorreu um erro inesperado."); }
});

document.querySelector<HTMLFormElement>("#daily-revenue-form")!.addEventListener("submit", async (event) => {
  event.preventDefault();
  try { managementView.showRevenueReport(await management.loadDailyRevenue(document.querySelector<HTMLInputElement>("#daily-revenue-period")!.value)); }
  catch (cause) { managementView.showError(cause instanceof Error ? cause.message : "Ocorreu um erro inesperado."); }
});

document.querySelector<HTMLFormElement>("#yearly-revenue-form")!.addEventListener("submit", async (event) => {
  event.preventDefault();
  const year = Number(document.querySelector<HTMLInputElement>("#yearly-revenue-period")!.value);
  try { managementView.showRevenueReport(await management.loadYearlyRevenue(year)); }
  catch (cause) { managementView.showError(cause instanceof Error ? cause.message : "Ocorreu um erro inesperado."); }
});

document.querySelector<HTMLFormElement>("#courtesy-form")!.addEventListener("submit", async (event) => {
  event.preventDefault();
  const [year, month] = document.querySelector<HTMLInputElement>("#courtesy-period")!.value.split("-").map(Number);
  try { managementView.showCourtesyReport(await management.loadMonthlyCourtesyCount(year, month)); }
  catch (cause) { managementView.showError(cause instanceof Error ? cause.message : "Ocorreu um erro inesperado."); }
});

document.querySelector<HTMLFormElement>("#daily-courtesy-form")!.addEventListener("submit", async (event) => {
  event.preventDefault();
  try { managementView.showCourtesyReport(await management.loadDailyCourtesyCount(document.querySelector<HTMLInputElement>("#daily-courtesy-period")!.value)); }
  catch (cause) { managementView.showError(cause instanceof Error ? cause.message : "Ocorreu um erro inesperado."); }
});

document.querySelector<HTMLFormElement>("#yearly-courtesy-form")!.addEventListener("submit", async (event) => {
  event.preventDefault();
  const year = Number(document.querySelector<HTMLInputElement>("#yearly-courtesy-period")!.value);
  try { managementView.showCourtesyReport(await management.loadYearlyCourtesyCount(year)); }
  catch (cause) { managementView.showError(cause instanceof Error ? cause.message : "Ocorreu um erro inesperado."); }
});

void renderTickets();
