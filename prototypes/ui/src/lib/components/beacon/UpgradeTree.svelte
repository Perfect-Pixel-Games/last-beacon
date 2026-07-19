<script lang="ts">
	import { siloRooms, siloLevel, upgradeTree, type UpgradeNode } from '$lib/placeholder-data';

	let { selectedId = $bindable(null) }: { selectedId?: string | null } = $props();

	const LABEL_W = 72;
	const LANE_W = 112;
	const ROOM_W = LANE_W * 3;
	const ROW_H = 130;
	const HEADER_H = 40;
	const NODE_W = 96;
	const NODE_H = 80;

	const maxRow = Math.max(...upgradeTree.map((node) => node.row));
	const rowsList = Array.from({ length: maxRow }, (_, i) => i + 1);
	const totalWidth = siloRooms.length * ROOM_W;
	const totalHeight = HEADER_H + maxRow * ROW_H;

	const nodesById = Object.fromEntries(upgradeTree.map((node) => [node.id, node]));

	function isAvailable(node: UpgradeNode): boolean {
		if (node.unlocked) return false;
		if (node.row > siloLevel) return false;
		return node.requires.every((id) => nodesById[id].unlocked);
	}

	function nodeTopY(row: number): number {
		return HEADER_H + (row - 1) * ROW_H + (ROW_H - NODE_H) / 2;
	}

	function colX(room: string, lane: number): number {
		return siloRooms.indexOf(room) * ROOM_W + lane * LANE_W + LANE_W / 2;
	}

	function center(node: UpgradeNode) {
		return { x: colX(node.room, node.lane), y: nodeTopY(node.row) + NODE_H / 2 };
	}

	function boxStyle(node: UpgradeNode): string {
		const roomIndex = siloRooms.indexOf(node.room);
		const left = roomIndex * ROOM_W + node.lane * LANE_W + (LANE_W - NODE_W) / 2;
		const top = nodeTopY(node.row);
		return `left:${left}px; top:${top}px; width:${NODE_W}px; height:${NODE_H}px;`;
	}

	type Edge = { from: UpgradeNode; to: UpgradeNode; active: boolean };

	function edgeKey(edge: Edge): string {
		return `${edge.from.id}->${edge.to.id}`;
	}

	// Every `requires` entry either stays in the same lane (any row distance — a straight vertical
	// line is always clear, since a room never places another node in that same lane in between)
	// or changes lane/room, in which case it's always exactly one row back.
	const edges: Edge[] = upgradeTree.flatMap((node) =>
		node.requires.map((reqId) => ({
			from: nodesById[reqId],
			to: node,
			active: nodesById[reqId].unlocked
		}))
	);

	// Cross-lane edges need a horizontal jog through the gap band directly below their source row.
	// By default every jog sits on the band's natural centerline; it only gets nudged apart from
	// another jog if their horizontal spans would actually overlap — like magnets that only push
	// apart when they'd otherwise collide, not a blanket even spread across every jog in the band.
	const JOG_SPACING = 8;
	const jogY = new Map<string, number>();
	const jogsByRow = new Map<number, Edge[]>();
	for (const edge of edges) {
		if (center(edge.from).x === center(edge.to).x) continue;
		const list = jogsByRow.get(edge.from.row) ?? [];
		list.push(edge);
		jogsByRow.set(edge.from.row, list);
	}
	for (const [row, list] of jogsByRow) {
		const spans = list
			.map((edge) => {
				const x1 = center(edge.from).x;
				const x2 = center(edge.to).x;
				return { edge, xMin: Math.min(x1, x2), xMax: Math.max(x1, x2) };
			})
			.sort((a, b) => a.xMin - b.xMin);

		// Greedy interval-graph colouring: each span takes the lowest level whose last-placed span
		// doesn't overlap it, so non-overlapping jogs can all share level 0 (the centerline).
		const levelEnds: number[] = [];
		const levelByKey = new Map<string, number>();
		for (const span of spans) {
			let level = 0;
			while (level < levelEnds.length && levelEnds[level] > span.xMin) level++;
			levelEnds[level] = span.xMax;
			levelByKey.set(edgeKey(span.edge), level);
		}

		const levelCount = levelEnds.length;
		const gapTop = nodeTopY(row) + NODE_H;
		const gapBottom = nodeTopY(row + 1);
		const mid = (gapTop + gapBottom) / 2;
		for (const edge of list) {
			const level = levelByKey.get(edgeKey(edge)) ?? 0;
			jogY.set(edgeKey(edge), mid + (level - (levelCount - 1) / 2) * JOG_SPACING);
		}
	}

	// Circuit-diagram-style crossings: where a jog's horizontal run passes over another edge's
	// vertical run (a same-lane line, or another jog's stub) without actually connecting to it,
	// hop the horizontal line over it with a small arc instead of drawing straight through.
	type VSeg = { key: string; x: number; yTop: number; yBottom: number };
	type HSeg = { key: string; y: number; xLeft: number; xRight: number };

	const vSegs: VSeg[] = [];
	const hSegs: HSeg[] = [];
	for (const edge of edges) {
		const c1 = center(edge.from);
		const c2 = center(edge.to);
		const key = edgeKey(edge);
		if (c1.x === c2.x) {
			vSegs.push({ key, x: c1.x, yTop: c1.y + NODE_H / 2, yBottom: c2.y - NODE_H / 2 });
		} else {
			const y = jogY.get(key)!;
			vSegs.push({ key, x: c1.x, yTop: c1.y + NODE_H / 2, yBottom: y });
			vSegs.push({ key, x: c2.x, yTop: y, yBottom: c2.y - NODE_H / 2 });
			hSegs.push({ key, y, xLeft: Math.min(c1.x, c2.x), xRight: Math.max(c1.x, c2.x) });
		}
	}

	const EPS = 0.5;
	const bumpsByKey = new Map<string, number[]>();
	for (const h of hSegs) {
		const crossings: number[] = [];
		for (const v of vSegs) {
			if (v.key === h.key) continue;
			if (v.x > h.xLeft + EPS && v.x < h.xRight - EPS && h.y > v.yTop + EPS && h.y < v.yBottom - EPS) {
				crossings.push(v.x);
			}
		}
		bumpsByKey.set(h.key, crossings);
	}

	const BUMP_R = 5;

	function jogPath(edge: Edge): string {
		const c1 = center(edge.from);
		const c2 = center(edge.to);
		const y = jogY.get(edgeKey(edge))!;
		const goingRight = c2.x >= c1.x;
		const crossings = (bumpsByKey.get(edgeKey(edge)) ?? []).slice().sort((a, b) => (goingRight ? a - b : b - a));

		let d = `M ${c1.x} ${c1.y + NODE_H / 2} L ${c1.x} ${y} `;
		let lastX = c1.x;
		for (const bx of crossings) {
			const enter = goingRight ? bx - BUMP_R : bx + BUMP_R;
			const exit = goingRight ? bx + BUMP_R : bx - BUMP_R;
			const pastLast = goingRight ? enter > lastX : enter < lastX;
			const beforeEnd = goingRight ? exit < c2.x : exit > c2.x;
			if (!pastLast || !beforeEnd) continue;
			const sweep = goingRight ? 0 : 1;
			d += `L ${enter} ${y} A ${BUMP_R} ${BUMP_R} 0 0 ${sweep} ${exit} ${y} `;
			lastX = exit;
		}
		d += `L ${c2.x} ${y} L ${c2.x} ${c2.y - NODE_H / 2}`;
		return d;
	}
</script>

<div class="flex" style="height:{totalHeight}px;">
	<div
		class="sticky left-0 z-20 shrink-0 border-r border-slate-700 bg-slate-900/95"
		style="width:{LABEL_W}px;"
	>
		<div style="height:{HEADER_H}px;"></div>
		{#each rowsList as row (row)}
			<div
				class="flex flex-col items-center justify-center font-mono text-[10px] uppercase tracking-wider {row >
				siloLevel
					? 'text-slate-600'
					: 'text-slate-400'}"
				style="height:{ROW_H}px;"
			>
				<span>Level {row}</span>
				{#if row > siloLevel}
					<span class="text-[9px] text-slate-700">Locked</span>
				{/if}
			</div>
		{/each}
	</div>

	<div class="relative shrink-0" style="width:{totalWidth}px; height:{totalHeight}px;">
		{#each Array.from({ length: siloRooms.length + 1 }) as _, i (i)}
			<div
				class="absolute top-0 w-px bg-slate-700"
				style="left:{i * ROOM_W}px; height:{totalHeight}px;"
			></div>
		{/each}

		{#each siloRooms as room, i (room)}
			<div
				class="absolute top-0 flex items-center justify-center px-2 text-center font-mono text-[10px] uppercase tracking-wider text-slate-400"
				style="left:{i * ROOM_W}px; width:{ROOM_W}px; height:{HEADER_H}px;"
			>
				{room}
			</div>
		{/each}

		<svg class="pointer-events-none absolute left-0 top-0" width={totalWidth} height={totalHeight}>
			{#each edges as edge (edgeKey(edge))}
				{@const c1 = center(edge.from)}
				{@const c2 = center(edge.to)}
				{@const stroke = edge.active ? 'var(--color-accent-beacon)' : 'var(--color-slate-700)'}
				{@const opacity = edge.active ? '0.7' : '0.5'}
				{#if c1.x === c2.x}
					<line
						x1={c1.x}
						y1={c1.y + NODE_H / 2}
						x2={c2.x}
						y2={c2.y - NODE_H / 2}
						{stroke}
						stroke-width="2"
						{opacity}
					/>
				{:else}
					<path d={jogPath(edge)} fill="none" {stroke} stroke-width="2" {opacity} />
				{/if}
			{/each}
		</svg>

		{#each upgradeTree as node (node.id)}
			{@const available = isAvailable(node)}
			<div
				role="button"
				tabindex="0"
				onclick={() => (selectedId = node.id)}
				onkeydown={(event) => {
					if (event.key === 'Enter' || event.key === ' ') selectedId = node.id;
				}}
				class="absolute flex cursor-pointer flex-col justify-between rounded-sm border-2 p-2 transition-colors {selectedId ===
				node.id
					? 'outline outline-2 outline-offset-2 outline-accent-beacon'
					: ''} {node.unlocked
					? 'border-accent-beacon bg-accent-beacon/10'
					: available
						? 'border-slate-400 bg-slate-800/80 hover:border-slate-300'
						: 'border-slate-700 bg-slate-900/60'}"
				style={boxStyle(node)}
			>
				<p
					class="truncate text-xs font-semibold {node.unlocked
						? 'text-accent-beacon'
						: available
							? 'text-slate-100'
							: 'text-slate-500'}"
				>
					{node.name}
				</p>
				<div class="flex items-center justify-between gap-1">
					<span class="truncate font-mono text-[9px] text-slate-500">{node.cost}</span>
					{#if node.unlocked}
						<span
							class="shrink-0 rounded-sm bg-accent-beacon px-1.5 py-0.5 font-mono text-[9px] font-semibold uppercase text-slate-950"
						>
							Installed
						</span>
					{:else}
						<span
							class="shrink-0 rounded-sm border px-1.5 py-0.5 font-mono text-[9px] uppercase {available
								? 'border-accent-beacon text-accent-beacon'
								: 'border-slate-700 text-slate-600'}"
						>
							Upgrade
						</span>
					{/if}
				</div>
			</div>
		{/each}
	</div>
</div>
