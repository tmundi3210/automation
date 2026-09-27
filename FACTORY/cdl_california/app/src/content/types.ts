// Content model shared by the build-time parser and the app.
export type TestId = 'GK' | 'CV';

export interface Lesson {
  id: string;            // "GK-07"
  num: number;
  test: TestId;
  title: string;
  handbook: string;      // "Section 2.6–2.6.8"
  pages: [string, string];
  weight: 'High' | 'Medium';
  minutes: number;
  objectives: string[];  // html
  recap: string[];       // html
  conceptIds: string[];
  numberIds: string[];
  trapIds: string[];
  tykIds: string[];
  flashIds: string[];
  itemIds: string[];     // authored practice test, in order
}

export interface Concept {
  id: string;            // "GK-07.c03"
  lesson: string;
  title: string;         // heading text without page cite
  section: string | null;// "2.6.1"
  pages: string[];
  ca: boolean;
  core: string[];        // html bullets: key points for the quick view
  coreExtra?: string;    // html: the key table or step list, when the bullets alone would miss it
  html: string;          // full rendered body (deep dive)
  hasBeyond: boolean;
  hasConflict: boolean;
}

export interface NumberFact {
  id: string; lesson: string; item: string; valueHtml: string; value: string; pages: string[]; ca: boolean; ku: string; concepts: string[];
}
export interface Trap {
  id: string; lesson: string; trap: string; correct: string; correctHtml: string; pages: string[]; ca: boolean; ku: string; concepts: string[];
}
export interface Tyk {
  id: string; lesson: string; box: string; n: number; q: string; aHtml: string; a: string; pages: string[]; ku: string; concepts: string[];
}
export interface Flashcard {
  id: string; lesson: string; n: number; q: string; a: string; ca: boolean; ku: string; concepts: string[];
}

export type Polarity = 'pos' | 'neg' | 'tf';
export type DistractorTag =
  | 'neighbor_number' | 'round_number' | 'sibling_rule_number' | 'alt_source' | 'trap_named' | 'plausible_generic' | 'unit_match' | 'true_statement';

export interface Item {
  id: string;
  lesson: string;
  test: TestId;
  origin: 'pack' | 'derived-number' | 'derived-trap';
  stem: string;          // html
  stemText: string;
  options: string[];     // html, authored order a,b,c
  optionsText: string[];
  key: number;           // 0..2
  explanation: string;   // html
  pages: string[];
  polarity: Polarity;
  numeric: boolean;
  tags: (DistractorTag | null)[]; // per option (null for key)
  notes?: (string | null)[];       // per option: why this wrong option is wrong (null for key)
  concepts: string[];
  ku: string;
  heldOut?: boolean;
  testContext?: TestId;
}

export interface GlossaryEntry { term: string; defHtml: string; lesson: string }

export interface Content {
  version: string;
  handbook: string;
  lessons: Lesson[];
  concepts: Record<string, Concept>;
  numbers: Record<string, NumberFact>;
  traps: Record<string, Trap>;
  tyk: Record<string, Tyk>;
  flash: Record<string, Flashcard>;
  items: Record<string, Item>;
  glossary: GlossaryEntry[];
  handbookWay: { topic: string; answer: string; elsewhere: string }[];
  mostMissed: { test: TestId; text: string; lesson: string }[];
  counts: Record<string, number>;
}
