// Courses, demo runs and test-attempt replays for SK-02 (DL 650 Section 12, pp. 12-1–12-4).
// Scoring facts used: encroachment = any part touches/crosses a line or cone, 1 error each; pull-up = stop AND pull forward
// (a stop without changing direction is not one; initial pull-ups not penalized, excessive ones are errors); looks max 2,
// straight line backing 1; Neutral + parking brake(s), face the vehicle, 3 points of contact, or possible automatic failure;
// final position exactly as instructed. Course sizes are illustrative (not to scale) except the 3 ft alley-dock rule.

export type Ex = 'straight' | 'offR' | 'offL' | 'parC' | 'parD' | 'alley';
export type Ev = 'enc' | 'pull' | 'stop' | 'look' | 'lookBad' | 'fwd';
/** kingpin x/y, trailer angle t, tractor angle c (deg). Angle 0 = cab pointing left (-x). */
export interface Pose { x: number; y: number; t: number; c: number }
export interface Frame { p: Pose; say: string; ev?: Ev; cone?: [number, number] }
export interface Final { ok: boolean; why: string }
export interface Run { ex: Ex; frames: Frame[]; final: Final }

type Base = 'straight' | 'off' | 'par' | 'alley';
export const EX: Record<Ex, { name: string; base: Base; mirror: boolean; looks: number; finish: string; page: string }> = {
  straight: { name: 'Straight line backing', base: 'straight', mirror: false, looks: 1, page: '12-2', finish: 'Back straight between the 2 rows of cones without touching or crossing them. Only 1 look allowed.' },
  offR: { name: 'Offset back/right', base: 'off', mirror: false, looks: 2, page: '12-2', finish: 'Drive toward the outer boundary, then back into the lane at your right rear until the front of the vehicle has passed the first set of cones.' },
  offL: { name: 'Offset back/left', base: 'off', mirror: true, looks: 2, page: '12-2', finish: 'Drive toward the outer boundary, then back into the lane at your left rear until the front of the vehicle has passed the first set of cones.' },
  parC: { name: 'Parallel park (conventional)', base: 'par', mirror: false, looks: 2, page: '12-2', finish: 'Space on your right. Drive past it, back in, and get the entire vehicle completely inside without crossing front, side or rear boundaries.' },
  parD: { name: 'Parallel park (driver side)', base: 'par', mirror: true, looks: 2, page: '12-2', finish: 'Space on your left. Drive past it, back in, and get the entire vehicle completely inside without crossing front, side or rear boundaries.' },
  alley: { name: 'Alley dock', base: 'alley', mirror: false, looks: 2, page: '12-2', finish: 'Drive past the alley, parallel to the outer boundary, then sight-side back in. Rear within 3 ft of the back of the alley, vehicle straight, no lines or cones touched.' },
};

const P = (x: number, y: number, t = 0, c = 0): Pose => ({ x, y, t, c });
const S0 = P(40, 50), O0 = P(140, 40), Q0 = P(45, 38), A0 = P(48, 24);

/** Clean demo runs for Explore (mirrored courses reuse the right-hand run). */
export const DEMO: Record<Base, Frame[]> = {
  straight: [
    { p: S0, say: 'Lined up in front of the lane. Look at the path, then check both mirrors.' },
    { p: P(90, 50), say: 'Backs slowly in the lowest reverse gear.' },
    { p: P(90, 50), ev: 'stop', say: 'Stops to check the mirrors, then keeps backing. No change of direction, so no pull-up.' },
    { p: P(140, 50), say: 'Backs straight to the end of the lane without touching a cone.' },
  ],
  off: [
    { p: O0, say: 'Starts in the left lane.' },
    { p: P(30, 40), ev: 'fwd', say: 'Drives straight forward toward the outer boundary (part of the exercise).' },
    { p: P(62, 37, -14, -4), say: 'Backs on a curve toward the neighboring lane.' },
    { p: P(95, 28, -8, -14), say: 'Trailer enters the other lane; the tractor follows.' },
    { p: P(140, 20), say: 'Stops once the front of the vehicle has passed the first set of cones.' },
  ],
  par: [
    { p: Q0, say: 'Drives past the space, parallel to it.' },
    { p: P(78, 34, -22, -6), say: 'Backs toward the space, trailer first.' },
    { p: P(100, 24, -8, -18), say: 'Trailer is in; the tractor swings in behind it.' },
    { p: P(112, 19), say: 'Straightens up with the entire vehicle inside the space.' },
  ],
  alley: [
    { p: A0, say: 'Drives past the alley, parallel to the outer boundary.' },
    { p: P(78, 30, 40, 12), say: 'Sight-side backs toward the alley (driver’s side, so you can see).' },
    { p: P(92, 40, 72, 45), say: 'Trailer enters the alley; turn the wheel to follow it.' },
    { p: P(100, 55, 90, 90), say: 'Stops straight in the alley with the rear 2 ft from the back line.' },
  ],
};

/** Test attempts the learner scores in the challenge. */
export const RUNS: Run[] = [
  { ex: 'straight', final: { ok: true, why: 'It ends at the end of the lane between the cones, as instructed.' }, frames: [
    { p: S0, say: 'Lined up in front of the lane.' },
    { p: P(80, 50), say: 'Backs slowly into the lane.' },
    { p: P(80, 50), ev: 'stop', say: 'Stops to check mirrors, then keeps backing.' },
    { p: P(100, 53, 6, 3), ev: 'enc', cone: [132, 60], say: 'Trailer drifts right; its rear corner touches a cone.' },
    { p: P(70, 50, 1, 0), ev: 'pull', say: 'Shifts to a forward gear and pulls ahead to straighten out.' },
    { p: P(140, 50), say: 'Backs straight to the end of the lane and stops.' },
  ] },
  { ex: 'straight', final: { ok: true, why: 'It ends at the end of the lane between the cones.' }, frames: [
    { p: S0, say: 'Lined up in front of the lane.' },
    { p: P(90, 50), say: 'Backs slowly.' },
    { p: P(90, 50), ev: 'look', say: 'Stops, puts it in Neutral, sets the parking brakes, climbs down facing the truck with 3 points of contact to look.' },
    { p: P(115, 50), say: 'Climbs back in and keeps backing.' },
    { p: P(115, 50), ev: 'look', say: 'Secures the truck again and opens the door to check the rear.' },
    { p: P(140, 50), say: 'Finishes straight at the end of the lane.' },
  ] },
  { ex: 'alley', final: { ok: false, why: 'The rear stopped 5 ft from the back of the alley. It must be within 3 ft (and straight).' }, frames: [
    { p: A0, say: 'Drives past the alley, parallel to the outer boundary.' },
    { p: P(78, 30, 40, 12), ev: 'enc', cone: [112, 45], say: 'Sight-side backs in; the trailer side touches the cone at the alley corner.' },
    { p: P(92, 40, 72, 45), say: 'Keeps backing into the alley.' },
    { p: P(92, 40, 72, 45), ev: 'look', say: 'Stops, Neutral, parking brakes set, climbs down facing the truck with 3 points of contact.' },
    { p: P(100, 53, 90, 90), say: 'Climbs back in, backs straight and stops with the rear 5 ft from the back line.' },
  ] },
  { ex: 'alley', final: { ok: true, why: 'Straight in the alley with the rear 2 ft from the back: within 3 ft.' }, frames: [
    { p: A0, say: 'Drives past the alley, parallel to the outer boundary.' },
    { p: P(80, 30, 36, 10), say: 'Starts backing; the angle is too shallow.' },
    { p: P(60, 26, 20, 4), ev: 'pull', say: 'Stops and pulls forward to get a better angle.' },
    { p: P(92, 40, 72, 45), say: 'Backs into the alley.' },
    { p: P(92, 40, 72, 45), ev: 'lookBad', say: 'Jumps out for a look — still in gear, parking brake not set.' },
    { p: P(100, 55, 90, 90), say: 'Backs in straight, rear 2 ft from the back line.' },
  ] },
  { ex: 'offR', final: { ok: false, why: 'The front of the tractor has not passed the first set of cones, so it is not in the final position.' }, frames: [
    { p: O0, say: 'Starts in the left lane.' },
    { p: P(30, 40), ev: 'fwd', say: 'Drives straight forward toward the outer boundary (part of the exercise).' },
    { p: P(62, 37, -14, -4), say: 'Backs toward the right-rear lane.' },
    { p: P(95, 28, -8, -14), ev: 'look', say: 'Stops, Neutral, parking brakes set, opens the door to check the right side.' },
    { p: P(128, 20), say: 'Backs in and stops — the front of the tractor is still short of the first cones.' },
  ] },
  { ex: 'parC', final: { ok: true, why: 'The entire vehicle ends completely inside the space.' }, frames: [
    { p: Q0, say: 'Drives past the space, parallel to it.' },
    { p: P(78, 34, -22, -6), say: 'Backs toward the space.' },
    { p: P(126, 20, 0, -4), ev: 'enc', cone: [160, 12], say: 'Backs too far: the trailer rear crosses the rear boundary.' },
    { p: P(112, 21, 0, -2), ev: 'pull', say: 'Pulls forward to clear the encroachment.' },
    { p: P(108, 21, 0, -8), ev: 'enc', cone: [90, 26], say: 'Straightening up, the cab’s mirror brushes the front cone.' },
    { p: P(112, 19), say: 'Settles with the whole rig inside the space.' },
  ] },
];

export interface Tally { enc: number; pull: number; looks: number; unsafe: boolean }
export function tally(frames: Frame[], upto = frames.length - 1): Tally {
  const f = frames.slice(0, upto + 1);
  return { enc: f.filter((x) => x.ev === 'enc').length, pull: f.filter((x) => x.ev === 'pull').length, looks: f.filter((x) => x.ev === 'look' || x.ev === 'lookBad').length, unsafe: f.some((x) => x.ev === 'lookBad') };
}
