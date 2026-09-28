// Widget contract. Each widget is one file in ./w/*.tsx exporting `meta` + default component.
// Files are auto-registered, so widgets can be added in parallel without touching shared code.
import type { ComponentType } from 'preact';

export interface WidgetEvidence { concepts: string[]; ok: boolean }
export interface WidgetProps {
  /** Report an untimed check result (counts toward mastery at half weight). Explore/timed interactions must NOT call this. */
  onEvidence: (e: WidgetEvidence) => void;
  /** Call once when the learner completes the widget's challenge without error (earns the topic stamp). */
  onChallenge?: () => void;
  /** Concept ids of the lesson the widget is embedded in (for evidence). */
  concepts: string[];
  reducedMotion: boolean;
}
export interface WidgetMeta {
  id: string;                 // kebab id, unique
  title: string;              // shown in the widget header
  lesson: string;             // "GK-07"
  anchor: RegExp;             // matched against concept titles; first match hosts the widget (else lesson top)
  summary: string;            // one line: what the learner does
  stamp?: { id: string; name: string; rule: string };
}
interface Mod { meta: WidgetMeta; default: ComponentType<WidgetProps> }

const mods = import.meta.glob(['./w/*.tsx', '!./w/*.*.tsx'], { eager: true }) as Record<string, Mod>;
export const WIDGETS: Mod[] = Object.values(mods).filter((m) => m && m.meta && m.default).sort((a, b) => a.meta.id.localeCompare(b.meta.id));

export function widgetsForLesson(lesson: string): Mod[] { return WIDGETS.filter((w) => w.meta.lesson === lesson); }
export function widgetById(id: string): Mod | undefined { return WIDGETS.find((w) => w.meta.id === id); }
