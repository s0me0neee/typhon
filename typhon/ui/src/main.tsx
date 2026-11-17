import { render } from "preact"
import { useEffect, useRef, useState } from "preact/hooks"
import type { Status } from "../../bindings/simulation"
import type { Biome, World } from "../../bindings/terrain"

function StepButton() {
	const [status, setStatus] = useState<Status>()
	const [busy, setBusy] = useState(false)
	useEffect(() => {
		fetch("/api/status")
			.then((r) => r.json())
			.then(setStatus)
	}, [])
	const step = async () => {
		setBusy(true)
		const r = await fetch("/api/step", { method: "POST" })
		setStatus(await r.json())
		setBusy(false)
	}
	return (
		<button onClick={step} disabled={busy || !status}>
			Year {status?.year ?? "…"} → step
		</button>
	)
}

const COLOR: Record<Biome, string> = {
	Plain: "#9bc56b",
	Water: "#3a7bd5",
	Wood: "#2f6b34",
	Hill: "#a08a62",
	Desert: "#e3cf8a",
}
// Pointy-top hexes: odd rows shift half a hex right, rows are 3/4 of a hex height apart.
const HEX_W = 8
const HEX_R = HEX_W / Math.sqrt(3)
const ROW_H = HEX_R * 1.5
const HEX_CORNERS = Array.from({ length: 6 }, (_, k) => {
	const a = (Math.PI / 3) * k - Math.PI / 2
	return [HEX_R * Math.cos(a), HEX_R * Math.sin(a)] as const
})

function hexPath(ctx: CanvasRenderingContext2D, cx: number, cy: number) {
	ctx.beginPath()
	for (const [dx, dy] of HEX_CORNERS) ctx.lineTo(cx + dx, cy + dy)
	ctx.closePath()
}

type Layer = "biome" | "elevation"

const HEAT: [number, number, number][] = [
	[68, 1, 84],
	[59, 82, 139],
	[33, 145, 140],
	[94, 201, 98],
	[253, 231, 37],
]

function heat(t: number): string {
	const p = Math.min(Math.max(t, 0), 1) * (HEAT.length - 1)
	const i = Math.min(Math.floor(p), HEAT.length - 2)
	const f = p - i
	const [a, b] = [HEAT[i]!, HEAT[i + 1]!]
	const mix = (k: number) => Math.round(a[k]! + (b[k]! - a[k]!) * f)
	return `rgb(${mix(0)}, ${mix(1)}, ${mix(2)})`
}

function elevationBounds(world: World): [number, number] {
	let lo = Infinity
	let hi = -Infinity
	for (const c of world.cells) {
		lo = Math.min(lo, c.terrain.elevation)
		hi = Math.max(hi, c.terrain.elevation)
	}
	return [lo, hi]
}

function Map({ world, layer }: { world: World; layer: Layer }) {
	const ref = useRef<HTMLCanvasElement>(null)
	const [lo, hi] = elevationBounds(world)
	const cssW = world.width * HEX_W + HEX_W / 2
	const cssH = (world.height - 1) * ROW_H + HEX_R * 2
	const dpr = window.devicePixelRatio || 1
	useEffect(() => {
		const canvas = ref.current!
		const ctx = canvas.getContext("2d")!
		ctx.setTransform(dpr, 0, 0, dpr, 0, 0)
		ctx.clearRect(0, 0, cssW, cssH)
		ctx.lineWidth = 0.6
		for (const c of world.cells) {
			const cx = c.u * HEX_W + (c.v % 2) * (HEX_W / 2) + HEX_W / 2
			const cy = c.v * ROW_H + HEX_R
			const color =
				layer === "biome"
					? COLOR[c.terrain.biome]
					: heat((c.terrain.elevation - lo) / (hi - lo || 1))
			hexPath(ctx, cx, cy)
			ctx.fillStyle = color
			ctx.fill()
			// Same-colour outline hides the anti-aliasing hairlines between neighbours.
			ctx.strokeStyle = color
			ctx.stroke()
		}
	}, [world, layer, dpr])
	return (
		<div>
			<canvas
				ref={ref}
				width={Math.ceil(cssW * dpr)}
				height={Math.ceil(cssH * dpr)}
				style={{ width: `${cssW}px`, height: `${cssH}px`, display: "block" }}
			/>
			{layer === "elevation" && <Legend lo={lo} hi={hi} />}
		</div>
	)
}

function Legend({ lo, hi }: { lo: number; hi: number }) {
	const stops = HEAT.map(([r, g, b]) => `rgb(${r}, ${g}, ${b})`).join(", ")
	return (
		<div
			style={{ display: "flex", alignItems: "center", gap: 8, fontSize: 12 }}
		>
			<span>{lo.toFixed(0)}</span>
			<div
				style={{
					width: 200,
					height: 10,
					background: `linear-gradient(to right, ${stops})`,
				}}
			/>
			<span>{hi.toFixed(0)}</span>
		</div>
	)
}

function LayerToggle({
	layer,
	setLayer,
}: {
	layer: Layer
	setLayer: (l: Layer) => void
}) {
	return (
		<span>
			{(["biome", "elevation"] as const).map((l) => (
				<label key={l} style={{ marginLeft: 12 }}>
					<input
						type="radio"
						name="layer"
						checked={layer === l}
						onChange={() => setLayer(l)}
					/>
					{l}
				</label>
			))}
		</span>
	)
}

function App() {
	const [world, setWorld] = useState<World>()
	const [layer, setLayer] = useState<Layer>("biome")
	useEffect(() => {
		fetch("/api/world")
			.then((r) => r.json())
			.then(setWorld)
	}, [])
	return (
		<>
			<StepButton />
			<LayerToggle layer={layer} setLayer={setLayer} />
			{world ? <Map world={world} layer={layer} /> : <p>loading…</p>}
		</>
	)
}

render(<App />, document.getElementById("app")!)
