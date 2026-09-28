// Step data for cv03-coupling. Source: lesson CV-03 (handbook 6.4.1–6.4.2, pp. 6-9 to 6-11), with CV-02 cross-refs.

/** What the side-view picture shows after a step. */
export interface Rig {
  tractor: 'far' | 'front' | 'touch' | 'under' | 'partly' | 'clear';
  tilt: boolean; jaws: 'open' | 'locked';
  height: 'unchecked' | 'ok';          // trailer height before coupling
  gear: 'down' | 'slight' | 'up';
  air: boolean; cord: boolean; knob: 'in' | 'out'; engine: 'on' | 'off';
  chocks: boolean; parked: boolean; key: boolean; look: '' | 'coupling' | 'supports';
}
/** Visual consequence drawn when a wrong action is picked. */
export type Fx = 'roll' | 'low' | 'high' | 'gap' | 'angle' | 'snag' | 'drop' | 'torn' | 'kingpin' | 'nobrakes' | 'hurt' | 'level' | 'dirt' | 'order';
/** order = a real step picked at the wrong time; setup = a factual error (wrong position, height, gear, gap…); skip = leaves out a required check. */
export type Kind = 'order' | 'setup' | 'skip';
export interface Wrong { label: string; kind: Kind; fx: Fx; result: string }
export interface Step { n: number; label: string; detail: string; why: string; page: string; set: Partial<Rig>; wrong: [Wrong, Wrong] }

export const START_COUPLE: Rig = { tractor: 'far', tilt: false, jaws: 'open', height: 'unchecked', gear: 'down', air: false, cord: false, knob: 'out', engine: 'on', chocks: false, parked: false, key: true, look: '' };
export const START_UNCOUPLE: Rig = { tractor: 'under', tilt: false, jaws: 'locked', height: 'ok', gear: 'up', air: true, cord: true, knob: 'in', engine: 'on', chocks: false, parked: false, key: true, look: '' };

export const COUPLE: Step[] = [
  { n: 1, label: 'Inspect the fifth wheel', page: '6-9', set: { tilt: true, jaws: 'open' },
    detail: 'No damaged or missing parts, mounting tight, plate greased. Set for coupling: tilted down toward the rear, jaws open, safety unlocking handle on automatic lock, sliding fifth wheel locked. Kingpin not bent or broken.',
    why: 'The tilt lets the trailer plate ride up onto the fifth wheel; open jaws let the kingpin enter and lock. A dry plate can cause steering problems.',
    wrong: [
      { label: 'Position the tractor straight in front of the trailer', kind: 'order', fx: 'order', result: 'That is Step 3. The driving part feels like the start, but Step 1 is inspecting the fifth wheel — damaged parts, no grease or closed jaws would go unseen.' },
      { label: 'Set the fifth wheel level, jaws open', kind: 'setup', fx: 'level', result: 'The handbook says tilted down toward the rear of the tractor, not level — the tilt lets the trailer plate ride up onto it.' }] },
  { n: 2, label: 'Inspect the area and chock the wheels', page: '6-9', set: { chocks: true },
    detail: 'Nothing and nobody in the way. Trailer wheels chocked (or trailer spring brakes on). Cargo secured.',
    why: 'The trailer must not roll when the tractor pushes on it, and the bump of coupling must not shift the cargo.',
    wrong: [
      { label: 'Back slowly until the fifth wheel just touches', kind: 'order', fx: 'roll', result: 'No chocks, no check of the area: when the tractor pushes on the trailer, it can roll away.' },
      { label: 'Connect the air lines', kind: 'order', fx: 'order', result: 'Too early — that is Step 7. First check the area and chock, then position the tractor.' }] },
  { n: 3, label: 'Position the tractor straight in front', page: '6-9', set: { tractor: 'front' },
    detail: 'Line up straight in front of the trailer. Check your line in both outside mirrors, down both sides of the trailer.',
    why: 'Never back under at an angle — you could shove the trailer sideways and break the landing gear.',
    wrong: [
      { label: 'Crank the trailer up so the tractor slides under without touching', kind: 'setup', fx: 'high', result: 'Trap: too high and it may not couple correctly — the trailer should be low enough to be lifted slightly. And height is Step 6; first line up straight in front (3).' },
      { label: 'Back under the trailer in lowest reverse', kind: 'order', fx: 'order', result: 'That is Step 10. You have not touched, secured, checked height or hooked up air yet.' }] },
  { n: 4, label: 'Back slowly until the fifth wheel just touches', page: '6-9', set: { tractor: 'touch' },
    detail: 'Stop when the fifth wheel just touches the trailer. Don’t bump it.',
    why: 'You still must secure the tractor, check height and hook up air before going under.',
    wrong: [
      { label: 'Back all the way under now', kind: 'order', fx: 'order', result: 'Backing under before air hookup is a classic trap: Steps 7–9 (air, brake check, lock brakes) all come before Step 10.' },
      { label: 'Raise the landing gear', kind: 'order', fx: 'drop', result: 'Nothing is holding the trailer up yet except its landing gear — it would drop. Raising the gear is Step 15.' }] },
  { n: 5, label: 'Secure the tractor', page: '6-9', set: { parked: true },
    detail: 'Parking brakes on, transmission in neutral.',
    why: 'You are about to leave the cab.',
    wrong: [
      { label: 'Get out and check trailer height, brakes off', kind: 'setup', fx: 'roll', result: 'You are leaving the cab with the tractor unsecured — it can move. Parking brakes on, neutral, then get out.' },
      { label: 'Remove the wheel chocks', kind: 'order', fx: 'roll', result: 'Chocks come off last (Step 16). Without them the trailer can roll when the tractor pushes on it.' }] },
  { n: 6, label: 'Check trailer height', page: '6-9', set: { height: 'ok' },
    detail: 'Trailer low enough that the tractor lifts it slightly as it backs under. Crank it up or down. Kingpin and fifth wheel lined up.',
    why: 'Too low: the tractor may hit and damage the trailer nose. Too high: it may not couple correctly.',
    wrong: [
      { label: 'Crank it high so the tractor slides under without touching', kind: 'setup', fx: 'high', result: 'Trailer too high → it may not couple correctly. It should be low enough to be lifted slightly.' },
      { label: 'Crank it well below the fifth wheel', kind: 'setup', fx: 'low', result: 'Trailer too low → the tractor may hit and damage the trailer nose.' }] },
  { n: 7, label: 'Connect the air lines', page: '6-9', set: { air: true },
    detail: 'Check the glad hand seals. Tractor emergency to trailer emergency, then tractor service to trailer service. Support the lines so they won’t be pinched or snagged.',
    why: 'The trailer tanks fill through the emergency line, and you must test the trailer brakes before going under.',
    wrong: [
      { label: 'Back under the trailer', kind: 'order', fx: 'order', result: 'Backing under before air hookup is the trap: connect air (7), supply air and check lines (8), lock trailer brakes (9) — then back under (10).' },
      { label: 'Plug in the electrical cord', kind: 'order', fx: 'order', result: 'The electrical cord is Step 14, after the coupling is inspected. Air comes now, so you can test the trailer brakes first.' }] },
  { n: 8, label: 'Supply air; check for crossed lines', page: '6-9', set: { knob: 'in' },
    detail: 'Push in the air supply knob; wait for normal pressure. Engine off; apply and release the trailer brakes — hear the brakes move, then air escape. Watch for big air loss. Restart the engine.',
    why: 'With crossed lines the trailer tanks never fill; an old trailer with no spring brakes could be driven off with no trailer brakes.',
    wrong: [
      { label: 'Check the lines with the engine running', kind: 'setup', fx: 'nobrakes', result: 'Engine off, so you can HEAR the brakes move and the air escape. With the engine running you can miss crossed lines.' },
      { label: 'Pull out the knob and back under', kind: 'skip', fx: 'nobrakes', result: 'You skipped the crossed-line check. With crossed lines, supply air goes down the service line and the trailer tanks never fill.' }] },
  { n: 9, label: 'Lock the trailer brakes', page: '6-10', set: { knob: 'out' },
    detail: 'Pull out the air supply knob (older rigs: lever from “normal” to “emergency”).',
    why: 'Pushing the knob in released the trailer brakes. Setting them again keeps the trailer still while you back under and tug.',
    wrong: [
      { label: 'Leave the knob pushed in and back under', kind: 'setup', fx: 'roll', result: 'Knob in = trailer brakes released. The trailer can be shoved back as you back under it.' },
      { label: 'Wind the landing gear up slightly and tug', kind: 'order', fx: 'order', result: 'That is the tug test (Step 11). You have not coupled yet.' }] },
  { n: 10, label: 'Back under slowly in lowest reverse', page: '6-10', set: { tractor: 'under', jaws: 'locked', tilt: false },
    detail: 'Lowest reverse gear, slowly. Stop as soon as the kingpin locks into the fifth wheel.',
    why: 'So you don’t hit the kingpin too hard.',
    wrong: [
      { label: 'Back under briskly in a higher gear', kind: 'setup', fx: 'kingpin', result: 'You may hit the kingpin too hard. Lowest reverse, slowly.' },
      { label: 'Raise the landing gear all the way', kind: 'order', fx: 'order', result: 'Not yet — you have not tested the connection (11) or inspected it (13). Landing gear up is Step 15.' }] },
  { n: 11, label: 'Tug test', page: '6-10', set: { gear: 'slight' },
    detail: 'Wind the landing gear slightly off the ground. Gently pull forward with the trailer brakes still locked.',
    why: 'A trailer really locked on will hold. If not, you find out now at a crawl, not on the road.',
    wrong: [
      { label: 'Push the knob in and pull forward', kind: 'setup', fx: 'nobrakes', result: 'With the trailer brakes released, the trailer just rolls along behind you — the tug proves nothing.' },
      { label: 'Get under and inspect the coupling', kind: 'order', fx: 'order', result: 'That is Step 13. First test the connection with a gentle tug (11), then secure the vehicle and take the key (12) before you go under.' }] },
  { n: 12, label: 'Secure the vehicle; take the key', page: '6-10', set: { parked: true, engine: 'off', key: false },
    detail: 'Neutral, parking brakes on, engine off — and take the key with you.',
    why: 'So nobody can move the truck while you are under it.',
    wrong: [
      { label: 'Leave the engine running and crawl under', kind: 'setup', fx: 'hurt', result: 'Someone could move the truck while you are under it. Engine off, take the key.' },
      { label: 'Raise the landing gear', kind: 'order', fx: 'order', result: 'Step 15. First secure the vehicle and inspect the coupling.' }] },
  { n: 13, label: 'Inspect the coupling', page: '6-10', set: { look: 'coupling' },
    detail: 'Flashlight if needed. No space between upper and lower fifth wheel. Get under and look at the back of the fifth wheel: jaws closed around the kingpin shank. Locking lever in “lock”, safety latch over the lever.',
    why: 'A gap can mean the kingpin is resting on top of closed jaws — the trailer could easily come off. Anything wrong → don’t drive.',
    wrong: [
      { label: 'Accept a small gap between the plates', kind: 'setup', fx: 'gap', result: 'No space at all is allowed. A gap can mean the kingpin sits on top of closed jaws — the trailer could come off.' },
      { label: 'Skip looking — the tug test passed', kind: 'skip', fx: 'gap', result: 'A good tug test does not replace the look. Get under and look at the back of the fifth wheel.' }] },
  { n: 14, label: 'Connect electrical cord; check lines', page: '6-10', set: { cord: true, look: '' },
    detail: 'Plug in the electrical cord and close its safety catch. Check air and electrical lines for damage and that they stay clear of moving parts.',
    why: 'The cord must be secured, and every line undamaged and clear of moving parts.',
    wrong: [
      { label: 'Raise the landing gear', kind: 'order', fx: 'order', result: 'Out of order: the handbook does the cord and line check (14) before the landing gear (15).' },
      { label: 'Remove the wheel chocks', kind: 'order', fx: 'order', result: 'Chocks are the very last step (16).' }] },
  { n: 15, label: 'Raise the landing gear all the way', page: '6-10', set: { gear: 'up' },
    detail: 'Start in low gear range; switch to high range once free of weight. Wind it all the way up and secure the crank handle. Check clearance: tractor frame to landing gear, tractor tires to trailer nose.',
    why: 'Gear left partly down can snag on railroad tracks or other objects.',
    wrong: [
      { label: 'Raise it just off the pavement', kind: 'setup', fx: 'snag', result: 'Landing gear just off the pavement can snag on railroad tracks. All the way up, crank secured.' },
      { label: 'Remove the chocks and drive', kind: 'order', fx: 'snag', result: 'The landing gear is still down — raise it all the way first (Step 15), then remove the chocks (16).' }] },
  { n: 16, label: 'Remove and store the chocks', page: '6-10', set: { chocks: false },
    detail: 'Pull the wheel chocks and store them safely.',
    why: 'This is the last coupling step.',
    wrong: [
      { label: 'Pull forward gently to tug-test once more', kind: 'order', fx: 'order', result: 'The tug test was Step 11, with the landing gear only slightly off the ground. The last coupling step is removing and storing the chocks.' },
      { label: 'Plug in the electrical cord', kind: 'order', fx: 'order', result: 'Already done (Step 14). The last step is removing and storing the chocks.' }] },
];

export const UNCOUPLE: Step[] = [
  { n: 1, label: 'Position the rig', page: '6-11', set: {},
    detail: 'Ground strong enough for the trailer’s weight. Tractor in line with the trailer.',
    why: 'Soft ground lets the legs sink; pulling out at an angle can damage the landing gear.',
    wrong: [
      { label: 'Lower the landing gear', kind: 'order', fx: 'order', result: 'That is Step 4. First position the rig on firm ground in line with the trailer (1), ease pressure on the jaws (2) and chock if needed (3).' },
      { label: 'Unlock the fifth wheel', kind: 'order', fx: 'order', result: 'That is Step 6. The jaws are still loaded and the trailer brakes are not locked.' }] },
  { n: 2, label: 'Ease pressure on the locking jaws', page: '6-11', set: { knob: 'out', parked: true },
    detail: 'Pull the air supply knob to lock the trailer brakes. Back up gently to take the load off the jaws. Set the parking brakes while still pushing against the kingpin.',
    why: 'The locking lever then releases more easily.',
    wrong: [
      { label: 'Pull forward to free the jaws', kind: 'setup', fx: 'kingpin', result: 'Trap: pulling forward puts MORE pressure on the jaws. Back up gently, then set the brakes while pushing.' },
      { label: 'Pull the release handle right away', kind: 'order', fx: 'order', result: 'With load on the jaws the lever is hard to release. Lock trailer brakes, back up gently, park while pushing first.' }] },
  { n: 3, label: 'Chock the trailer wheels', page: '6-11', set: { chocks: true },
    detail: 'If the trailer has no spring brakes, or you are not sure.',
    why: 'Air can leak from the trailer tank; its emergency brakes then let go and the trailer could roll.',
    wrong: [
      { label: 'Skip chocks — the emergency brakes will hold', kind: 'skip', fx: 'roll', result: 'Air-only emergency brakes hold only while tank air lasts. When it leaks away the trailer can roll.' },
      { label: 'Disconnect the air lines', kind: 'order', fx: 'order', result: 'That is Step 5. Chock first (Step 3), then lower the landing gear (4).' }] },
  { n: 4, label: 'Lower the landing gear', page: '6-11', set: { gear: 'down' },
    detail: 'Empty trailer: until firmly on the ground. Loaded: after firm contact, a few extra turns in low gear — but don’t lift the trailer off the fifth wheel.',
    why: 'Takes weight off the tractor, so unlatching and next coupling are easier.',
    wrong: [
      { label: 'Crank until the trailer lifts off the fifth wheel', kind: 'setup', fx: 'high', result: 'A few extra turns only — never lift the trailer off the fifth wheel.' },
      { label: 'Unlock the fifth wheel', kind: 'order', fx: 'drop', result: 'The landing gear is still up. Unlock now and the trailer nose has nothing to stand on when you pull out.' }] },
  { n: 5, label: 'Disconnect air lines and electrical cable', page: '6-11', set: { air: false, cord: false },
    detail: 'Glad hands on the dummy couplers (or locked together). Cable hung plug-down. Lines supported.',
    why: 'Keeps dirt and water out; plug down keeps moisture out; supported lines are not damaged when the tractor moves.',
    wrong: [
      { label: 'Let the glad hands hang loose', kind: 'setup', fx: 'dirt', result: 'Dirt and water get into the couplers and air lines. Use the dummy couplers or lock them together.' },
      { label: 'Pull the tractor partly clear', kind: 'order', fx: 'torn', result: 'The lines are still connected — they would be torn apart.' }] },
  { n: 6, label: 'Unlock the fifth wheel', page: '6-11', set: { jaws: 'open' },
    detail: 'Lift the lock on the release handle; pull the handle to “open”. Keep legs and feet away from the rear tractor wheels.',
    why: 'You could be badly hurt if the rig moves.',
    wrong: [
      { label: 'Reach in beside the rear tractor wheels', kind: 'setup', fx: 'hurt', result: 'Keep legs and feet clear of the rear tractor wheels — you could be badly hurt if the rig moves.' },
      { label: 'Pull the tractor clear', kind: 'order', fx: 'order', result: 'The jaws are still locked on the kingpin. Unlock first.' }] },
  { n: 7, label: 'Pull partly clear; stop with frame under trailer', page: '6-11', set: { tractor: 'partly', parked: false },
    detail: 'Drive forward until the fifth wheel slides out from under the trailer. Stop with the tractor frame still under the trailer.',
    why: 'If the landing gear collapses or sinks, the trailer lands on the frame, not the ground.',
    wrong: [
      { label: 'Drive fully out in one go', kind: 'setup', fx: 'drop', result: 'Trap: if the landing gear collapses or sinks, the trailer falls to the ground. Stop with the frame still under it.' },
      { label: 'Inspect the trailer supports', kind: 'order', fx: 'order', result: 'That is Step 9. First pull partly clear and secure the tractor.' }] },
  { n: 8, label: 'Secure the tractor', page: '6-11', set: { parked: true },
    detail: 'Parking brake on, transmission in neutral.',
    why: 'You are getting out to inspect.',
    wrong: [
      { label: 'Pull the tractor clear', kind: 'order', fx: 'drop', result: 'You skipped securing and inspecting — a sinking landing gear would not be caught.' },
      { label: 'Get out without setting the brake', kind: 'setup', fx: 'roll', result: 'An unsecured tractor can roll while you are out. Parking brake on, neutral.' }] },
  { n: 9, label: 'Inspect the trailer supports', page: '6-11', set: { look: 'supports' },
    detail: 'Is the ground holding the trailer? Is the landing gear undamaged?',
    why: 'The frame is still under the trailer — this is your safe moment to check.',
    wrong: [
      { label: 'Pull clear without looking', kind: 'skip', fx: 'drop', result: 'If the ground is soft or the gear is damaged, the trailer drops once the frame leaves.' },
      { label: 'Remove the wheel chocks', kind: 'setup', fx: 'roll', result: 'Chock removal is the last COUPLING step. Here, a trailer without spring brakes could roll.' }] },
  { n: 10, label: 'Pull the tractor clear', page: '6-11', set: { tractor: 'clear', parked: false, look: '' },
    detail: 'Release the parking brakes, check the area around you, and drive forward until clear.',
    why: 'Check around first — people or objects may be near the rig.',
    wrong: [
      { label: 'Drive off without checking around', kind: 'skip', fx: 'hurt', result: 'Check the area before you move.' },
      { label: 'Keep the parking brakes set and pull clear', kind: 'setup', fx: 'order', result: 'Release the parking brakes first, check the area around you, then drive forward until clear.' }] },
];

/** Apply steps 0..k-1 to the start state. */
export function rigAfter(start: Rig, seq: Step[], k: number): Rig {
  let r = { ...start };
  for (let i = 0; i < k && i < seq.length; i++) r = { ...r, ...seq[i].set };
  return r;
}
