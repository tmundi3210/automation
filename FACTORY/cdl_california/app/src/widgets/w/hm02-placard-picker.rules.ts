// Placard Tables 1 and 2 exactly as HM-02 states them (DL 650 pp. 9-9–9-11). No other source.

export interface Mat { id: string; cls: string; name: string; table: 1 | 2; placard: string | null; note?: string }

export const MATS: Mat[] = [
  // Table 1 — placard any amount (p. 9-10, Figure 9.7)
  { id: '1.1', cls: '1.1', name: '1.1 Mass explosives', table: 1, placard: 'Explosives 1.1' },
  { id: '1.2', cls: '1.2', name: '1.2 Projection hazards', table: 1, placard: 'Explosives 1.2' },
  { id: '1.3', cls: '1.3', name: '1.3 Mass fire hazards', table: 1, placard: 'Explosives 1.3' },
  { id: '2.3', cls: '2.3', name: '2.3 Poisonous/toxic gases', table: 1, placard: 'Poison Gas' },
  { id: '4.3', cls: '4.3', name: '4.3 Dangerous when wet', table: 1, placard: 'Dangerous When Wet' },
  { id: '5.2B', cls: '5.2', name: '5.2 Organic peroxide, Type B, temperature controlled', table: 1, placard: 'Organic Peroxide' },
  { id: '6.1AB', cls: '6.1', name: '6.1 Inhalation hazard, zone A or B', table: 1, placard: 'Poison Inhalation' },
  { id: '7Y3', cls: '7', name: '7 Radioactive, Yellow III label', table: 1, placard: 'Radioactive' },
  // Table 2 — 1,001 lb or more (p. 9-10, Figure 9.8)
  { id: '1.4', cls: '1.4', name: '1.4 Minor explosion', table: 2, placard: 'Explosives 1.4' },
  { id: '1.5', cls: '1.5', name: '1.5 Very insensitive', table: 2, placard: 'Explosives 1.5' },
  { id: '1.6', cls: '1.6', name: '1.6 Extremely insensitive', table: 2, placard: 'Explosives 1.6' },
  { id: '2.1', cls: '2.1', name: '2.1 Flammable gases', table: 2, placard: 'Flammable Gas' },
  { id: '2.2', cls: '2.2', name: '2.2 Non-flammable gases', table: 2, placard: 'Non-Flammable Gas' },
  { id: '3', cls: '3', name: '3 Flammable liquids', table: 2, placard: 'Flammable' },
  { id: 'comb', cls: '', name: 'Combustible liquid', table: 2, placard: 'Combustible', note: 'FLAMMABLE may be used instead on a cargo tank or portable tank.' },
  { id: '4.1', cls: '4.1', name: '4.1 Flammable solids', table: 2, placard: 'Flammable Solid' },
  { id: '4.2', cls: '4.2', name: '4.2 Spontaneously combustible', table: 2, placard: 'Spontaneously Combustible' },
  { id: '5.1', cls: '5.1', name: '5.1 Oxidizers', table: 2, placard: 'Oxidizer' },
  { id: '5.2', cls: '5.2', name: '5.2 Organic peroxide (not Type B temp. controlled)', table: 2, placard: 'Organic Peroxide' },
  { id: '6.1', cls: '6.1', name: '6.1 Poison (not inhalation zone A or B)', table: 2, placard: 'Poison' },
  { id: '6.2', cls: '6.2', name: '6.2 Infectious substances', table: 2, placard: null },
  { id: '8', cls: '8', name: '8 Corrosives', table: 2, placard: 'Corrosive' },
  { id: '9', cls: '9', name: '9 Miscellaneous HazMat', table: 2, placard: 'Class 9', note: 'Not required for domestic transportation.' },
  { id: 'ormd', cls: '', name: 'ORM-D', table: 2, placard: null },
];
export const mat = (id: string) => MATS.find((m) => m.id === id)!;

export interface Pkg { mat: string; lb: number; stop: 'A' | 'B'; bulk?: boolean; inh?: boolean; dww?: boolean }
export type Status = 'req' | 'no-placard' | 'not-req' | 'under';
export interface Line { placard: string; cls: string; status: Status; why: string }
export interface Result { lines: Line[]; total2: number; required: string[]; dangerous: null | { covers: string[]; keep: string[] } }

const lb = (n: number) => n.toLocaleString('en-US');

export function placardsFor(pkgs: Pkg[]): Result {
  const lines: Line[] = [];
  const add = (l: Line) => { const i = lines.findIndex((x) => x.placard === l.placard); if (i < 0) lines.push(l); else if (lines[i].status !== 'req' && l.status === 'req') lines[i] = l; };
  for (const p of pkgs) {
    const m = mat(p.mat);
    if (m.table === 1) add({ placard: m.placard!, cls: m.cls, status: 'req', why: `Table 1 (${m.name}): placard any amount.` });
    if (p.inh) add({ placard: 'Poison Inhalation', cls: '', status: 'req', why: 'INHALATION HAZARD on the paper or package: POISON GAS or POISON INHALATION placard in addition to class placards. The 1,000-pound exception does not apply.' });
    if (p.dww) add({ placard: 'Dangerous When Wet', cls: '4.3', status: 'req', why: 'Secondary hazard of dangerous when wet: DANGEROUS WHEN WET placard in addition to class placards. The 1,000-pound exception does not apply.' });
    if (m.table === 2 && p.bulk && m.placard && m.id !== '9') add({ placard: m.placard, cls: m.cls, status: 'req', why: `Bulk packaging (over 119 gallons): the 1,001-lb rule does not apply. Placard it, even with only residue.${m.note && m.id === 'comb' ? ' ' + m.note : ''}` });
  }
  const t2 = pkgs.filter((p) => mat(p.mat).table === 2 && !p.bulk);
  const total2 = t2.reduce((s, p) => s + p.lb, 0);
  const over = total2 >= 1001;
  for (const p of t2) {
    const m = mat(p.mat);
    if (!m.placard) add({ placard: `No placard (${m.name})`, cls: m.cls, status: 'no-placard', why: `Table 2 lists no placard for ${m.name}.` });
    else if (m.id === '9') add({ placard: m.placard, cls: m.cls, status: 'not-req', why: 'Class 9 placard is not required for domestic transportation.' });
    else if (over) add({ placard: m.placard, cls: m.cls, status: 'req', why: `Table 2 total on board is ${lb(total2)} lb (packages included), 1,001 or more.` });
    else add({ placard: m.placard, cls: m.cls, status: 'under', why: `Table 2 total on board is only ${lb(total2)} lb, under 1,001: no placard needed for it.` });
  }
  // DANGEROUS option: 1,001+ lb of 2 or more Table 2 classes needing different placards (p. 9-10)
  let dangerous: Result['dangerous'] = null;
  if (over) {
    const t2req = [...new Set(t2.map((p) => mat(p.mat)).filter((m) => m.placard && m.id !== '9').map((m) => m.placard!))];
    if (t2req.length >= 2) {
      const heavy = new Set<string>();
      const byClassStop = new Map<string, number>();
      for (const p of t2) { const m = mat(p.mat); if (!m.placard || m.id === '9') continue; const k = `${m.placard}|${p.stop}`; byClassStop.set(k, (byClassStop.get(k) ?? 0) + p.lb); }
      byClassStop.forEach((w, k) => { if (w >= 2205) heavy.add(k.split('|')[0]); });
      const bulkReq = new Set(lines.filter((l) => l.status === 'req' && pkgs.some((p) => p.bulk && mat(p.mat).placard === l.placard)).map((l) => l.placard));
      const covers = t2req.filter((pl) => !heavy.has(pl) && !bulkReq.has(pl));
      if (covers.length) dangerous = { covers, keep: t2req.filter((pl) => heavy.has(pl)) };
    }
  }
  const required = lines.filter((l) => l.status === 'req').map((l) => l.placard);
  return { lines, total2, required, dangerous };
}
