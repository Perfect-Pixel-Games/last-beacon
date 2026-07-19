// Mock content for the whitebox prototype, sourced from README.md's system descriptions.
// Values are illustrative placeholders, not real game data.

// Broad module groups shown as Fabrication's category tabs. Each groups several specific
// module types (shown as a small tag on each item) so the tab bar stays short and scannable.
export const moduleCategories = ['Movement', 'Electrical', 'Salvage', 'Defense'];

export interface ModuleItem {
	id: string;
	category: string;
	type: string;
	name: string;
	weight: number;
	power: number;
	storage: number;
	durability: number;
	mobility: number;
	cost: number;
	feature?: string;
}

// Items currently held in storage, browsable per module category in Fabrication.
// Stat fields are deltas the item would apply to the equipped robot if added.
export const moduleInventory: ModuleItem[] = [
	{
		id: 'mod-bat-01',
		category: 'Electrical',
		type: 'Batteries',
		name: 'Cell Pack Mk.I',
		weight: 10,
		power: 3,
		storage: 0,
		durability: 5,
		mobility: 0,
		cost: 150
	},
	{
		id: 'mod-bat-02',
		category: 'Electrical',
		type: 'Batteries',
		name: 'Solar Cell Pack',
		weight: 14,
		power: 5,
		storage: 0,
		durability: 0,
		mobility: 0,
		cost: 220,
		feature: 'Solar Panels'
	},
	{
		id: 'mod-cargo-01',
		category: 'Salvage',
		type: 'Cargo Bays',
		name: 'Standard Bay',
		weight: 25,
		power: 0,
		storage: 4,
		durability: 0,
		mobility: 0,
		cost: 180
	},
	{
		id: 'mod-cargo-02',
		category: 'Salvage',
		type: 'Cargo Bays',
		name: 'Reinforced Bay',
		weight: 40,
		power: 0,
		storage: 8,
		durability: 5,
		mobility: 0,
		cost: 260
	},
	{
		id: 'mod-laser-01',
		category: 'Salvage',
		type: 'Mining Lasers',
		name: 'Cutter Mk.I',
		weight: 15,
		power: 4,
		storage: 0,
		durability: 0,
		mobility: 0,
		cost: 200
	},
	{
		id: 'mod-laser-02',
		category: 'Salvage',
		type: 'Mining Lasers',
		name: 'Cutter Mk.II',
		weight: 22,
		power: 7,
		storage: 0,
		durability: 0,
		mobility: 0,
		cost: 320,
		feature: 'Deep Drill'
	},
	{
		id: 'mod-scan-01',
		category: 'Electrical',
		type: 'Scanners',
		name: 'Motion Scanner',
		weight: 5,
		power: 1,
		storage: 0,
		durability: 0,
		mobility: 0,
		cost: 90
	},
	{
		id: 'mod-scan-02',
		category: 'Electrical',
		type: 'Scanners',
		name: 'Long Range Scanner',
		weight: 8,
		power: 2,
		storage: 0,
		durability: 0,
		mobility: 0,
		cost: 260,
		feature: 'Long Range Scanner'
	},
	{
		id: 'mod-sensor-01',
		category: 'Electrical',
		type: 'Sensors',
		name: 'Motion Sensor',
		weight: 3,
		power: 1,
		storage: 0,
		durability: 0,
		mobility: 0,
		cost: 70
	},
	{
		id: 'mod-sensor-02',
		category: 'Electrical',
		type: 'Sensors',
		name: 'Thermal Sensor',
		weight: 4,
		power: 1,
		storage: 0,
		durability: 0,
		mobility: 0,
		cost: 140,
		feature: 'Thermal Vision'
	},
	{
		id: 'mod-armour-01',
		category: 'Defense',
		type: 'Armour',
		name: 'Light Plating',
		weight: 30,
		power: 0,
		storage: 0,
		durability: 10,
		mobility: -3,
		cost: 150
	},
	{
		id: 'mod-armour-02',
		category: 'Defense',
		type: 'Armour',
		name: 'Heavy Plating',
		weight: 60,
		power: 0,
		storage: 0,
		durability: 20,
		mobility: -8,
		cost: 280
	},
	{
		id: 'mod-armour-03',
		category: 'Defense',
		type: 'Armour',
		name: 'Composite Plating',
		weight: 25,
		power: 0,
		storage: 0,
		durability: 8,
		mobility: -2,
		cost: 170
	},
	{
		id: 'mod-armour-04',
		category: 'Defense',
		type: 'Armour',
		name: 'Ceramic Plating',
		weight: 28,
		power: 0,
		storage: 0,
		durability: 12,
		mobility: -3,
		cost: 210
	},
	{
		id: 'mod-armour-05',
		category: 'Defense',
		type: 'Armour',
		name: 'Titanium Plating',
		weight: 45,
		power: 0,
		storage: 0,
		durability: 18,
		mobility: -5,
		cost: 320
	},
	{
		id: 'mod-armour-06',
		category: 'Defense',
		type: 'Armour',
		name: 'Spaced Armour',
		weight: 38,
		power: 0,
		storage: 0,
		durability: 14,
		mobility: -4,
		cost: 260
	},
	{
		id: 'mod-armour-07',
		category: 'Defense',
		type: 'Armour',
		name: 'Reinforced Hull Plating',
		weight: 50,
		power: 0,
		storage: 0,
		durability: 16,
		mobility: -6,
		cost: 300
	},
	{
		id: 'mod-armour-08',
		category: 'Defense',
		type: 'Armour',
		name: 'Ablative Shielding',
		weight: 22,
		power: 0,
		storage: 0,
		durability: 9,
		mobility: -2,
		cost: 190
	},
	{
		id: 'mod-armour-09',
		category: 'Defense',
		type: 'Armour',
		name: 'Reactive Plating',
		weight: 42,
		power: 2,
		storage: 0,
		durability: 22,
		mobility: -6,
		cost: 380,
		feature: 'Reactive Shielding'
	},
	{
		id: 'mod-armour-10',
		category: 'Defense',
		type: 'Armour',
		name: 'Deflector Array',
		weight: 20,
		power: 6,
		storage: 0,
		durability: 15,
		mobility: -1,
		cost: 420,
		feature: 'Energy Deflector'
	},
	{
		id: 'mod-armour-11',
		category: 'Defense',
		type: 'Armour',
		name: 'Kinetic Dampers',
		weight: 26,
		power: 0,
		storage: 0,
		durability: 11,
		mobility: -3,
		cost: 230
	},
	{
		id: 'mod-armour-12',
		category: 'Defense',
		type: 'Armour',
		name: 'Impact Foam Layer',
		weight: 15,
		power: 0,
		storage: 0,
		durability: 6,
		mobility: -1,
		cost: 110
	},
	{
		id: 'mod-armour-13',
		category: 'Defense',
		type: 'Armour',
		name: 'Layered Composite',
		weight: 33,
		power: 0,
		storage: 0,
		durability: 13,
		mobility: -4,
		cost: 250
	},
	{
		id: 'mod-armour-14',
		category: 'Defense',
		type: 'Armour',
		name: 'Scrap Plating',
		weight: 35,
		power: 0,
		storage: 0,
		durability: 7,
		mobility: -5,
		cost: 90
	},
	{
		id: 'mod-armour-15',
		category: 'Defense',
		type: 'Armour',
		name: 'Salvaged Plating',
		weight: 32,
		power: 0,
		storage: 0,
		durability: 6,
		mobility: -4,
		cost: 80
	},
	{
		id: 'mod-armour-16',
		category: 'Defense',
		type: 'Armour',
		name: 'Riot Shield Plating',
		weight: 24,
		power: 0,
		storage: 0,
		durability: 10,
		mobility: -3,
		cost: 180
	},
	{
		id: 'mod-armour-17',
		category: 'Defense',
		type: 'Armour',
		name: 'Blast Panels',
		weight: 48,
		power: 0,
		storage: 0,
		durability: 19,
		mobility: -7,
		cost: 340
	},
	{
		id: 'mod-armour-18',
		category: 'Defense',
		type: 'Armour',
		name: 'Thermal Plating',
		weight: 20,
		power: 1,
		storage: 0,
		durability: 8,
		mobility: -2,
		cost: 200
	},
	{
		id: 'mod-armour-19',
		category: 'Defense',
		type: 'Armour',
		name: 'Radiation Shielding',
		weight: 36,
		power: 0,
		storage: 0,
		durability: 14,
		mobility: -5,
		cost: 310
	},
	{
		id: 'mod-armour-20',
		category: 'Defense',
		type: 'Armour',
		name: 'EM Shielding',
		weight: 18,
		power: 3,
		storage: 0,
		durability: 9,
		mobility: -1,
		cost: 260
	},
	{
		id: 'mod-armour-21',
		category: 'Defense',
		type: 'Armour',
		name: 'Mesh Armour',
		weight: 14,
		power: 0,
		storage: 0,
		durability: 5,
		mobility: -1,
		cost: 120
	},
	{
		id: 'mod-armour-22',
		category: 'Defense',
		type: 'Armour',
		name: 'Segmented Plating',
		weight: 29,
		power: 0,
		storage: 0,
		durability: 12,
		mobility: -3,
		cost: 220
	},
	{
		id: 'mod-armour-23',
		category: 'Defense',
		type: 'Armour',
		name: 'Bunker Plating',
		weight: 70,
		power: 0,
		storage: 0,
		durability: 28,
		mobility: -12,
		cost: 480
	},
	{
		id: 'mod-armour-24',
		category: 'Defense',
		type: 'Armour',
		name: 'Riveted Plating',
		weight: 27,
		power: 0,
		storage: 0,
		durability: 10,
		mobility: -3,
		cost: 170
	},
	{
		id: 'mod-armour-25',
		category: 'Defense',
		type: 'Armour',
		name: 'Polymer Composite Armour',
		weight: 19,
		power: 0,
		storage: 0,
		durability: 8,
		mobility: -1,
		cost: 240
	},
	{
		id: 'mod-armour-26',
		category: 'Defense',
		type: 'Armour',
		name: 'Nano-Weave Plating',
		weight: 16,
		power: 1,
		storage: 0,
		durability: 10,
		mobility: -1,
		cost: 360
	},
	{
		id: 'mod-armour-27',
		category: 'Defense',
		type: 'Armour',
		name: 'Adaptive Camo Plating',
		weight: 17,
		power: 5,
		storage: 0,
		durability: 6,
		mobility: -1,
		cost: 400,
		feature: 'Adaptive Camouflage'
	},
	{
		id: 'mod-armour-28',
		category: 'Defense',
		type: 'Armour',
		name: 'Overlapping Scale Plating',
		weight: 31,
		power: 0,
		storage: 0,
		durability: 13,
		mobility: -4,
		cost: 260
	},
	{
		id: 'mod-armour-29',
		category: 'Defense',
		type: 'Armour',
		name: 'Anti-Corrosion Coating',
		weight: 8,
		power: 0,
		storage: 0,
		durability: 4,
		mobility: 0,
		cost: 150
	},
	{
		id: 'mod-armour-30',
		category: 'Defense',
		type: 'Armour',
		name: 'Prototype Alloy Plating',
		weight: 40,
		power: 3,
		storage: 0,
		durability: 25,
		mobility: -5,
		cost: 650,
		feature: 'Experimental Alloy'
	},
	{
		id: 'mod-hover-01',
		category: 'Movement',
		type: 'Hover Drives',
		name: 'Hover Unit Mk.I',
		weight: 35,
		power: 8,
		storage: 0,
		durability: 0,
		mobility: 15,
		cost: 400
	},
	{
		id: 'mod-hover-02',
		category: 'Movement',
		type: 'Hover Drives',
		name: 'Hover Unit Mk.II',
		weight: 45,
		power: 12,
		storage: 0,
		durability: 0,
		mobility: 25,
		cost: 560,
		feature: 'Silent Running'
	},
	{
		id: 'mod-wheel-01',
		category: 'Movement',
		type: 'Wheels',
		name: 'All-Terrain Wheels',
		weight: 20,
		power: 0,
		storage: 0,
		durability: 5,
		mobility: 10,
		cost: 130
	},
	{
		id: 'mod-wheel-02',
		category: 'Movement',
		type: 'Wheels',
		name: 'Racing Wheels',
		weight: 12,
		power: 0,
		storage: 0,
		durability: -3,
		mobility: 18,
		cost: 190
	},
	{
		id: 'mod-track-01',
		category: 'Movement',
		type: 'Tracks',
		name: 'Standard Tracks',
		weight: 40,
		power: 0,
		storage: 0,
		durability: 5,
		mobility: 12,
		cost: 210
	},
	{
		id: 'mod-track-02',
		category: 'Movement',
		type: 'Tracks',
		name: 'Reinforced Tracks',
		weight: 55,
		power: 0,
		storage: 0,
		durability: 12,
		mobility: 15,
		cost: 300
	},
	{
		id: 'mod-thrust-01',
		category: 'Movement',
		type: 'Thrusters',
		name: 'Micro Thruster',
		weight: 18,
		power: 5,
		storage: 0,
		durability: 0,
		mobility: 10,
		cost: 240
	},
	{
		id: 'mod-thrust-02',
		category: 'Movement',
		type: 'Thrusters',
		name: 'Boost Thruster',
		weight: 28,
		power: 9,
		storage: 0,
		durability: 0,
		mobility: 20,
		cost: 380,
		feature: 'Emergency Boost'
	}
];

// Features the currently equipped robot already has installed.
export const robotFeatures = ['Headlights', 'Fuel Generator'];

// The 5 resource tiers, roughly matching the danger/rarity of the surface zone they're
// salvaged from — Scrap and Electrical from easy sites, Exotic only from the deepest ones.
export const siloResources = [
	{ name: 'Scrap', amount: 2450, dotClass: 'bg-slate-400' },
	{ name: 'Electrical', amount: 640, dotClass: 'bg-amber-400' },
	{ name: 'Mechanical', amount: 310, dotClass: 'bg-cyan-400' },
	{ name: 'Chemical', amount: 85, dotClass: 'bg-emerald-400' },
	{ name: 'Exotic', amount: 12, dotClass: 'bg-fuchsia-400' }
];

// Column order for the Silo Upgrades tree.
export const siloRooms = [
	'Robot Assembly Bays',
	'Research Labs',
	'Power Generation',
	'Storage Warehouses',
	'Manufacturing Facilities',
	'Radar Arrays',
	'Drone Hangars',
	'Teleporter Improvements'
];

// Overall silo level. Every row beyond this is locked outright, regardless of prerequisites.
export const siloLevel = 12;

export interface UpgradeNode {
	id: string;
	room: string; // must match an entry in siloRooms
	row: number; // 1-based, top to bottom; row 1 is the room's "unlock" node
	lane: 0 | 1 | 2; // horizontal slot within the room's column; row 1 is always lane 1 (centered)
	name: string;
	description: string;
	cost: string;
	requires: string[]; // ids of prerequisite nodes
	unlocked: boolean; // already installed
}

// A room can offer up to 3 parallel upgrade branches at a given row (lanes 0-2); row 1 always
// has a single centered node that unlocks the room. Rooms may skip rows entirely. Same-room,
// same-lane requirements may span multiple rows (a skip); any requirement that changes lane or
// room is always exactly one row back, so it only ever needs a single connector jog.
export const upgradeTree: UpgradeNode[] = [
	// Robot Assembly Bays
	{
		id: 'assembly-r1',
		room: 'Robot Assembly Bays',
		row: 1,
		lane: 1,
		name: 'Assembly Bay Unlock',
		description: 'Brings this facility online for the first time.',
		cost: '190 Scrap',
		requires: [],
		unlocked: true
	},
	{
		id: 'assembly-r2a',
		room: 'Robot Assembly Bays',
		row: 2,
		lane: 0,
		name: 'Assembly Bay II-A: Rapid',
		description: 'An alternate assembly bay upgrade path, tier 2.',
		cost: '280 + parts Scrap',
		requires: ['assembly-r1'],
		unlocked: true
	},
	{
		id: 'assembly-r2b',
		room: 'Robot Assembly Bays',
		row: 2,
		lane: 1,
		name: 'Assembly Bay II-B: Balanced',
		description: 'An alternate assembly bay upgrade path, tier 2.',
		cost: '280 + parts Scrap',
		requires: ['assembly-r1'],
		unlocked: false
	},
	{
		id: 'assembly-r2c',
		room: 'Robot Assembly Bays',
		row: 2,
		lane: 2,
		name: 'Assembly Bay II-C: Precision',
		description: 'An alternate assembly bay upgrade path, tier 2.',
		cost: '280 + parts Scrap',
		requires: ['assembly-r1'],
		unlocked: false
	},
	{
		id: 'assembly-r3',
		room: 'Robot Assembly Bays',
		row: 3,
		lane: 1,
		name: 'Assembly Bay III',
		description: 'Tier 3 upgrade for the assembly bay.',
		cost: '370 Scrap',
		requires: ['assembly-r2a'],
		unlocked: true
	},
	{
		id: 'assembly-r4',
		room: 'Robot Assembly Bays',
		row: 4,
		lane: 1,
		name: 'Assembly Bay IV',
		description: 'Tier 4 upgrade for the assembly bay.',
		cost: '460 Scrap',
		requires: ['assembly-r3'],
		unlocked: true
	},
	{
		id: 'assembly-r5',
		room: 'Robot Assembly Bays',
		row: 5,
		lane: 1,
		name: 'Assembly Bay V',
		description: 'Tier 5 upgrade for the assembly bay.',
		cost: '550 Scrap',
		requires: ['assembly-r4', 'research-r4'],
		unlocked: true
	},
	{
		id: 'assembly-r6',
		room: 'Robot Assembly Bays',
		row: 6,
		lane: 1,
		name: 'Assembly Bay VI',
		description: 'Tier 6 upgrade for the assembly bay.',
		cost: '640 Scrap',
		requires: ['assembly-r5'],
		unlocked: true
	},
	{
		id: 'assembly-r7',
		room: 'Robot Assembly Bays',
		row: 7,
		lane: 1,
		name: 'Assembly Bay VII',
		description: 'Tier 7 upgrade for the assembly bay.',
		cost: '730 Scrap',
		requires: ['assembly-r6'],
		unlocked: true
	},
	{
		id: 'assembly-r8',
		room: 'Robot Assembly Bays',
		row: 8,
		lane: 1,
		name: 'Assembly Bay VIII',
		description: 'Tier 8 upgrade for the assembly bay.',
		cost: '820 Scrap',
		requires: ['assembly-r7'],
		unlocked: true
	},
	{
		id: 'assembly-r9a',
		room: 'Robot Assembly Bays',
		row: 9,
		lane: 0,
		name: 'Assembly Bay IX-A: Rapid',
		description: 'An alternate assembly bay upgrade path, tier 9.',
		cost: '910 + parts Scrap',
		requires: ['assembly-r8'],
		unlocked: false
	},
	{
		id: 'assembly-r9b',
		room: 'Robot Assembly Bays',
		row: 9,
		lane: 2,
		name: 'Assembly Bay IX-B: Balanced',
		description: 'An alternate assembly bay upgrade path, tier 9.',
		cost: '910 + parts Scrap',
		requires: ['assembly-r8'],
		unlocked: false
	},
	{
		id: 'assembly-r10',
		room: 'Robot Assembly Bays',
		row: 10,
		lane: 1,
		name: 'Assembly Bay X',
		description: 'Tier 10 upgrade for the assembly bay.',
		cost: '1000 Scrap',
		requires: ['assembly-r9a'],
		unlocked: false
	},
	{
		id: 'assembly-r11',
		room: 'Robot Assembly Bays',
		row: 11,
		lane: 1,
		name: 'Assembly Bay XI',
		description: 'Tier 11 upgrade for the assembly bay.',
		cost: '1090 Scrap',
		requires: ['assembly-r10'],
		unlocked: false
	},
	{
		id: 'assembly-r12',
		room: 'Robot Assembly Bays',
		row: 12,
		lane: 1,
		name: 'Assembly Bay XII',
		description: 'Tier 12 upgrade for the assembly bay.',
		cost: '1180 Scrap',
		requires: ['assembly-r11'],
		unlocked: false
	},
	{
		id: 'assembly-r13',
		room: 'Robot Assembly Bays',
		row: 13,
		lane: 1,
		name: 'Assembly Bay XIII',
		description: 'Tier 13 upgrade for the assembly bay.',
		cost: '1270 Scrap',
		requires: ['assembly-r12'],
		unlocked: false
	},
	{
		id: 'assembly-r14',
		room: 'Robot Assembly Bays',
		row: 14,
		lane: 1,
		name: 'Assembly Bay XIV',
		description: 'Tier 14 upgrade for the assembly bay.',
		cost: '1360 Scrap',
		requires: ['assembly-r13'],
		unlocked: false
	},
	{
		id: 'assembly-r15',
		room: 'Robot Assembly Bays',
		row: 15,
		lane: 1,
		name: 'Assembly Bay XV',
		description: 'Tier 15 upgrade for the assembly bay.',
		cost: '1450 Scrap',
		requires: ['assembly-r14'],
		unlocked: false
	},
	{
		id: 'assembly-r16',
		room: 'Robot Assembly Bays',
		row: 16,
		lane: 1,
		name: 'Assembly Bay XVI',
		description: 'Tier 16 upgrade for the assembly bay.',
		cost: '1540 Scrap',
		requires: ['assembly-r15'],
		unlocked: false
	},
	{
		id: 'assembly-r17',
		room: 'Robot Assembly Bays',
		row: 17,
		lane: 1,
		name: 'Assembly Bay XVII',
		description: 'Tier 17 upgrade for the assembly bay.',
		cost: '1630 Scrap',
		requires: ['assembly-r16'],
		unlocked: false
	},
	{
		id: 'assembly-r18',
		room: 'Robot Assembly Bays',
		row: 18,
		lane: 1,
		name: 'Assembly Bay XVIII',
		description: 'Tier 18 upgrade for the assembly bay.',
		cost: '1720 Scrap',
		requires: ['assembly-r17', 'teleporter-r17'],
		unlocked: false
	},
	{
		id: 'assembly-r19',
		room: 'Robot Assembly Bays',
		row: 19,
		lane: 1,
		name: 'Assembly Bay XIX',
		description: 'Tier 19 upgrade for the assembly bay.',
		cost: '1810 Scrap',
		requires: ['assembly-r18'],
		unlocked: false
	},
	{
		id: 'assembly-r20',
		room: 'Robot Assembly Bays',
		row: 20,
		lane: 1,
		name: 'Assembly Bay XX',
		description: 'Tier 20 upgrade for the assembly bay.',
		cost: '1900 Scrap',
		requires: ['assembly-r19'],
		unlocked: false
	},
	// Research Labs
	{
		id: 'research-r1',
		room: 'Research Labs',
		row: 1,
		lane: 1,
		name: 'Research Lab Unlock',
		description: 'Brings this facility online for the first time.',
		cost: '190 Scrap',
		requires: [],
		unlocked: true
	},
	{
		id: 'research-r2',
		room: 'Research Labs',
		row: 2,
		lane: 1,
		name: 'Research Lab II',
		description: 'Tier 2 upgrade for the research lab.',
		cost: '280 Scrap',
		requires: ['research-r1'],
		unlocked: true
	},
	{
		id: 'research-r3',
		room: 'Research Labs',
		row: 3,
		lane: 1,
		name: 'Research Lab III',
		description: 'Tier 3 upgrade for the research lab.',
		cost: '370 Scrap',
		requires: ['research-r2'],
		unlocked: true
	},
	{
		id: 'research-r4',
		room: 'Research Labs',
		row: 4,
		lane: 1,
		name: 'Research Lab IV',
		description: 'Tier 4 upgrade for the research lab.',
		cost: '460 Scrap',
		requires: ['research-r3'],
		unlocked: true
	},
	{
		id: 'research-r6',
		room: 'Research Labs',
		row: 6,
		lane: 1,
		name: 'Research Lab VI',
		description: 'Tier 6 upgrade for the research lab.',
		cost: '640 Scrap',
		requires: ['research-r4'],
		unlocked: false
	},
	{
		id: 'research-r7',
		room: 'Research Labs',
		row: 7,
		lane: 1,
		name: 'Research Lab VII',
		description: 'Tier 7 upgrade for the research lab.',
		cost: '730 Scrap',
		requires: ['research-r6'],
		unlocked: false
	},
	{
		id: 'research-r8',
		room: 'Research Labs',
		row: 8,
		lane: 1,
		name: 'Research Lab VIII',
		description: 'Tier 8 upgrade for the research lab.',
		cost: '820 Scrap',
		requires: ['research-r7', 'power-r7'],
		unlocked: false
	},
	{
		id: 'research-r9',
		room: 'Research Labs',
		row: 9,
		lane: 1,
		name: 'Research Lab IX',
		description: 'Tier 9 upgrade for the research lab.',
		cost: '910 Scrap',
		requires: ['research-r8'],
		unlocked: false
	},
	{
		id: 'research-r10a',
		room: 'Research Labs',
		row: 10,
		lane: 0,
		name: 'Research Lab X-A: Rapid',
		description: 'An alternate research lab upgrade path, tier 10.',
		cost: '1000 + parts Scrap',
		requires: ['research-r9'],
		unlocked: false
	},
	{
		id: 'research-r10b',
		room: 'Research Labs',
		row: 10,
		lane: 2,
		name: 'Research Lab X-B: Balanced',
		description: 'An alternate research lab upgrade path, tier 10.',
		cost: '1000 + parts Scrap',
		requires: ['research-r9'],
		unlocked: false
	},
	{
		id: 'research-r11',
		room: 'Research Labs',
		row: 11,
		lane: 1,
		name: 'Research Lab XI',
		description: 'Tier 11 upgrade for the research lab.',
		cost: '1090 Scrap',
		requires: ['research-r10a'],
		unlocked: false
	},
	{
		id: 'research-r12',
		room: 'Research Labs',
		row: 12,
		lane: 1,
		name: 'Research Lab XII',
		description: 'Tier 12 upgrade for the research lab.',
		cost: '1180 Scrap',
		requires: ['research-r11'],
		unlocked: false
	},
	{
		id: 'research-r14',
		room: 'Research Labs',
		row: 14,
		lane: 1,
		name: 'Research Lab XIV',
		description: 'Tier 14 upgrade for the research lab.',
		cost: '1360 Scrap',
		requires: ['research-r12'],
		unlocked: false
	},
	{
		id: 'research-r15',
		room: 'Research Labs',
		row: 15,
		lane: 1,
		name: 'Research Lab XV',
		description: 'Tier 15 upgrade for the research lab.',
		cost: '1450 Scrap',
		requires: ['research-r14'],
		unlocked: false
	},
	{
		id: 'research-r16',
		room: 'Research Labs',
		row: 16,
		lane: 1,
		name: 'Research Lab XVI',
		description: 'Tier 16 upgrade for the research lab.',
		cost: '1540 Scrap',
		requires: ['research-r15', 'hangar-r15'],
		unlocked: false
	},
	{
		id: 'research-r17',
		room: 'Research Labs',
		row: 17,
		lane: 1,
		name: 'Research Lab XVII',
		description: 'Tier 17 upgrade for the research lab.',
		cost: '1630 Scrap',
		requires: ['research-r16'],
		unlocked: false
	},
	{
		id: 'research-r18',
		room: 'Research Labs',
		row: 18,
		lane: 1,
		name: 'Research Lab XVIII',
		description: 'Tier 18 upgrade for the research lab.',
		cost: '1720 Scrap',
		requires: ['research-r17'],
		unlocked: false
	},
	{
		id: 'research-r19',
		room: 'Research Labs',
		row: 19,
		lane: 1,
		name: 'Research Lab XIX',
		description: 'Tier 19 upgrade for the research lab.',
		cost: '1810 Scrap',
		requires: ['research-r18'],
		unlocked: false
	},
	{
		id: 'research-r20',
		room: 'Research Labs',
		row: 20,
		lane: 1,
		name: 'Research Lab XX',
		description: 'Tier 20 upgrade for the research lab.',
		cost: '1900 Scrap',
		requires: ['research-r19'],
		unlocked: false
	},
	// Power Generation
	{
		id: 'power-r1',
		room: 'Power Generation',
		row: 1,
		lane: 1,
		name: 'Reactor Unlock',
		description: 'Brings this facility online for the first time.',
		cost: '190 Scrap',
		requires: [],
		unlocked: true
	},
	{
		id: 'power-r2',
		room: 'Power Generation',
		row: 2,
		lane: 1,
		name: 'Reactor II',
		description: 'Tier 2 upgrade for the reactor.',
		cost: '280 Scrap',
		requires: ['power-r1'],
		unlocked: true
	},
	{
		id: 'power-r3',
		room: 'Power Generation',
		row: 3,
		lane: 1,
		name: 'Reactor III',
		description: 'Tier 3 upgrade for the reactor.',
		cost: '370 Scrap',
		requires: ['power-r2'],
		unlocked: true
	},
	{
		id: 'power-r4',
		room: 'Power Generation',
		row: 4,
		lane: 1,
		name: 'Reactor IV',
		description: 'Tier 4 upgrade for the reactor.',
		cost: '460 Scrap',
		requires: ['power-r3'],
		unlocked: true
	},
	{
		id: 'power-r5',
		room: 'Power Generation',
		row: 5,
		lane: 1,
		name: 'Reactor V',
		description: 'Tier 5 upgrade for the reactor.',
		cost: '550 Scrap',
		requires: ['power-r4'],
		unlocked: true
	},
	{
		id: 'power-r6',
		room: 'Power Generation',
		row: 6,
		lane: 1,
		name: 'Reactor VI',
		description: 'Tier 6 upgrade for the reactor.',
		cost: '640 Scrap',
		requires: ['power-r5', 'storage-r5'],
		unlocked: true
	},
	{
		id: 'power-r7',
		room: 'Power Generation',
		row: 7,
		lane: 1,
		name: 'Reactor VII',
		description: 'Tier 7 upgrade for the reactor.',
		cost: '730 Scrap',
		requires: ['power-r6'],
		unlocked: true
	},
	{
		id: 'power-r10',
		room: 'Power Generation',
		row: 10,
		lane: 1,
		name: 'Reactor X',
		description: 'Tier 10 upgrade for the reactor.',
		cost: '1000 Scrap',
		requires: ['power-r7'],
		unlocked: true
	},
	{
		id: 'power-r11',
		room: 'Power Generation',
		row: 11,
		lane: 1,
		name: 'Reactor XI',
		description: 'Tier 11 upgrade for the reactor.',
		cost: '1090 Scrap',
		requires: ['power-r10'],
		unlocked: false
	},
	{
		id: 'power-r12',
		room: 'Power Generation',
		row: 12,
		lane: 1,
		name: 'Reactor XII',
		description: 'Tier 12 upgrade for the reactor.',
		cost: '1180 Scrap',
		requires: ['power-r11'],
		unlocked: false
	},
	{
		id: 'power-r13',
		room: 'Power Generation',
		row: 13,
		lane: 1,
		name: 'Reactor XIII',
		description: 'Tier 13 upgrade for the reactor.',
		cost: '1270 Scrap',
		requires: ['power-r12'],
		unlocked: false
	},
	{
		id: 'power-r14',
		room: 'Power Generation',
		row: 14,
		lane: 1,
		name: 'Reactor XIV',
		description: 'Tier 14 upgrade for the reactor.',
		cost: '1360 Scrap',
		requires: ['power-r13'],
		unlocked: false
	},
	{
		id: 'power-r15a',
		room: 'Power Generation',
		row: 15,
		lane: 0,
		name: 'Reactor XV-A: Rapid',
		description: 'An alternate reactor upgrade path, tier 15.',
		cost: '1450 + parts Scrap',
		requires: ['power-r14'],
		unlocked: false
	},
	{
		id: 'power-r15b',
		room: 'Power Generation',
		row: 15,
		lane: 2,
		name: 'Reactor XV-B: Balanced',
		description: 'An alternate reactor upgrade path, tier 15.',
		cost: '1450 + parts Scrap',
		requires: ['power-r14'],
		unlocked: false
	},
	{
		id: 'power-r16',
		room: 'Power Generation',
		row: 16,
		lane: 1,
		name: 'Reactor XVI',
		description: 'Tier 16 upgrade for the reactor.',
		cost: '1540 Scrap',
		requires: ['power-r15a'],
		unlocked: false
	},
	{
		id: 'power-r17',
		room: 'Power Generation',
		row: 17,
		lane: 1,
		name: 'Reactor XVII',
		description: 'Tier 17 upgrade for the reactor.',
		cost: '1630 Scrap',
		requires: ['power-r16', 'teleporter-r16'],
		unlocked: false
	},
	{
		id: 'power-r18',
		room: 'Power Generation',
		row: 18,
		lane: 1,
		name: 'Reactor XVIII',
		description: 'Tier 18 upgrade for the reactor.',
		cost: '1720 Scrap',
		requires: ['power-r17'],
		unlocked: false
	},
	{
		id: 'power-r19',
		room: 'Power Generation',
		row: 19,
		lane: 1,
		name: 'Reactor XIX',
		description: 'Tier 19 upgrade for the reactor.',
		cost: '1810 Scrap',
		requires: ['power-r18'],
		unlocked: false
	},
	{
		id: 'power-r20',
		room: 'Power Generation',
		row: 20,
		lane: 1,
		name: 'Reactor XX',
		description: 'Tier 20 upgrade for the reactor.',
		cost: '1900 Scrap',
		requires: ['power-r19'],
		unlocked: false
	},
	// Storage Warehouses
	{
		id: 'storage-r1',
		room: 'Storage Warehouses',
		row: 1,
		lane: 1,
		name: 'Warehouse Unlock',
		description: 'Brings this facility online for the first time.',
		cost: '190 Scrap',
		requires: [],
		unlocked: true
	},
	{
		id: 'storage-r2',
		room: 'Storage Warehouses',
		row: 2,
		lane: 1,
		name: 'Warehouse II',
		description: 'Tier 2 upgrade for the warehouse.',
		cost: '280 Scrap',
		requires: ['storage-r1'],
		unlocked: true
	},
	{
		id: 'storage-r4',
		room: 'Storage Warehouses',
		row: 4,
		lane: 1,
		name: 'Warehouse IV',
		description: 'Tier 4 upgrade for the warehouse.',
		cost: '460 Scrap',
		requires: ['storage-r2'],
		unlocked: true
	},
	{
		id: 'storage-r5',
		room: 'Storage Warehouses',
		row: 5,
		lane: 1,
		name: 'Warehouse V',
		description: 'Tier 5 upgrade for the warehouse.',
		cost: '550 Scrap',
		requires: ['storage-r4'],
		unlocked: true
	},
	{
		id: 'storage-r6',
		room: 'Storage Warehouses',
		row: 6,
		lane: 1,
		name: 'Warehouse VI',
		description: 'Tier 6 upgrade for the warehouse.',
		cost: '640 Scrap',
		requires: ['storage-r5'],
		unlocked: true
	},
	{
		id: 'storage-r7',
		room: 'Storage Warehouses',
		row: 7,
		lane: 1,
		name: 'Warehouse VII',
		description: 'Tier 7 upgrade for the warehouse.',
		cost: '730 Scrap',
		requires: ['storage-r6'],
		unlocked: false
	},
	{
		id: 'storage-r8',
		room: 'Storage Warehouses',
		row: 8,
		lane: 1,
		name: 'Warehouse VIII',
		description: 'Tier 8 upgrade for the warehouse.',
		cost: '820 Scrap',
		requires: ['storage-r7'],
		unlocked: false
	},
	{
		id: 'storage-r9',
		room: 'Storage Warehouses',
		row: 9,
		lane: 1,
		name: 'Warehouse IX',
		description: 'Tier 9 upgrade for the warehouse.',
		cost: '910 Scrap',
		requires: ['storage-r8', 'radar-r8'],
		unlocked: false
	},
	{
		id: 'storage-r10',
		room: 'Storage Warehouses',
		row: 10,
		lane: 1,
		name: 'Warehouse X',
		description: 'Tier 10 upgrade for the warehouse.',
		cost: '1000 Scrap',
		requires: ['storage-r9'],
		unlocked: false
	},
	{
		id: 'storage-r11a',
		room: 'Storage Warehouses',
		row: 11,
		lane: 0,
		name: 'Warehouse XI-A: Rapid',
		description: 'An alternate warehouse upgrade path, tier 11.',
		cost: '1090 + parts Scrap',
		requires: ['storage-r10'],
		unlocked: false
	},
	{
		id: 'storage-r11b',
		room: 'Storage Warehouses',
		row: 11,
		lane: 2,
		name: 'Warehouse XI-B: Balanced',
		description: 'An alternate warehouse upgrade path, tier 11.',
		cost: '1090 + parts Scrap',
		requires: ['storage-r10'],
		unlocked: false
	},
	{
		id: 'storage-r12',
		room: 'Storage Warehouses',
		row: 12,
		lane: 1,
		name: 'Warehouse XII',
		description: 'Tier 12 upgrade for the warehouse.',
		cost: '1180 Scrap',
		requires: ['storage-r11a'],
		unlocked: false
	},
	{
		id: 'storage-r13',
		room: 'Storage Warehouses',
		row: 13,
		lane: 1,
		name: 'Warehouse XIII',
		description: 'Tier 13 upgrade for the warehouse.',
		cost: '1270 Scrap',
		requires: ['storage-r12'],
		unlocked: false
	},
	{
		id: 'storage-r14',
		room: 'Storage Warehouses',
		row: 14,
		lane: 1,
		name: 'Warehouse XIV',
		description: 'Tier 14 upgrade for the warehouse.',
		cost: '1360 Scrap',
		requires: ['storage-r13', 'manufacturing-r13'],
		unlocked: false
	},
	{
		id: 'storage-r15',
		room: 'Storage Warehouses',
		row: 15,
		lane: 1,
		name: 'Warehouse XV',
		description: 'Tier 15 upgrade for the warehouse.',
		cost: '1450 Scrap',
		requires: ['storage-r14'],
		unlocked: false
	},
	{
		id: 'storage-r16',
		room: 'Storage Warehouses',
		row: 16,
		lane: 1,
		name: 'Warehouse XVI',
		description: 'Tier 16 upgrade for the warehouse.',
		cost: '1540 Scrap',
		requires: ['storage-r15'],
		unlocked: false
	},
	{
		id: 'storage-r17',
		room: 'Storage Warehouses',
		row: 17,
		lane: 1,
		name: 'Warehouse XVII',
		description: 'Tier 17 upgrade for the warehouse.',
		cost: '1630 Scrap',
		requires: ['storage-r16'],
		unlocked: false
	},
	{
		id: 'storage-r18',
		room: 'Storage Warehouses',
		row: 18,
		lane: 1,
		name: 'Warehouse XVIII',
		description: 'Tier 18 upgrade for the warehouse.',
		cost: '1720 Scrap',
		requires: ['storage-r17'],
		unlocked: false
	},
	{
		id: 'storage-r19',
		room: 'Storage Warehouses',
		row: 19,
		lane: 1,
		name: 'Warehouse XIX',
		description: 'Tier 19 upgrade for the warehouse.',
		cost: '1810 Scrap',
		requires: ['storage-r18'],
		unlocked: false
	},
	{
		id: 'storage-r20',
		room: 'Storage Warehouses',
		row: 20,
		lane: 1,
		name: 'Warehouse XX',
		description: 'Tier 20 upgrade for the warehouse.',
		cost: '1900 Scrap',
		requires: ['storage-r19'],
		unlocked: false
	},
	// Manufacturing Facilities
	{
		id: 'manufacturing-r1',
		room: 'Manufacturing Facilities',
		row: 1,
		lane: 1,
		name: 'Manufacturing Unlock',
		description: 'Brings this facility online for the first time.',
		cost: '190 Scrap',
		requires: [],
		unlocked: true
	},
	{
		id: 'manufacturing-r2',
		room: 'Manufacturing Facilities',
		row: 2,
		lane: 1,
		name: 'Manufacturing II',
		description: 'Tier 2 upgrade for the manufacturing.',
		cost: '280 Scrap',
		requires: ['manufacturing-r1'],
		unlocked: true
	},
	{
		id: 'manufacturing-r3',
		room: 'Manufacturing Facilities',
		row: 3,
		lane: 1,
		name: 'Manufacturing III',
		description: 'Tier 3 upgrade for the manufacturing.',
		cost: '370 Scrap',
		requires: ['manufacturing-r2'],
		unlocked: false
	},
	{
		id: 'manufacturing-r4',
		room: 'Manufacturing Facilities',
		row: 4,
		lane: 1,
		name: 'Manufacturing IV',
		description: 'Tier 4 upgrade for the manufacturing.',
		cost: '460 Scrap',
		requires: ['manufacturing-r3'],
		unlocked: false
	},
	{
		id: 'manufacturing-r5',
		room: 'Manufacturing Facilities',
		row: 5,
		lane: 1,
		name: 'Manufacturing V',
		description: 'Tier 5 upgrade for the manufacturing.',
		cost: '550 Scrap',
		requires: ['manufacturing-r4'],
		unlocked: false
	},
	{
		id: 'manufacturing-r9',
		room: 'Manufacturing Facilities',
		row: 9,
		lane: 1,
		name: 'Manufacturing IX',
		description: 'Tier 9 upgrade for the manufacturing.',
		cost: '910 Scrap',
		requires: ['manufacturing-r5'],
		unlocked: false
	},
	{
		id: 'manufacturing-r10',
		room: 'Manufacturing Facilities',
		row: 10,
		lane: 1,
		name: 'Manufacturing X',
		description: 'Tier 10 upgrade for the manufacturing.',
		cost: '1000 Scrap',
		requires: ['manufacturing-r9'],
		unlocked: false
	},
	{
		id: 'manufacturing-r11',
		room: 'Manufacturing Facilities',
		row: 11,
		lane: 1,
		name: 'Manufacturing XI',
		description: 'Tier 11 upgrade for the manufacturing.',
		cost: '1090 Scrap',
		requires: ['manufacturing-r10', 'radar-r10'],
		unlocked: false
	},
	{
		id: 'manufacturing-r12',
		room: 'Manufacturing Facilities',
		row: 12,
		lane: 1,
		name: 'Manufacturing XII',
		description: 'Tier 12 upgrade for the manufacturing.',
		cost: '1180 Scrap',
		requires: ['manufacturing-r11'],
		unlocked: false
	},
	{
		id: 'manufacturing-r13',
		room: 'Manufacturing Facilities',
		row: 13,
		lane: 1,
		name: 'Manufacturing XIII',
		description: 'Tier 13 upgrade for the manufacturing.',
		cost: '1270 Scrap',
		requires: ['manufacturing-r12'],
		unlocked: false
	},
	{
		id: 'manufacturing-r14a',
		room: 'Manufacturing Facilities',
		row: 14,
		lane: 0,
		name: 'Manufacturing XIV-A: Rapid',
		description: 'An alternate manufacturing upgrade path, tier 14.',
		cost: '1360 + parts Scrap',
		requires: ['manufacturing-r13'],
		unlocked: false
	},
	{
		id: 'manufacturing-r14b',
		room: 'Manufacturing Facilities',
		row: 14,
		lane: 1,
		name: 'Manufacturing XIV-B: Balanced',
		description: 'An alternate manufacturing upgrade path, tier 14.',
		cost: '1360 + parts Scrap',
		requires: ['manufacturing-r13'],
		unlocked: false
	},
	{
		id: 'manufacturing-r14c',
		room: 'Manufacturing Facilities',
		row: 14,
		lane: 2,
		name: 'Manufacturing XIV-C: Precision',
		description: 'An alternate manufacturing upgrade path, tier 14.',
		cost: '1360 + parts Scrap',
		requires: ['manufacturing-r13'],
		unlocked: false
	},
	{
		id: 'manufacturing-r15',
		room: 'Manufacturing Facilities',
		row: 15,
		lane: 1,
		name: 'Manufacturing XV',
		description: 'Tier 15 upgrade for the manufacturing.',
		cost: '1450 Scrap',
		requires: ['manufacturing-r14a'],
		unlocked: false
	},
	{
		id: 'manufacturing-r16',
		room: 'Manufacturing Facilities',
		row: 16,
		lane: 1,
		name: 'Manufacturing XVI',
		description: 'Tier 16 upgrade for the manufacturing.',
		cost: '1540 Scrap',
		requires: ['manufacturing-r15'],
		unlocked: false
	},
	{
		id: 'manufacturing-r17',
		room: 'Manufacturing Facilities',
		row: 17,
		lane: 1,
		name: 'Manufacturing XVII',
		description: 'Tier 17 upgrade for the manufacturing.',
		cost: '1630 Scrap',
		requires: ['manufacturing-r16'],
		unlocked: false
	},
	{
		id: 'manufacturing-r18',
		room: 'Manufacturing Facilities',
		row: 18,
		lane: 1,
		name: 'Manufacturing XVIII',
		description: 'Tier 18 upgrade for the manufacturing.',
		cost: '1720 Scrap',
		requires: ['manufacturing-r17'],
		unlocked: false
	},
	{
		id: 'manufacturing-r19',
		room: 'Manufacturing Facilities',
		row: 19,
		lane: 1,
		name: 'Manufacturing XIX',
		description: 'Tier 19 upgrade for the manufacturing.',
		cost: '1810 Scrap',
		requires: ['manufacturing-r18'],
		unlocked: false
	},
	{
		id: 'manufacturing-r20',
		room: 'Manufacturing Facilities',
		row: 20,
		lane: 1,
		name: 'Manufacturing XX',
		description: 'Tier 20 upgrade for the manufacturing.',
		cost: '1900 Scrap',
		requires: ['manufacturing-r19'],
		unlocked: false
	},
	// Radar Arrays
	{
		id: 'radar-r1',
		room: 'Radar Arrays',
		row: 1,
		lane: 1,
		name: 'Radar Unlock',
		description: 'Brings this facility online for the first time.',
		cost: '190 Scrap',
		requires: [],
		unlocked: false
	},
	{
		id: 'radar-r2',
		room: 'Radar Arrays',
		row: 2,
		lane: 1,
		name: 'Radar II',
		description: 'Tier 2 upgrade for the radar.',
		cost: '280 Scrap',
		requires: ['radar-r1'],
		unlocked: false
	},
	{
		id: 'radar-r3',
		room: 'Radar Arrays',
		row: 3,
		lane: 1,
		name: 'Radar III',
		description: 'Tier 3 upgrade for the radar.',
		cost: '370 Scrap',
		requires: ['radar-r2'],
		unlocked: false
	},
	{
		id: 'radar-r5',
		room: 'Radar Arrays',
		row: 5,
		lane: 1,
		name: 'Radar V',
		description: 'Tier 5 upgrade for the radar.',
		cost: '550 Scrap',
		requires: ['radar-r3'],
		unlocked: false
	},
	{
		id: 'radar-r6',
		room: 'Radar Arrays',
		row: 6,
		lane: 1,
		name: 'Radar VI',
		description: 'Tier 6 upgrade for the radar.',
		cost: '640 Scrap',
		requires: ['radar-r5'],
		unlocked: false
	},
	{
		id: 'radar-r7',
		room: 'Radar Arrays',
		row: 7,
		lane: 1,
		name: 'Radar VII',
		description: 'Tier 7 upgrade for the radar.',
		cost: '730 Scrap',
		requires: ['radar-r6'],
		unlocked: false
	},
	{
		id: 'radar-r8',
		room: 'Radar Arrays',
		row: 8,
		lane: 1,
		name: 'Radar VIII',
		description: 'Tier 8 upgrade for the radar.',
		cost: '820 Scrap',
		requires: ['radar-r7'],
		unlocked: false
	},
	{
		id: 'radar-r9',
		room: 'Radar Arrays',
		row: 9,
		lane: 1,
		name: 'Radar IX',
		description: 'Tier 9 upgrade for the radar.',
		cost: '910 Scrap',
		requires: ['radar-r8', 'hangar-r8'],
		unlocked: false
	},
	{
		id: 'radar-r10',
		room: 'Radar Arrays',
		row: 10,
		lane: 1,
		name: 'Radar X',
		description: 'Tier 10 upgrade for the radar.',
		cost: '1000 Scrap',
		requires: ['radar-r9'],
		unlocked: false
	},
	{
		id: 'radar-r11',
		room: 'Radar Arrays',
		row: 11,
		lane: 1,
		name: 'Radar XI',
		description: 'Tier 11 upgrade for the radar.',
		cost: '1090 Scrap',
		requires: ['radar-r10'],
		unlocked: false
	},
	{
		id: 'radar-r12',
		room: 'Radar Arrays',
		row: 12,
		lane: 1,
		name: 'Radar XII',
		description: 'Tier 12 upgrade for the radar.',
		cost: '1180 Scrap',
		requires: ['radar-r11'],
		unlocked: false
	},
	{
		id: 'radar-r13',
		room: 'Radar Arrays',
		row: 13,
		lane: 1,
		name: 'Radar XIII',
		description: 'Tier 13 upgrade for the radar.',
		cost: '1270 Scrap',
		requires: ['radar-r12'],
		unlocked: false
	},
	{
		id: 'radar-r14',
		room: 'Radar Arrays',
		row: 14,
		lane: 1,
		name: 'Radar XIV',
		description: 'Tier 14 upgrade for the radar.',
		cost: '1360 Scrap',
		requires: ['radar-r13'],
		unlocked: false
	},
	{
		id: 'radar-r15',
		room: 'Radar Arrays',
		row: 15,
		lane: 1,
		name: 'Radar XV',
		description: 'Tier 15 upgrade for the radar.',
		cost: '1450 Scrap',
		requires: ['radar-r14'],
		unlocked: false
	},
	{
		id: 'radar-r16',
		room: 'Radar Arrays',
		row: 16,
		lane: 1,
		name: 'Radar XVI',
		description: 'Tier 16 upgrade for the radar.',
		cost: '1540 Scrap',
		requires: ['radar-r15'],
		unlocked: false
	},
	{
		id: 'radar-r18',
		room: 'Radar Arrays',
		row: 18,
		lane: 1,
		name: 'Radar XVIII',
		description: 'Tier 18 upgrade for the radar.',
		cost: '1720 Scrap',
		requires: ['radar-r16'],
		unlocked: false
	},
	{
		id: 'radar-r19',
		room: 'Radar Arrays',
		row: 19,
		lane: 1,
		name: 'Radar XIX',
		description: 'Tier 19 upgrade for the radar.',
		cost: '1810 Scrap',
		requires: ['radar-r18'],
		unlocked: false
	},
	{
		id: 'radar-r20',
		room: 'Radar Arrays',
		row: 20,
		lane: 1,
		name: 'Radar XX',
		description: 'Tier 20 upgrade for the radar.',
		cost: '1900 Scrap',
		requires: ['radar-r19'],
		unlocked: false
	},
	// Drone Hangars
	{
		id: 'hangar-r1',
		room: 'Drone Hangars',
		row: 1,
		lane: 1,
		name: 'Hangar Unlock',
		description: 'Brings this facility online for the first time.',
		cost: '190 Scrap',
		requires: [],
		unlocked: false
	},
	{
		id: 'hangar-r2',
		room: 'Drone Hangars',
		row: 2,
		lane: 1,
		name: 'Hangar II',
		description: 'Tier 2 upgrade for the hangar.',
		cost: '280 Scrap',
		requires: ['hangar-r1'],
		unlocked: false
	},
	{
		id: 'hangar-r3',
		room: 'Drone Hangars',
		row: 3,
		lane: 1,
		name: 'Hangar III',
		description: 'Tier 3 upgrade for the hangar.',
		cost: '370 Scrap',
		requires: ['hangar-r2'],
		unlocked: false
	},
	{
		id: 'hangar-r4',
		room: 'Drone Hangars',
		row: 4,
		lane: 1,
		name: 'Hangar IV',
		description: 'Tier 4 upgrade for the hangar.',
		cost: '460 Scrap',
		requires: ['hangar-r3'],
		unlocked: false
	},
	{
		id: 'hangar-r5a',
		room: 'Drone Hangars',
		row: 5,
		lane: 0,
		name: 'Hangar V-A: Rapid',
		description: 'An alternate hangar upgrade path, tier 5.',
		cost: '550 + parts Scrap',
		requires: ['hangar-r4'],
		unlocked: false
	},
	{
		id: 'hangar-r5b',
		room: 'Drone Hangars',
		row: 5,
		lane: 2,
		name: 'Hangar V-B: Balanced',
		description: 'An alternate hangar upgrade path, tier 5.',
		cost: '550 + parts Scrap',
		requires: ['hangar-r4'],
		unlocked: false
	},
	{
		id: 'hangar-r6',
		room: 'Drone Hangars',
		row: 6,
		lane: 1,
		name: 'Hangar VI',
		description: 'Tier 6 upgrade for the hangar.',
		cost: '640 Scrap',
		requires: ['hangar-r5a'],
		unlocked: false
	},
	{
		id: 'hangar-r7',
		room: 'Drone Hangars',
		row: 7,
		lane: 1,
		name: 'Hangar VII',
		description: 'Tier 7 upgrade for the hangar.',
		cost: '730 Scrap',
		requires: ['hangar-r6', 'research-r6'],
		unlocked: false
	},
	{
		id: 'hangar-r8',
		room: 'Drone Hangars',
		row: 8,
		lane: 1,
		name: 'Hangar VIII',
		description: 'Tier 8 upgrade for the hangar.',
		cost: '820 Scrap',
		requires: ['hangar-r7'],
		unlocked: false
	},
	{
		id: 'hangar-r9',
		room: 'Drone Hangars',
		row: 9,
		lane: 1,
		name: 'Hangar IX',
		description: 'Tier 9 upgrade for the hangar.',
		cost: '910 Scrap',
		requires: ['hangar-r8'],
		unlocked: false
	},
	{
		id: 'hangar-r13',
		room: 'Drone Hangars',
		row: 13,
		lane: 1,
		name: 'Hangar XIII',
		description: 'Tier 13 upgrade for the hangar.',
		cost: '1270 Scrap',
		requires: ['hangar-r9'],
		unlocked: false
	},
	{
		id: 'hangar-r14',
		room: 'Drone Hangars',
		row: 14,
		lane: 1,
		name: 'Hangar XIV',
		description: 'Tier 14 upgrade for the hangar.',
		cost: '1360 Scrap',
		requires: ['hangar-r13'],
		unlocked: false
	},
	{
		id: 'hangar-r15',
		room: 'Drone Hangars',
		row: 15,
		lane: 1,
		name: 'Hangar XV',
		description: 'Tier 15 upgrade for the hangar.',
		cost: '1450 Scrap',
		requires: ['hangar-r14', 'teleporter-r14'],
		unlocked: false
	},
	{
		id: 'hangar-r16',
		room: 'Drone Hangars',
		row: 16,
		lane: 1,
		name: 'Hangar XVI',
		description: 'Tier 16 upgrade for the hangar.',
		cost: '1540 Scrap',
		requires: ['hangar-r15'],
		unlocked: false
	},
	{
		id: 'hangar-r17',
		room: 'Drone Hangars',
		row: 17,
		lane: 1,
		name: 'Hangar XVII',
		description: 'Tier 17 upgrade for the hangar.',
		cost: '1630 Scrap',
		requires: ['hangar-r16'],
		unlocked: false
	},
	{
		id: 'hangar-r18',
		room: 'Drone Hangars',
		row: 18,
		lane: 1,
		name: 'Hangar XVIII',
		description: 'Tier 18 upgrade for the hangar.',
		cost: '1720 Scrap',
		requires: ['hangar-r17'],
		unlocked: false
	},
	{
		id: 'hangar-r19',
		room: 'Drone Hangars',
		row: 19,
		lane: 1,
		name: 'Hangar XIX',
		description: 'Tier 19 upgrade for the hangar.',
		cost: '1810 Scrap',
		requires: ['hangar-r18'],
		unlocked: false
	},
	{
		id: 'hangar-r20',
		room: 'Drone Hangars',
		row: 20,
		lane: 1,
		name: 'Hangar XX',
		description: 'Tier 20 upgrade for the hangar.',
		cost: '1900 Scrap',
		requires: ['hangar-r19'],
		unlocked: false
	},
	// Teleporter Improvements
	{
		id: 'teleporter-r1',
		room: 'Teleporter Improvements',
		row: 1,
		lane: 1,
		name: 'Teleporter Unlock',
		description: 'Brings this facility online for the first time.',
		cost: '190 Scrap',
		requires: [],
		unlocked: true
	},
	{
		id: 'teleporter-r3',
		room: 'Teleporter Improvements',
		row: 3,
		lane: 1,
		name: 'Teleporter III',
		description: 'Tier 3 upgrade for the teleporter.',
		cost: '370 Scrap',
		requires: ['teleporter-r1'],
		unlocked: true
	},
	{
		id: 'teleporter-r4',
		room: 'Teleporter Improvements',
		row: 4,
		lane: 1,
		name: 'Teleporter IV',
		description: 'Tier 4 upgrade for the teleporter.',
		cost: '460 Scrap',
		requires: ['teleporter-r3'],
		unlocked: true
	},
	{
		id: 'teleporter-r5',
		room: 'Teleporter Improvements',
		row: 5,
		lane: 1,
		name: 'Teleporter V',
		description: 'Tier 5 upgrade for the teleporter.',
		cost: '550 Scrap',
		requires: ['teleporter-r4'],
		unlocked: false
	},
	{
		id: 'teleporter-r6',
		room: 'Teleporter Improvements',
		row: 6,
		lane: 1,
		name: 'Teleporter VI',
		description: 'Tier 6 upgrade for the teleporter.',
		cost: '640 Scrap',
		requires: ['teleporter-r5', 'hangar-r5a'],
		unlocked: false
	},
	{
		id: 'teleporter-r7',
		room: 'Teleporter Improvements',
		row: 7,
		lane: 1,
		name: 'Teleporter VII',
		description: 'Tier 7 upgrade for the teleporter.',
		cost: '730 Scrap',
		requires: ['teleporter-r6'],
		unlocked: false
	},
	{
		id: 'teleporter-r8',
		room: 'Teleporter Improvements',
		row: 8,
		lane: 1,
		name: 'Teleporter VIII',
		description: 'Tier 8 upgrade for the teleporter.',
		cost: '820 Scrap',
		requires: ['teleporter-r7'],
		unlocked: false
	},
	{
		id: 'teleporter-r10',
		room: 'Teleporter Improvements',
		row: 10,
		lane: 1,
		name: 'Teleporter X',
		description: 'Tier 10 upgrade for the teleporter.',
		cost: '1000 Scrap',
		requires: ['teleporter-r8'],
		unlocked: false
	},
	{
		id: 'teleporter-r11',
		room: 'Teleporter Improvements',
		row: 11,
		lane: 1,
		name: 'Teleporter XI',
		description: 'Tier 11 upgrade for the teleporter.',
		cost: '1090 Scrap',
		requires: ['teleporter-r10'],
		unlocked: false
	},
	{
		id: 'teleporter-r12',
		room: 'Teleporter Improvements',
		row: 12,
		lane: 1,
		name: 'Teleporter XII',
		description: 'Tier 12 upgrade for the teleporter.',
		cost: '1180 Scrap',
		requires: ['teleporter-r11', 'assembly-r11'],
		unlocked: false
	},
	{
		id: 'teleporter-r13',
		room: 'Teleporter Improvements',
		row: 13,
		lane: 1,
		name: 'Teleporter XIII',
		description: 'Tier 13 upgrade for the teleporter.',
		cost: '1270 Scrap',
		requires: ['teleporter-r12'],
		unlocked: false
	},
	{
		id: 'teleporter-r14',
		room: 'Teleporter Improvements',
		row: 14,
		lane: 1,
		name: 'Teleporter XIV',
		description: 'Tier 14 upgrade for the teleporter.',
		cost: '1360 Scrap',
		requires: ['teleporter-r13'],
		unlocked: false
	},
	{
		id: 'teleporter-r15',
		room: 'Teleporter Improvements',
		row: 15,
		lane: 1,
		name: 'Teleporter XV',
		description: 'Tier 15 upgrade for the teleporter.',
		cost: '1450 Scrap',
		requires: ['teleporter-r14'],
		unlocked: false
	},
	{
		id: 'teleporter-r16',
		room: 'Teleporter Improvements',
		row: 16,
		lane: 1,
		name: 'Teleporter XVI',
		description: 'Tier 16 upgrade for the teleporter.',
		cost: '1540 Scrap',
		requires: ['teleporter-r15'],
		unlocked: false
	},
	{
		id: 'teleporter-r17',
		room: 'Teleporter Improvements',
		row: 17,
		lane: 1,
		name: 'Teleporter XVII',
		description: 'Tier 17 upgrade for the teleporter.',
		cost: '1630 Scrap',
		requires: ['teleporter-r16'],
		unlocked: false
	},
	{
		id: 'teleporter-r18',
		room: 'Teleporter Improvements',
		row: 18,
		lane: 1,
		name: 'Teleporter XVIII',
		description: 'Tier 18 upgrade for the teleporter.',
		cost: '1720 Scrap',
		requires: ['teleporter-r17'],
		unlocked: false
	},
	{
		id: 'teleporter-r19',
		room: 'Teleporter Improvements',
		row: 19,
		lane: 1,
		name: 'Teleporter XIX',
		description: 'Tier 19 upgrade for the teleporter.',
		cost: '1810 Scrap',
		requires: ['teleporter-r18'],
		unlocked: false
	},
	{
		id: 'teleporter-r20',
		room: 'Teleporter Improvements',
		row: 20,
		lane: 1,
		name: 'Teleporter XX',
		description: 'Tier 20 upgrade for the teleporter.',
		cost: '1900 Scrap',
		requires: ['teleporter-r19'],
		unlocked: false
	},
];


export const robotRoster = [
	{
		id: 'r-01',
		name: 'Scavenger Mk.I',
		equipped: true,
		health: 100,
		weight: '340 kg',
		power: '12 kW',
		storage: '18 slots'
	},
	{
		id: 'r-02',
		name: 'Prospector',
		equipped: false,
		health: 62,
		weight: '510 kg',
		power: '18 kW',
		storage: '12 slots'
	},
	{
		id: 'r-03',
		name: 'Longhauler',
		equipped: false,
		health: 88,
		weight: '680 kg',
		power: '22 kW',
		storage: '30 slots'
	},
	{
		id: 'r-04',
		name: 'Skiff',
		equipped: false,
		health: 45,
		weight: '210 kg',
		power: '9 kW',
		storage: '8 slots'
	},
	{
		id: 'r-05',
		name: 'Hauler-9',
		equipped: false,
		health: 100,
		weight: '760 kg',
		power: '26 kW',
		storage: '34 slots'
	}
];

export type MissionCategory = 'main' | 'side' | 'passive';

export interface Mission {
	id: string;
	category: MissionCategory;
	title: string;
	giver: string;
	reward: string;
	description: string;
	active: boolean;
}

// Main and side missions are mutually exclusive with each other (max 1 active across both).
// Passive missions can each be toggled independently.
export const missions: Mission[] = [
	{
		id: 'm-main-01',
		category: 'main',
		title: 'Recover the Prospector',
		giver: 'Beacon Command',
		reward: '+400 Scrap, Prospector Chassis',
		description:
			'The Prospector went dark near the Solar Farm three cycles ago. Recover its remains and salvage whatever cargo survived the fall.',
		active: true
	},
	{
		id: 'm-main-02',
		category: 'main',
		title: 'Silence the Scrapper Camp',
		giver: 'Beacon Command',
		reward: '+250 Scrap, Radar Array Blueprint',
		description:
			'Rival scavengers have set up camp near the Collapsed Highway. Disrupt their operation before they start raiding our routes.',
		active: false
	},
	{
		id: 'm-side-01',
		category: 'side',
		title: 'Scout the Solar Farm',
		giver: 'Research Labs',
		reward: '+120 Components',
		description:
			'Early scans show intact panels at the old solar farm. Confirm the site is safe to strip for parts.',
		active: false
	},
	{
		id: 'm-side-02',
		category: 'side',
		title: 'Map the Mining Facility',
		giver: 'Radar Team',
		reward: '+1 Radar Array Level',
		description:
			'Chart the collapsed tunnels beneath the mining facility to improve our long-range scans.',
		active: false
	},
	{
		id: 'm-passive-01',
		category: 'passive',
		title: 'Salvage 5 Scrap Caches',
		giver: 'Standing Order',
		reward: '+80 Scrap',
		description:
			'Standing bounty for any scrap caches recovered during an expedition, regardless of destination.',
		active: true
	},
	{
		id: 'm-passive-02',
		category: 'passive',
		title: 'Reach the Comms Tower',
		giver: 'Standing Order',
		reward: '+1 Colony Morale',
		description: 'The colony wants eyes on the old comms tower. Reaching it, even briefly, will do.',
		active: false
	},
	{
		id: 'm-passive-03',
		category: 'passive',
		title: 'Return with a Full Cargo Hold',
		giver: 'Standing Order',
		reward: '+150 Scrap',
		description: 'A bonus for expeditions that return with every cargo slot filled.',
		active: false
	}
];
