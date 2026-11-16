import { Elysia } from "elysia"
import { elysia_port, log } from "./config"

export function startServer() {
	new Elysia()
		.get("/", "hello elysia")
		.listen(elysia_port, (s) =>
			log.info({ port: s.port }, "llm server listening"),
		)
}
