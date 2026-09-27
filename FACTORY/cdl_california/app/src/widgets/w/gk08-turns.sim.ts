// Geometry helper for gk08-turns: a tractor-semitrailer drawn as two bodies whose rear points
// follow the point ahead of them (tractrix). This reproduces off-tracking: rear wheels cut inside.
export type P = [number, number];
export type Seg = [P, P, P, P];
export interface Frame { f: P; h: P; r: P }

const bz = (s: Seg, t: number): P => {
  const u = 1 - t;
  const k = [u * u * u, 3 * u * u * t, 3 * u * t * t, t * t * t];
  return [k[0] * s[0][0] + k[1] * s[1][0] + k[2] * s[2][0] + k[3] * s[3][0], k[0] * s[0][1] + k[1] * s[1][1] + k[2] * s[2][1] + k[3] * s[3][1]];
};

/** Sample cubic segments into points about 2 units apart. */
export function sample(segs: Seg[]): P[] {
  const raw: P[] = [];
  for (const s of segs) for (let i = 0; i < 80; i++) raw.push(bz(s, i / 80));
  raw.push(segs[segs.length - 1][3]);
  const out: P[] = [raw[0]];
  for (const p of raw) { const q = out[out.length - 1]; if (Math.hypot(p[0] - q[0], p[1] - q[1]) >= 2) out.push(p); }
  return out;
}

/** f = front of tractor, h = hitch (kingpin), r = trailer rear axle. L1 tractor length, L2 trailer length. */
export function simulate(fp: P[], L1 = 30, L2 = 85): Frame[] {
  const d = Math.hypot(fp[1][0] - fp[0][0], fp[1][1] - fp[0][1]);
  const dx = (fp[1][0] - fp[0][0]) / d, dy = (fp[1][1] - fp[0][1]) / d;
  let h: P = [fp[0][0] - dx * L1, fp[0][1] - dy * L1];
  let r: P = [h[0] - dx * L2, h[1] - dy * L2];
  return fp.map((f) => {
    let e = Math.hypot(f[0] - h[0], f[1] - h[1]);
    h = [f[0] - (f[0] - h[0]) * L1 / e, f[1] - (f[1] - h[1]) * L1 / e];
    e = Math.hypot(h[0] - r[0], h[1] - r[1]);
    r = [h[0] - (h[0] - r[0]) * L2 / e, h[1] - (h[1] - r[1]) * L2 / e];
    return { f, h, r };
  });
}

export interface Box { x: number; y: number; w: number; h: number }
/** Does the body from a to b (half-width pad) touch box? */
export function bodyHits(a: P, b: P, bx: Box, pad = 8): boolean {
  for (let t = 0; t <= 1; t += 0.02) {
    const x = a[0] + (b[0] - a[0]) * t, y = a[1] + (b[1] - a[1]) * t;
    if (x > bx.x - pad && x < bx.x + bx.w + pad && y > bx.y - pad && y < bx.y + bx.h + pad) return true;
  }
  return false;
}
export const firstHit = (fr: Frame[], bx: Box, from = 0) => fr.findIndex((x, i) => i >= from && bodyHits(x.h, x.r, bx));

// ---- Scenes used by the widget (viewBox units; a lane is 40 wide) ----
const TAIL_E: Seg = [[300, 170], [380, 170], [460, 170], [540, 170]];
/** Right turn, north → east. Roads: N-S x 60–140 (NB lane 100–140), E-W y 110–190 (EB lane 150–190). Curb corner (140,190), radius 24. */
export const BUTTON = simulate(sample([[[120, 400], [120, 340], [120, 300], [120, 180]], [[120, 180], [120, 128], [180, 136], [210, 136]], [[210, 136], [250, 136], [260, 170], [300, 170]], TAIL_E]));
export const JUG = simulate(sample([[[120, 400], [120, 360], [70, 350], [70, 310]], [[70, 310], [70, 230], [80, 140], [170, 144]], [[170, 144], [230, 170], [260, 170], [300, 170]], TAIL_E]));
/** The car that slips into the gap on the right during the jug handle. */
export const CAR: Box = { x: 113, y: 218, w: 22, h: 40 };
export const JUG_OPEN = JUG.findIndex((x) => x.f[1] < 200 && !bodyHits(x.h, x.r, CAR, 4));
export const JUG_HIT = firstHit(JUG, CAR, JUG_OPEN + 1);
export const startIdx = (fr: Frame[]) => fr.findIndex((x) => x.f[1] <= 250);
export const endIdx = (fr: Frame[]) => fr.findIndex((x) => x.r[0] > 230);

/** Left turn, north → west, on a wider road: NB inside lane x 140–180, right-hand lane x 180–220; WB lanes y 40–120. */
const TAIL_W: Seg = [[-60, 60], [-140, 60], [-220, 60], [-300, 60]];
export function leftTurn(lane: 'inside' | 'right', start: 'soon' | 'center'): Frame[] {
  const x = lane === 'right' ? 200 : 160;
  const y0 = start === 'soon' ? 215 : 125;
  const lead: Seg[] = lane === 'inside'
    ? [[[x, 420], [x, 360], [x, 300], [x, y0 + 60]], [[x, y0 + 60], [x, y0 + 30], [x + 30, y0 + 20], [x + 30, y0]]]
    : [[[x, 420], [x, 360], [x, 300], [x, y0]]];
  const sx = lane === 'inside' ? x + 30 : x;
  return simulate(sample([...lead, [[sx, y0], [sx, y0 - 60], [sx - 40, 60], [sx - 90, 60]], [[sx - 90, 60], [sx - 140, 60], [-30, 60], [-60, 60]], TAIL_W]));
}
