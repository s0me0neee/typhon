import pino from "pino"
import pretty from "pino-pretty"

export const log = pino(pretty({ sync: true }))

const port = process.env.PORT
if (!port) throw Error("Failed to get port from env var PORT")
export const elysia_port = parseInt(port)
export const axum_port = process.argv[2]

export const axum = (route: string) => `http://127.0.0.1:${axum_port}${route}`
