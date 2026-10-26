# Parking Control

Parking Control is a web application for managing paid parking at shopping centers. It is designed around three workflows: vehicle entry and exit, cashier payment, and management reporting. The backend is organized in layers, separating the domain, application use cases, persistence, and presentation.

## Features

- Issue a parking ticket when a vehicle enters.
- Validate a vehicle's exit using its ticket code or license plate.
- Calculate parking charges and record cashier payments.
- Report daily or monthly revenue and courtesy ticket counts.
- Store parking tickets and payments in PostgreSQL.

The pricing policy includes 15 minutes of courtesy, R$ 15.00 for stays up to one hour, and R$ 4.50 for each additional hour portion.

## Technology

- **Backend:** Rust, Rocket, SeaORM, PostgreSQL
- **Frontend:** TypeScript, served as static files by Rocket

## API overview

### Barrier

`POST /api/barrier/entry-tickets` issues an entry ticket.

```json
{
  "vehiclePlate": "ABC1234"
}
```

The result includes the ticket ID and code, vehicle plate, and entry time.

`POST /api/barrier/exit-validation` checks whether the vehicle may leave. Provide a ticket code or vehicle plate.

```json
{
  "ticketCode": "ticket-code",
  "vehiclePlate": "ABC1234"
}
```

The result includes the ticket ID and `mayOpenBarrier` decision.

### Cashier

`GET /api/cashier/tickets/{ticketCode}/charge` returns the amount due and whether the courtesy period applies. The response contains `ticketId`, `amount`, and `courtesy`.

`POST /api/cashier/payments` records the payment and releases the ticket.

```json
{
  "ticketId": "uuid",
  "amountReceived": "15.00"
}
```

### Management reports

- `GET /api/management/reports/revenue?day=YYYY-MM-DD` returns revenue for a day.
- `GET /api/management/reports/revenue?year=YYYY&month=MM` returns revenue for a month.
- `GET /api/management/reports/courtesy?day=YYYY-MM-DD` returns the courtesy ticket count for a day.
- `GET /api/management/reports/courtesy?year=YYYY&month=MM` returns the courtesy ticket count for a month.

Revenue results contain the report period (`start`, `end`) and `totalReceived`. Courtesy results contain the period and `ticketCount`.

## Project structure

- `src/domain` contains parking and payment entities, repository contracts, domain errors, and pricing rules.
- `src/application` defines the barrier, cashier, and management use cases.
- `src/infrastructure/persistence` contains PostgreSQL persistence and repository implementations.
- `src/presentation` contains the API and MVC controller contracts.
- `frontend` contains TypeScript models, services, controllers, views, and the static entry page.

## Running locally

Run the Rust application from the repository root:

```sh
cargo run
```

Rocket serves static files from `frontend/public`. Build the TypeScript frontend with:

```sh
cd frontend
npm install
npm run build
```

The application expects PostgreSQL for persistence. Configure a reachable database before using persistence features.
