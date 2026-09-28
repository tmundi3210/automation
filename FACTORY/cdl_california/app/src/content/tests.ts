// The CA CDL tests this app teaches. Question counts/pass marks: DMV touchscreen format (research/CA_RULES.md §2a).
import type { TestId } from './types';

export type Endo = 'T' | 'N' | 'P' | 'S' | 'H';

export interface TestInfo {
  id: TestId; name: string; short: string;
  /** written knowledge test format; null = no written test (skills lessons) */
  n: number | null; pass: number | null;
  sections: string;
  endo?: Endo;
  blurb: string;
}

export const TESTS: Record<TestId, TestInfo> = {
  GK: { id: 'GK', name: 'General Knowledge', short: 'General', n: 50, pass: 40, sections: '1–3', blurb: 'Everyone takes it: rules, inspection, safe driving, cargo.' },
  CV: { id: 'CV', name: 'Combination Vehicles', short: 'Combination', n: 20, pass: 16, sections: '6', blurb: 'Class A: pulling a trailer, trailer air brakes, coupling.' },
  AB: { id: 'AB', name: 'Air Brakes', short: 'Air Brakes', n: 25, pass: 20, sections: '5', blurb: 'Needed if your vehicle has air brakes (without it you get an "L" restriction).' },
  DT: { id: 'DT', name: 'Doubles and Triples', short: 'Doubles', n: 20, pass: 16, sections: '7', endo: 'T', blurb: 'T endorsement: pulling 2 trailers.' },
  TK: { id: 'TK', name: 'Tank Vehicles', short: 'Tank', n: 20, pass: 16, sections: '8', endo: 'N', blurb: 'N endorsement: liquids or gases in tanks.' },
  PV: { id: 'PV', name: 'Passenger Transport', short: 'Passenger', n: 20, pass: 16, sections: '4', endo: 'P', blurb: 'P endorsement: buses and vehicles for more than 10 people.' },
  SB: { id: 'SB', name: 'School Bus', short: 'School Bus', n: 20, pass: 16, sections: '10', endo: 'S', blurb: 'S endorsement (also needs P): driving a school bus.' },
  HM: { id: 'HM', name: 'Hazardous Materials', short: 'HazMat', n: 30, pass: 24, sections: '9', endo: 'H', blurb: 'H endorsement: placarded hazardous materials (age 21+, TSA check).' },
  SK: { id: 'SK', name: 'Skills Tests', short: 'Skills', n: null, pass: null, sections: '11–13', blurb: 'The 3 driving tests after your permit: inspection, basic control, road test.' },
};

export const TEST_ORDER: TestId[] = ['GK', 'CV', 'AB', 'DT', 'TK', 'PV', 'SB', 'HM', 'SK'];
export const ENDOS: { e: Endo; test: TestId; label: string }[] = [
  { e: 'T', test: 'DT', label: 'T · Doubles/Triples' },
  { e: 'N', test: 'TK', label: 'N · Tank' },
  { e: 'P', test: 'PV', label: 'P · Passenger' },
  { e: 'S', test: 'SB', label: 'S · School bus' },
  { e: 'H', test: 'HM', label: 'H · Hazardous materials' },
];

export const isWritten = (t: TestId) => TESTS[t].n !== null;

export interface TestChoice { cls: 'A' | 'B' | 'C' | null; airBrakesPassed: boolean; noAirBrakes?: boolean; endorsements?: Endo[]; skills?: boolean }

/** Tests (in study order) for a class + choices. S needs P; Class C always needs at least one of P/S/H/N (shown in onboarding). */
export function testsFor(p: TestChoice): TestId[] {
  const out = new Set<TestId>(['GK']);
  if ((p.cls ?? 'A') === 'A') out.add('CV');
  if (!p.airBrakesPassed && !p.noAirBrakes) out.add('AB');
  const en = new Set(p.endorsements ?? []);
  if (en.has('S')) en.add('P');
  for (const x of ENDOS) if (en.has(x.e)) out.add(x.test);
  if (p.skills !== false) out.add('SK');
  return TEST_ORDER.filter((t) => out.has(t));
}
