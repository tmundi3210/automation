// Part data for cv00-rig-anatomy. Every definition is taken from lessons CV-01..CV-04 (tires: GK-04, which CV-04 points back to).
export type PartId =
  | 'fifth' | 'jaws' | 'kingpin' | 'upper' | 'glad' | 'red' | 'blue' | 'cord' | 'tpv'
  | 'tanks' | 'relay' | 'landing' | 'spring' | 'abs' | 'dolly' | 'tires';
export type View = 'rig' | 'close';

export interface Part {
  id: PartId; name: string; view: View;
  /** hotspot badge (bx,by) and the point it points at (tx,ty), in the view's 600x340 viewBox */
  b: [number, number]; t: [number, number];
  def: string; check: string; pages: string[];
}

export const PARTS: Part[] = [
  { id: 'fifth', name: 'Fifth wheel (lower)', view: 'close', b: [560, 305], t: [528, 213],
    def: 'The round coupling plate on the back of the tractor. The trailer’s upper plate rests on it.',
    check: 'Firmly mounted to the frame, no missing or damaged parts, enough grease (a dry plate can cause steering problems). Ready to couple = tilted down toward the rear of the tractor, jaws open.',
    pages: ['6-9', '6-16'] },
  { id: 'jaws', name: 'Locking jaws & release handle', view: 'close', b: [362, 305], t: [436, 219],
    def: 'The parts inside the fifth wheel that close on the kingpin. The locking lever (release handle) opens them; a safety latch keeps that lever from opening by accident.',
    check: 'After coupling, look at the back of the fifth wheel: jaws closed around the kingpin’s shank — never its head — lever in “lock”, safety latch over the lever.',
    pages: ['6-10', '6-16'] },
  { id: 'kingpin', name: 'Kingpin', view: 'close', b: [450, 108], t: [450, 216],
    def: 'The steel pin pointing down from the trailer’s upper plate. From top to bottom: base, shank, head.',
    check: 'Not bent, broken or damaged. The fifth-wheel jaws must lock around its shank.',
    pages: ['6-9', '6-16'] },
  { id: 'upper', name: 'Upper fifth wheel (glide plate)', view: 'close', b: [560, 108], t: [566, 195],
    def: 'The flat plate under the front of the trailer. It sits on the tractor’s fifth wheel.',
    check: 'Glide plate firmly mounted to the trailer frame. No gap between the upper and lower fifth wheel — a gap can mean the kingpin is resting on top of closed jaws.',
    pages: ['6-10', '6-16'] },
  { id: 'glad', name: 'Glad hands', view: 'close', b: [362, 52], t: [292, 116],
    def: 'Couplers that join the tractor’s service and emergency air lines to the trailer’s lines. Each has a rubber seal.',
    check: 'Clean them and look for cracked seals. Connect at a right angle (90°), push the seals together and turn. No trailer? Park them on the dummy couplers to keep out water and dirt.',
    pages: ['6-6'] },
  { id: 'red', name: 'Emergency (supply) line — red', view: 'close', b: [256, 30], t: [262, 136],
    def: 'The red line. It has two jobs: fill the trailer air tanks, and control the trailer emergency brakes.',
    check: 'If it loses pressure (hose torn, trailer breaks loose), the trailer emergency brakes come on and the tractor protection valve closes — the knob pops out.',
    pages: ['6-5'] },
  { id: 'blue', name: 'Service (control) line — blue', view: 'close', b: [240, 305], t: [190, 190],
    def: 'The blue line, also called the control or signal line. It carries air whose pressure you set with the foot brake or trailer hand valve, and it tells the relay valves how much air to send to the brakes.',
    check: 'A major leak may go unnoticed until you brake; then tank pressure falls fast, and if it falls far enough the trailer emergency brakes come on.',
    pages: ['6-5', '6-7'] },
  { id: 'cord', name: 'Electrical cord', view: 'close', b: [176, 30], t: [206, 112],
    def: 'The cable that plugs the tractor into the trailer’s electrical socket.',
    check: 'Pushed firmly into its socket, safety catch closed, no damage, clear of moving parts. When you unhook it, hang it plug-down to keep moisture out.',
    pages: ['6-10', '6-11', '6-16'] },
  { id: 'tpv', name: 'Tractor protection valve', view: 'close', b: [60, 305], t: [163, 241],
    def: 'Keeps air in the tractor if the trailer breaks away or leaks badly. You work it from the cab with the red, 8-sided trailer air supply knob.',
    check: 'It closes by itself when pressure drops to 20–45 psi: no air leaves the tractor, and the trailer emergency brakes come on. Test: engine off, pump the brake pedal until the knob pops out.',
    pages: ['6-5', '6-17'] },
  { id: 'tanks', name: 'Trailer air tanks', view: 'rig', b: [436, 318], t: [438, 241],
    def: 'Tanks on the trailer (at least one on every trailer and converter dolly) that hold the air that works the trailer brakes.',
    check: 'They fill through the emergency (supply) line — not the service line. Drain every tank every day: water and oil can stop the brakes working right.',
    pages: ['6-6'] },
  { id: 'relay', name: 'Relay valve', view: 'rig', b: [470, 150], t: [476, 236],
    def: 'A valve on the trailer that sends air from the trailer’s own tank to its brakes.',
    check: 'Service-line pressure tells it how much air to send. Relay valves let the trailer brakes come on faster than they otherwise could.',
    pages: ['6-5', '6-6'] },
  { id: 'landing', name: 'Landing gear & crank', view: 'rig', b: [382, 150], t: [372, 258],
    def: 'The two front legs that hold the trailer up when no tractor is under it. You wind them up and down with a crank handle.',
    check: 'For driving: raised all the way up and the crank handle secured. Gear left partly down can snag on railroad tracks or other objects.',
    pages: ['6-10', '6-16'] },
  { id: 'spring', name: 'Spring brakes (brake chambers)', view: 'rig', b: [556, 150], t: [529, 243],
    def: 'Brakes held on by strong springs and released by air. On a trailer they work as the parking brakes.',
    check: 'Not required on converter dollies or trailers built before 1975. Those have air-only emergency brakes and no parking brake — always chock their wheels when you park.',
    pages: ['6-7'] },
  { id: 'abs', name: 'Trailer ABS lamp', view: 'rig', b: [226, 150], t: [213, 208],
    def: 'The yellow ABS malfunction lamp on the left side of the trailer, at the front or rear corner.',
    check: 'Trailers and dollies built on or after March 1, 1998 must have ABS. If ABS fails you still have your regular brakes — get it serviced soon.',
    pages: ['6-8'] },
  { id: 'dolly', name: 'Converter dolly', view: 'rig', b: [190, 40], t: [112, 70],
    def: 'A short unit on wheels used to hook up the next trailer: trailer → dolly → second trailer.',
    check: 'Carries at least one air tank. Spring brakes are not required on dollies. Dollies built on or after March 1, 1998 need ABS and a lamp on the left side.',
    pages: ['6-6', '6-7', '6-8'] },
  { id: 'tires', name: 'Tires & mud flaps', view: 'rig', b: [572, 318], t: [566, 284],
    def: 'A combination has more tires, wheels, lights and reflectors to check. Mud flaps are the splash guards behind the wheels.',
    check: 'Tread at least 4/32 inch in every major groove on front tires, 2/32 inch on all others. Splash guards fastened, not dragging or rubbing the tires.',
    pages: ['6-16', '2-2', '2-7'] },
];
export const PART = Object.fromEntries(PARTS.map((p) => [p.id, p])) as Record<PartId, Part>;

/** "Name that part": 8 fixed rounds. opts[0] is always the answer; display order rotates by round. */
export const ROUNDS: { id: PartId; wrong: [PartId, PartId] }[] = [
  { id: 'kingpin', wrong: ['jaws', 'upper'] },
  { id: 'red', wrong: ['blue', 'cord'] },
  { id: 'landing', wrong: ['spring', 'dolly'] },
  { id: 'tpv', wrong: ['relay', 'glad'] },
  { id: 'blue', wrong: ['red', 'cord'] },
  { id: 'relay', wrong: ['tpv', 'tanks'] },
  { id: 'abs', wrong: ['cord', 'glad'] },
  { id: 'jaws', wrong: ['fifth', 'kingpin'] },
];
