import { render } from "preact"
import { useEffect, useRef, useState } from "preact/hooks"
import type { Biome, World } from "../bindings/terrain"

const COLOR: Record<Biome, string> = {
	Plain: "#9bc56b",
	Water: "#3a7bd5",
	Wood: "#2f6b34",
	Hill: "#a08a62",
	Desert: "#e3cf8a",
}
const CELL = 24

function Map({ world }: { world: World }) {
	const ref = useRef<HTMLCanvasElement>(null)
	useEffect(() => {
		const ctx = ref.current!.getContext("2d")!
		for (const c of world.cells) {
			const x = c.u * CELL + (c.v % 2) * (CELL / 2) + CELL / 2
			const y = c.v * CELL + CELL / 2
			ctx.beginPath()
			ctx.arc(x, y, CELL / 2 - 1, 0, Math.PI * 2)
			ctx.fillStyle = COLOR[c.terrain.biome]
			ctx.fill()
		}
	}, [world])
	return (
		<canvas
			ref={ref}
			width={world.width * CELL + CELL / 2}
			height={world.height * CELL}
		/>
	)
}

function App() {
	const [world, setWorld] = useState<World>()
	useEffect(() => {
		fetch("/api/world")
			.then((r) => r.json())
			.then(setWorld)
	}, [])
	return world ? <Map world={world} /> : <p>loading…</p>
}

render(<App />, document.getElementById("app")!)
