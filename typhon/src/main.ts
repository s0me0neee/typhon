import { TOML } from "bun"
import { Elysia } from "elysia"
import pino from "pino"

const log = pino({ transport: { target: "pino-pretty" } })
const port = process.env.PORT
if (!port) throw Error("Failed to get port from env var PORT")
const axum_port = process.argv[2]
log.info({ port }, "llm server listening")
log.info({ axum_port }, "configured sim server at")

new Elysia().get("/", "hello elysia").listen(parseInt(port))
