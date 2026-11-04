import { ParkingApi } from "./services/parking-api.js";

const api = new ParkingApi();
const form = document.querySelector<HTMLFormElement>("#entry-form")!;
const plateInput = document.querySelector<HTMLInputElement>("#vehicle-plate")!;
const submit = document.querySelector<HTMLButtonElement>("#issue-ticket")!;
const result = document.querySelector<HTMLElement>("#ticket-result")!;
const error = document.querySelector<HTMLElement>("#entry-error")!;

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
  } catch (cause) {
    error.textContent = cause instanceof Error ? cause.message : "Ocorreu um erro inesperado.";
  } finally {
    submit.disabled = false;
    submit.textContent = "Emitir ticket";
  }
});
