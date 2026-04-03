// 2D Canvas
export const BOX_WIDTH = 10;
export const BOX_MARGIN = 2;
export const TEXT_HEIGHT = 15;
export const CANVAS_MARGIN = 20;
export const HEADER_HEIGHT = 60;
export const BOX_BORDER_RADIUS = 2;
export const WEEKS_IN_YEAR = 53;
export const DAYS_IN_WEEK = 7;
export const FONT_SIZE = 10;
export const FONT_FAMILY = "'SF Mono', ui-monospace, Menlo, Monaco, 'Cascadia Mono', 'Segoe UI Mono', monospace";

// 3D Isometric (obelisk.js)
export const CUBE_SIZE = 16;
export const MAX_CUBE_HEIGHT = 100;
export const MIN_CUBE_HEIGHT = 3;
export const ISO_ORIGIN = { x: 130, y: 90 };
export const CUBE_GAP = 2;
export const ISO_CANVAS_WIDTH = 1000;
export const ISO_CANVAS_HEIGHT = 600;

// Labels
export const DAY_LABELS_SHORT = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
export const MONTH_LABELS_SHORT = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

// Source configuration
export const SOURCE_DISPLAY_NAMES: Record<string, string> = {
  opencode: "OpenCode",
  claude: "Claude Code",
  codex: "Codex CLI",
  gemini: "Gemini CLI",
  cursor: "Cursor",
  amp: "Amp",
  droid: "Droid",
  openclaw: "OpenClaw",
  hermes: "Hermes Agent",
  pi: "Pi",
  kimi: "Kimi",
  qwen: "Qwen",
  roocode: "Roo Code",
  kilocode: "Kilo",
  kilo: "Kilo",
  mux: "Mux",
  synthetic: "Synthetic",
};

const SOURCE_LOGO_FILES: Record<string, string> = {
  opencode: "opencode.png",
  claude: "claude.jpg",
  codex: "openai.jpg",
  gemini: "gemini.png",
  cursor: "cursor.jpg",
  amp: "amp.png",
  droid: "droid.png",
  openclaw: "openclaw.jpg",
  hermes: "hermes.png",
  pi: "pi.png",
  kimi: "kimi.png",
  qwen: "qwen.png",
  roocode: "roocode.png",
  kilocode: "kilocode.png",
  kilo: "kilocode.png",
  mux: "mux.png",
  synthetic: "synthetic.png",
};

// Client logos from GitHub CDN (public repo)
const GITHUB_CDN_BASE = "https://raw.githubusercontent.com/junhoyeo/tokscale/main/.github/assets";
export const SOURCE_LOGOS: Record<string, string> = Object.fromEntries(
  Object.entries(SOURCE_LOGO_FILES).map(([client, fileName]) => [
    client,
    `${GITHUB_CDN_BASE}/client-${fileName}`,
  ]),
);

// Client logos served by the local frontend public assets
export const LOCAL_SOURCE_LOGOS: Record<string, string> = Object.fromEntries(
  Object.entries(SOURCE_LOGO_FILES).map(([client, fileName]) => [
    client,
    `/assets/logos/${fileName}`,
  ]),
);

export const SOURCE_COLORS: Record<string, string> = {
  opencode: "#00A8E8",
  claude: "#f97316",
  codex: "#3b82f6",
  gemini: "#8b5cf6",
  cursor: "#22c55e",
  amp: "#EC4899",
  droid: "#1F1D1C",
  openclaw: "#EF4444",
  hermes: "#FFD700",
  pi: "#6366F1",
  kimi: "#8B5CF6",
  qwen: "#1A73E8",
  roocode: "#10B981",
  kilocode: "#F59E0B",
  kilo: "#F59E0B",
  mux: "#171717",
  synthetic: "#4ADE80",
};

export const SOURCE_TEXT_COLORS: Record<string, string> = {
  droid: "#FFFFFF",
};

// Derived values
export const CELL_SIZE = BOX_WIDTH + BOX_MARGIN;

export const calculateCanvasWidth = (weeks: number = WEEKS_IN_YEAR): number =>
  CANVAS_MARGIN * 2 + TEXT_HEIGHT + weeks * CELL_SIZE;

export const calculateCanvasHeight = (): number =>
  HEADER_HEIGHT + DAYS_IN_WEEK * CELL_SIZE + CANVAS_MARGIN;

// Interaction timing
export const TOOLTIP_DELAY = 100;
export const THEME_TRANSITION_DURATION = 200;
export const INTERACTION_DEBOUNCE = 16;
