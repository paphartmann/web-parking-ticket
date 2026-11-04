import { ParkingApi } from "./services/parking-api.js";
const api = new ParkingApi();
const form = document.querySelector("#entry-form");
const plateInput = document.querySelector("#vehicle-plate");
const submit = document.querySelector("#issue-ticket");
const result = document.querySelector("#ticket-result");
const error = document.querySelector("#entry-error");
form.addEventListener("submit", async (event) => {
    event.preventDefault();
    error.textContent = "";
    result.hidden = true;
    submit.disabled = true;
    submit.textContent = "Emitindo…";
    try {
        const ticket = await api.issueEntryTicket({ vehiclePlate: plateInput.value.trim() });
        document.querySelector("#ticket-code").textContent = ticket.ticketCode;
        document.querySelector("#ticket-plate").textContent = ticket.vehiclePlate;
        document.querySelector("#ticket-time").textContent = new Date(ticket.enteredAt).toLocaleString("pt-BR");
        result.hidden = false;
        form.reset();
    }
    catch (cause) {
        error.textContent = cause instanceof Error ? cause.message : "Ocorreu um erro inesperado.";
    }
    finally {
        submit.disabled = false;
        submit.textContent = "Emitir ticket";
    }
});
