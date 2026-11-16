import { axum, axum_port, log } from "./config"
import { startServer } from "./server"

log.info({ axum_port }, "configured sim server at")

startServer()

console.log("-".repeat(process.stdout.columns ?? 80))

const rsp = await fetch(axum("/health"))
log.info({ body: await rsp.text() }, "Health")
