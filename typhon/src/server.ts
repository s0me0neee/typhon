import { Elysia } from "elysia"
import page from "../ui/index.html"
import { axum, elysia_port, log } from "./config"

export function startServer() {
	new Elysia()
		.get("/", page)
		.get("/api/world", () => fetch(axum("/view/world")))
		.listen(elysia_port, (s) =>
			log.info(
				{ url: `http://localhost:${s.port}` },
				"ui and llm server listening",
			),
		)
}
