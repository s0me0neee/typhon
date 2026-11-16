import { TOML } from "bun"

const cwd = process.cwd()
const conf = Bun.file(`${cwd}/conf.toml`)

if (!(await conf.exists())) {
	throw Error(`Can't find conf.toml at ${cwd}/conf.toml`)
}

type Conf = { axum_port: number; elysia_port: number }
const elysia_port = (TOML.parse(await conf.text()) as Conf).elysia_port
const axum_port = (TOML.parse(await conf.text()) as Conf).axum_port

const axum_path = cwd + "/typhon-server"
const elysia_path = cwd + "/typhon"
const log_level = process.env.LOG_LEVEL

const rust_log = log_level
	? { RUST_LOG: `server=${log_level},tower_http=${log_level}` }
	: {}
const bun_log = log_level ? { LOG_LEVEL: log_level } : {}

async function waitFor(url: string, timeoutMs = 120_000) {
	const start = Date.now()
	while (Date.now() - start < timeoutMs) {
		try {
			const r = await fetch(url)
			if (r.ok) return
		} catch {}
		await Bun.sleep(100)
	}
	throw new Error(`timeout: ${url}`)
}

const axum = Bun.spawn(
	["cargo", "run", "-p", "server", elysia_port.toString()],
	{
		cwd,
		env: { ...process.env, PORT: axum_port.toString(), ...rust_log },
		stdout: "inherit",
		stderr: "inherit",
	},
)

await Promise.race([
	waitFor(`http://127.0.0.1:${axum_port}/health`),
	axum.exited.then((c) => {
		throw new Error(`axum exited ${c}`)
	}),
])

const elysia = Bun.spawn(
	["bun", "run", `${elysia_path}/src/main.ts`, axum_port.toString()],
	{
		cwd,
		env: { ...process.env, PORT: elysia_port.toString(), ...bun_log },
		stdout: "inherit",
		stderr: "inherit",
	},
)
