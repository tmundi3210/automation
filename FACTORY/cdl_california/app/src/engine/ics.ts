// Study plan → iCalendar (.ics) so the learner's own calendar app can remind them. PWA only.
import type { Content } from '../content/types';
import type { Plan } from './plan';

const esc = (s: string) => s.replace(/[\;,]/g, (m) => '\\' + m).replace(/\n/g, '\\n');
export function planToIcs(plan: Plan, c: Content, stamp: number): string {
  const dt = (k: string) => k.replace(/-/g, '');
  const now = new Date(stamp).toISOString().replace(/[-:]/g, '').replace(/\.\d+Z$/, 'Z');
  const lines = ['BEGIN:VCALENDAR', 'VERSION:2.0', 'PRODID:-//CDL Workshop CA//Study plan//EN', 'CALSCALE:GREGORIAN'];
  for (const d of plan.days) {
    if (d.rest) continue;
    const parts: string[] = [];
    if (d.day === plan.examDay) parts.push('Test day: light review only');
    for (const l of d.lessons) { const L = c.lessons.find((x) => x.id === l); if (L) parts.push(`${L.id} ${L.title} (~${L.minutes} min)`); }
    if (d.reviewMin && d.day !== plan.examDay) parts.push(`Review ~${d.reviewMin} min`);
    if (d.mock) parts.push(`${d.mock} mock test`);
    if (!parts.length) continue;
    const next = new Date(+d.day.slice(0, 4), +d.day.slice(5, 7) - 1, +d.day.slice(8, 10) + 1);
    const end = `${next.getFullYear()}${String(next.getMonth() + 1).padStart(2, '0')}${String(next.getDate()).padStart(2, '0')}`;
    lines.push('BEGIN:VEVENT', `UID:cdlws-${d.day}@cdl-workshop`, `DTSTAMP:${now}`, `DTSTART;VALUE=DATE:${dt(d.day)}`, `DTEND;VALUE=DATE:${end}`,
      `SUMMARY:${esc(d.day === plan.examDay ? 'CDL knowledge test' : 'CDL study: ' + (d.lessons[0] ?? 'review'))}`, `DESCRIPTION:${esc(parts.join('\n'))}`,
      'BEGIN:VALARM', 'TRIGGER:PT9H', 'ACTION:DISPLAY', 'DESCRIPTION:CDL study time', 'END:VALARM', 'END:VEVENT');
  }
  lines.push('END:VCALENDAR');
  return lines.join('\r\n');
}
