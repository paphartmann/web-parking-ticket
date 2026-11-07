import { ParkingApi } from "./services/parking-api.js";

const api = new ParkingApi();
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
    const quote = await api.calculateCharge(code);
    message.textContent = `Valor: ${new Intl.NumberFormat("pt-BR", { style: "currency", currency: "BRL" }).format(Number(quote.amount))}${quote.courtesy ? " (cortesia)" : ""}`;
    document.querySelector<HTMLInputElement>("#payment-ticket-id")!.value = quote.ticketId;
    document.querySelector<HTMLInputElement>("#payment-amount")!.value = quote.amount;
  } catch (cause) {
    message.textContent = cause instanceof Error ? cause.message : "Ocorreu um erro inesperado.";
  }
});

document.querySelector<HTMLFormElement>("#payment-form")!.addEventListener("submit", async (event) => {
  event.preventDefault();
  const message = document.querySelector<HTMLElement>("#payment-result")!;
  try {
    await api.registerPayment(
      document.querySelector<HTMLInputElement>("#payment-ticket-id")!.value,
      document.querySelector<HTMLInputElement>("#payment-amount")!.value,
    );
    message.textContent = "Pagamento registrado.";
    await renderTickets();
  } catch (cause) {
    message.textContent = cause instanceof Error ? cause.message : "Ocorreu um erro inesperado.";
  }
});

void renderTickets();
