// Parking / attending / no-flares / no-smoking checks exactly as HM-04 states them (DL 650 pp. 9-16, 9-18).

export type Load = 'exp' | 'fl';
export type Stop = 'park' | 'brief';
export type Att = 'cab' | 'sleeper' | 'near' | 'far' | 'other' | 'none';
export type Dev = 'none' | 'tri' | 'red' | 'flare';
export type Smoke = 'none' | 's10' | 's40';
export type Prop = 'public' | 'private' | 'shipper' | 'haven';
export type Feat = 'bridge' | 'tunnel' | 'building' | 'crowd' | 'fire';
export interface Spot { id: string; n: number; name: string; x: number; y: number; road: number; d: Record<Feat, number>; prop: Prop; propName: string }
export interface State { load: Load; spot: string; stop: Stop; att: Att; dev: Dev; smoke: Smoke }

export const FEAT_NAME: Record<Feat, string> = { bridge: 'bridge', tunnel: 'tunnel', building: 'building', crowd: 'place where people gather', fire: 'open fire' };
/** Distances are the scenario's, in feet (the map is not to scale). */
export const SPOTS: Spot[] = [
  { id: 'shoulder', n: 1, name: 'Highway shoulder', x: 196, y: 75, road: 3, d: { bridge: 650, tunnel: 600, building: 400, crowd: 420, fire: 500 }, prop: 'public', propName: 'public road shoulder' },
  { id: 'bridge', n: 2, name: 'Wide shoulder by the bridge', x: 92, y: 88, road: 12, d: { bridge: 150, tunnel: 1100, building: 500, crowd: 520, fire: 900 }, prop: 'public', propName: 'public road shoulder' },
  { id: 'diner', n: 3, name: 'Diner parking lot', x: 118, y: 132, road: 120, d: { bridge: 480, tunnel: 800, building: 50, crowd: 60, fire: 700 }, prop: 'private', propName: 'private lot (owner not told of the danger)' },
  { id: 'fuel', n: 4, name: 'Fuel island', x: 250, y: 132, road: 90, d: { bridge: 900, tunnel: 450, building: 60, crowd: 350, fire: 400 }, prop: 'private', propName: 'fuel station' },
  { id: 'pullout', n: 5, name: 'Gravel pullout near a brush fire', x: 256, y: 96, road: 40, d: { bridge: 1000, tunnel: 450, building: 380, crowd: 500, fire: 200 }, prop: 'public', propName: 'public pullout' },
  { id: 'turnout', n: 6, name: 'Roadside turnout', x: 190, y: 106, road: 60, d: { bridge: 600, tunnel: 700, building: 350, crowd: 360, fire: 450 }, prop: 'public', propName: 'public turnout' },
  { id: 'shipper', n: 7, name: "Shipper's yard", x: 114, y: 222, road: 400, d: { bridge: 700, tunnel: 1200, building: 350, crowd: 900, fire: 1000 }, prop: 'shipper', propName: "shipper's property" },
  { id: 'haven', n: 8, name: 'Safe haven', x: 300, y: 222, road: 300, d: { bridge: 1300, tunnel: 500, building: 400, crowd: 800, fire: 600 }, prop: 'haven', propName: 'safe haven approved by local authorities' },
];
export const spot = (id: string) => SPOTS.find((s) => s.id === id)!;

export const ATT: Record<Att, string> = { cab: 'You, awake in the driver’s seat', sleeper: 'You, in the sleeper berth', near: 'You, 60 ft away, truck in clear view', far: 'You, inside a store 150 ft away', other: 'A co-worker, 50 ft away, clear view', none: 'Nobody' };
export const DEV: Record<Dev, string> = { none: 'Not broken down', tri: 'Reflective triangles', red: 'Red electric lights', flare: 'Flares (fusees)' };
export const SMOKE: Record<Smoke, string> = { none: 'No one smoking', s10: 'Someone smoking 10 ft away', s40: 'Someone smoking 40 ft away' };
export const LOAD: Record<Load, string> = { exp: 'Division 1.1, 1.2 or 1.3 explosives', fl: 'Class 3 flammable liquid cargo tank' };

export type Key = 'road' | 'r300' | 'private' | 'attend' | 'flare' | 'smoke';
export interface Check { key: Key; ok: boolean | null; title: string; text: string; page: string }

const validAtt = (a: Att) => a === 'cab' || a === 'near' || a === 'other';
const whyAtt: Partial<Record<Att, string>> = {
  sleeper: 'The sleeper berth does not count: the attendant must be in the vehicle awake and not in the sleeper, or within 100 ft with it in clear view.',
  far: '150 ft is beyond 100 ft, and the truck is out of clear view.',
  none: 'Someone must always watch it.',
};

export function check(s: State): Check[] {
  const sp = spot(s.spot), exp = s.load === 'exp', out: Check[] = [];
  // 5 ft of the traveled road
  if (exp) out.push({ key: 'road', ok: sp.road >= 5, page: '9-16', title: `5 ft from the traveled road (${sp.road} ft)`, text: sp.road >= 5 ? 'More than 5 ft from the traveled part of the road.' : 'Never park Division 1.1, 1.2 or 1.3 explosives within 5 ft of the traveled part of the road — no exception, not even brief.' });
  else out.push({ key: 'road', ok: sp.road >= 5 || s.stop === 'brief', page: '9-16', title: `5 ft from the traveled road (${sp.road} ft)`, text: sp.road >= 5 ? 'More than 5 ft from the traveled part of the road.' : s.stop === 'brief' ? 'Allowed within 5 ft only because your work requires it, and only briefly.' : 'A placarded vehicle may park within 5 ft only if your work requires it, and only briefly.' });
  // 300 ft
  const near = (Object.keys(sp.d) as Feat[]).filter((f) => sp.d[f] < 300 && (exp || f === 'fire'));
  const list = near.map((f) => `${FEAT_NAME[f]} (${sp.d[f]} ft)`).join(', ');
  if (exp) out.push({ key: 'r300', ok: !near.length || s.stop === 'brief', page: '9-16', title: '300 ft from bridge, tunnel, building, crowd, open fire',
    text: !near.length ? 'Nothing listed is within 300 ft.' : s.stop === 'brief' ? `Within 300 ft of ${list}, but a short stop needed to operate the vehicle (like fueling) is the exception.` : `Within 300 ft of ${list}. An explosion there would multiply the harm.` });
  else out.push({ key: 'r300', ok: !near.length, page: '9-16', title: '300 ft from an open fire', text: !near.length ? 'No open fire within 300 ft. (The bridge/tunnel/building/crowd 300 ft rule is for Division 1.1–1.3 explosives.)' : `Open fire only ${sp.d.fire} ft away. Don't park a placarded vehicle within 300 ft of an open fire.` });
  // private property (explosives, parking)
  if (exp && s.stop === 'park' && sp.prop === 'private') out.push({ key: 'private', ok: false, page: '9-16', title: 'Private property', text: 'Don’t park on private property unless the owner knows the danger.' });
  // attending
  if (exp) {
    let ok = validAtt(s.att), text = ok ? 'Attended: in the vehicle awake, or within 100 ft with clear view.' : whyAtt[s.att]!;
    if (sp.prop === 'haven') { ok = true; text = 'A safe haven is the one place you may leave an explosives load unattended.'; }
    else if (s.att === 'other' && sp.prop !== 'shipper') { ok = false; text = 'Someone else may watch it only on the shipper’s, carrier’s or consignee’s property. Otherwise use a safe haven.'; }
    else if (s.att === 'other') text = 'On the shipper’s property, someone else may watch it for you (within 100 ft, clear view).';
    out.push({ key: 'attend', ok, page: '9-16', title: `Attended (${ATT[s.att]})`, text });
  } else {
    const onRoad = sp.prop === 'public';
    out.push({ key: 'attend', ok: onRoad ? validAtt(s.att) : null, page: '9-16', title: `Attended (${ATT[s.att]})`,
      text: !onRoad ? 'Not on a public road or shoulder, so this watch rule does not apply here.' : validAtt(s.att) ? 'Watched on a public road or shoulder: in the vehicle awake, or within 100 ft with clear view.' : `On a public road or shoulder someone must always watch it. ${whyAtt[s.att]}` });
  }
  // flares
  out.push({ key: 'flare', ok: s.dev === 'none' ? null : s.dev !== 'flare', page: '9-16', title: `Warning devices (${DEV[s.dev]})`,
    text: s.dev === 'none' ? 'Not broken down: no warning devices set out.' : s.dev === 'flare' ? `Never use burning signals near ${exp ? 'Division 1.1–1.3 explosives' : 'a Class 3 tank, loaded or empty — the vapor can ignite'}. Use reflective triangles or red electric lights.` : 'Reflective triangles or red electric lights: the right choice.' });
  // smoking
  out.push({ key: 'smoke', ok: s.smoke === 'none' ? null : s.smoke === 's40', page: '9-18', title: `No smoking within 25 ft (${SMOKE[s.smoke]})`,
    text: s.smoke === 'none' ? 'No one is smoking nearby.' : s.smoke === 's40' ? '40 ft is beyond 25 ft.' : `No smoking or lighted cigarette, cigar or pipe within 25 ft of a vehicle with ${exp ? 'Class 1 explosives' : 'Class 3 flammable liquids'}.` });
  return out;
}
export const broken = (s: State) => check(s).filter((c) => c.ok === false).map((c) => c.key);
