/**
 * Bun build script for tauri-plugin-macos-permissions-api
 * Bundles TypeScript from guest-js/ to dist-js/ in both ESM and CommonJS formats
 */

import { $ } from "bun";
import { existsSync } from "fs";
import { mkdir } from "fs/promises";

const ENTRY = "./guest-js/index.ts";
const OUT_DIR = "./dist-js";
const EXTERNAL_DEPS = ["@tauri-apps/api"];

async function buildPlugin() {
	console.log("🔨 Building tauri-plugin-macos-permissions-api TypeScript bindings...\n");

	// Ensure output directory exists
	if (!existsSync(OUT_DIR)) {
		await mkdir(OUT_DIR, { recursive: true });
	}

	// Build ESM format
	console.log("📦 Building ESM format...");
	const esmResult = await Bun.build({
		entrypoints: [ENTRY],
		outdir: OUT_DIR,
		format: "esm",
		target: "node",
		external: EXTERNAL_DEPS,
		sourcemap: "linked",
		naming: {
			entry: "[name].js", // Output: index.js
		},
	});

	if (!esmResult.success) {
		console.error("❌ ESM build failed!");
		for (const log of esmResult.logs) {
			console.error(log);
		}
		process.exit(1);
	}

	console.log(`✅ ESM built: ${OUT_DIR}/index.js`);

	// Build CommonJS format
	console.log("\n📦 Building CommonJS format...");
	const cjsResult = await Bun.build({
		entrypoints: [ENTRY],
		outdir: OUT_DIR,
		format: "cjs",
		target: "node",
		external: EXTERNAL_DEPS,
		sourcemap: "linked",
		naming: {
			entry: "[name].cjs", // Output: index.cjs
		},
	});

	if (!cjsResult.success) {
		console.error("❌ CommonJS build failed!");
		for (const log of cjsResult.logs) {
			console.error(log);
		}
		process.exit(1);
	}

	console.log(`✅ CommonJS built: ${OUT_DIR}/index.cjs`);

	// Generate TypeScript declarations using tsconfig.json
	console.log("\n📝 Generating TypeScript declarations...");
	const tscResult = await $`bun tsc`.nothrow();

	if (tscResult.exitCode !== 0) {
		console.error("❌ TypeScript declaration generation failed!");
		console.error(tscResult.stderr.toString());
		process.exit(1);
	}

	console.log(`✅ Declarations generated: ${OUT_DIR}/index.d.ts`);

	console.log("\n🎉 Build successful!");
	console.log(`   ESM:  ${OUT_DIR}/index.js`);
	console.log(`   CJS:  ${OUT_DIR}/index.cjs`);
	console.log(`   DTS:  ${OUT_DIR}/index.d.ts`);
}

await buildPlugin();
