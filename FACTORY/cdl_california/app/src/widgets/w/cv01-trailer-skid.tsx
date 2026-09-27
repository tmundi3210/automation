import { useEffect, useRef, useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'cv01-trailer-skid', title: 'Trailer skid: mirror check', lesson: 'CV-01', anchor: /prevent trailer skids/i,
  summary: 'Brake hard, glance in the mirror, see the trailer swing, and pick your move. Try every option to see what it does.',
  stamp: { id: 'jackknife-stopped', name: 'Jackknife stopped', rule: 'Make all 4 skid and stopping decisions correctly with no mistakes.' },
};

/* ---------- visuals: left mirror view + top view. `a` = trailer swing angle in degrees (0 = straight) */
function Scene({ a, mirror }: { a: number; mirror: boolean }) {
  const s = Math.min(a, 70);
  const far = 26 + s * 1.5; // how far the trailer's rear moves out in the mirror
  const out = s >= 22;
  const rad = (s * Math.PI) / 180;
  const hx = 250, hy = 70; // hitch in top view (tractor heads up)
  const tl = 96;
  const rx = hx - Math.sin(rad) * tl, ry = hy + Math.cos(rad) * tl;
  const label = s < 3 ? 'Trailer straight behind you' : s >= 45 ? 'Jackknife: rig folded into a V' : out ? 'Trailer has left your lane' : 'Trailer swinging out';
  return (
    <svg viewBox="0 0 360 220" width="100%" style={{ maxWidth: '540px', marginInline: 'auto' }} role="img" aria-label={`${mirror ? 'Left mirror and top view' : 'Top view (mirror not checked)'}: ${label}.`}>
      <rect x="0" y="0" width="360" height="220" fill="var(--surface-2)" />
      {/* mirror */}
      <g>
        <rect x="8" y="10" width="150" height="176" rx="14" fill="var(--ink)" />
        <rect x="14" y="16" width="138" height="164" rx="10" fill={mirror ? 'var(--surface)' : 'var(--ink-2)'} />
        {mirror ? (
          <g>
            <polygon points="14,180 152,180 152,60 70,60" fill="var(--surface-2)" />
            <line x1="40" y1="180" x2="84" y2="62" stroke="var(--amber)" stroke-width="3" stroke-dasharray="12 9" />
            <polygon points={`152,34 152,176 ${152 - far},${118 - s * 0.2} ${152 - far},${70 - s * 0.3}`} fill={out ? 'var(--red-soft)' : 'var(--surface-2)'} stroke={out ? 'var(--red)' : 'var(--ink)'} stroke-width="2" />
            <text x="22" y="36" font-size="13" fill="var(--ink-2)">mirror</text>
          </g>
        ) : <text x="83" y="102" font-size="14" text-anchor="middle" fill="var(--surface)">not checked</text>}
        <text x="83" y="206" font-size="13" text-anchor="middle" fill="var(--ink)" font-weight="700">{mirror ? (s < 3 ? '✓ behind you' : out ? '✕ out of lane' : '! swinging') : '?'}</text>
      </g>
      {/* top view */}
      <rect x="182" y="10" width="170" height="176" fill="var(--surface)" stroke="var(--line)" />
      <line x1="210" y1="10" x2="210" y2="186" stroke="var(--amber)" stroke-width="2" stroke-dasharray="10 8" />
      <line x1="290" y1="10" x2="290" y2="186" stroke="var(--amber)" stroke-width="2" stroke-dasharray="10 8" />
      <g transform={`rotate(${-s * 0.25} ${hx} ${hy})`}>
        <rect x={hx - 12} y="26" width="24" height="46" rx="4" fill="var(--accent)" stroke="var(--ink)" stroke-width="1.5" />
        <rect x={hx - 10} y="28" width="20" height="9" rx="2" fill="var(--surface)" stroke="var(--ink)" />
      </g>
      <g transform={`translate(${hx} ${hy}) rotate(${s})`}>
        <rect x="-13" y="0" width="26" height={tl} rx="2" fill={out ? 'var(--red-soft)' : 'var(--surface-2)'} stroke={out ? 'var(--red)' : 'var(--ink)'} stroke-width="1.5" />
      </g>
      {s >= 3 && <path d={`M ${rx.toFixed(1)} ${ry.toFixed(1)} q -6 10 -2 22`} fill="none" stroke="var(--ink-2)" stroke-width="2" stroke-dasharray="3 3" />}
      <text x="267" y="206" font-size="13" text-anchor="middle" fill={out ? 'var(--red)' : 'var(--ink)'} font-weight="700">{label.length > 26 ? label.split(':')[0] : label}</text>
    </svg>
  );
}

/** Animate a number toward a target (instant when reducedMotion). */
function useTween(target: number, reduced: boolean) {
  const [v, setV] = useState(target);
  const cur = useRef(target), raf = useRef(0);
  useEffect(() => {
    if (reduced) { cur.current = target; setV(target); return; }
    const from = cur.current; let st = 0;
    const step = (ts: number) => { if (!st) st = ts; const u = Math.min(1, (ts - st) / 900); cur.current = from + (target - from) * (1 - (1 - u) ** 3); setV(cur.current); if (u < 1) raf.current = requestAnimationFrame(step); };
    raf.current = requestAnimationFrame(step);
    return () => cancelAnimationFrame(raf.current);
  }, [target, reduced]);
  return v;
}

type Act = 'release' | 'valve' | 'steer' | 'harder';
const ACTS: { id: Act; t: string; a: number; ok?: boolean; head: string; why: string }[] = [
  { id: 'valve', t: 'Pull the trailer hand valve', a: 62, head: 'Jackknife', why: 'The hand valve brakes only the trailer. Locked trailer brakes caused the skid, so more trailer braking locks the wheels harder and the trailer swings further. Some drivers say they do this — the handbook says no.' },
  { id: 'steer', t: 'Steer hard to catch it', a: 40, head: 'Still skidding — and tipping risk', why: 'The trailer wheels are still locked, so it keeps swinging. A sudden jerk of the wheel can also tip the trailer over (crack-the-whip).' },
  { id: 'release', t: 'Release the brakes', a: 0, ok: true, head: 'Trailer straightens', why: 'With the brakes released, the trailer wheels roll and grip the road again. With traction back, the trailer follows the tractor and straightens out.' },
  { id: 'harder', t: 'Brake harder', a: 62, head: 'Jackknife', why: 'Harder braking keeps the trailer wheels locked. Locked wheels have no grip, so the trailer swings around to the side.' },
];

/* ---------- challenge */
interface Opt { t: string; ok?: boolean; why: string; a?: number }
interface Q { q: string; opts: Opt[]; rule: string; page: string; a?: number; mirror?: boolean }
const QS: Q[] = [
  { q: 'Traffic stops suddenly and you brake hard. Your trailer is lightly loaded. What should you do while you brake?', page: '6-3', a: 0, mirror: false,
    rule: 'You see a trailer skid first — and best — in your mirrors. Whenever you brake hard, glance at the mirrors to confirm the trailer is still behind you.',
    opts: [{ t: 'Wait until you feel the trailer tug through the steering wheel', why: 'By the time you feel it, the trailer may have left your lane. The mirror shows it first.' }, { t: 'Glance in the mirrors to check the trailer is still behind you', ok: true, why: 'Right — the mirror is the earliest warning. If the trailer has already left your lane, the jackknife is very hard to stop.' }, { t: 'Listen for the trailer tires squealing', why: 'Sound is not the handbook’s signal. The mirror shows the skid first and best.' }] },
  { q: 'In the mirror, the trailer is swinging out of line. What do you do?', page: '6-3', a: 16, mirror: true,
    rule: 'Stop using the brake: release it so the tires grip again. Do not use the trailer hand brake to straighten out the rig — the trailer brakes caused the skid.',
    opts: ACTS.map((x) => ({ t: x.t, ok: x.ok, why: `${x.head}. ${x.why}`, a: x.a })) },
  { q: 'Today your trailer is empty. Compared with the same rig fully loaded, your stopping distance is:', page: '6-2',
    rule: 'An empty combination takes longer to stop: very stiff springs and strong brakes give poor traction, so the wheels lock up easily. The trailer can swing out or the tractor can jackknife.',
    opts: [{ t: 'Shorter — it is lighter', why: 'Lighter feels easier, but the tires grip poorly and the wheels lock.' }, { t: 'Longer — poor traction, the wheels lock easily', ok: true, why: 'Right — brake early and keep lots of space ahead.' }, { t: 'About the same', why: 'The handbook says an empty rig takes longer to stop.' }] },
  { q: 'You drop the trailer and drive the tractor alone (bobtail). Compared with a tractor-semitrailer at maximum gross weight, the bobtail:', page: '6-2',
    rule: 'A bobtail tractor is hard to stop smoothly and needs more distance to stop than a tractor-semitrailer loaded to maximum gross weight.',
    opts: [{ t: 'Stops in a shorter distance', why: 'A small vehicle seems easier to stop — the handbook says it takes longer.' }, { t: 'Stops in about the same distance', why: 'It takes longer, so start braking earlier.' }, { t: 'Takes longer to stop', ok: true, why: 'Right — keep more space and brake early.' }] },
];

function Challenge({ onEvidence, onChallenge, concepts, reducedMotion }: WidgetProps) {
  const [i, setI] = useState(0), [pick, setPick] = useState<number | null>(null), [miss, setMiss] = useState(0);
  const q = QS[i];
  const target = q?.a === undefined ? 0 : pick !== null && q.opts[pick].a !== undefined ? q.opts[pick].a! : pick !== null && i === 0 ? (q.opts[pick].ok ? 8 : 30) : q.a;
  const a = useTween(target, reducedMotion);
  if (!q) return (
    <div class={`feedback ${miss === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{miss === 0 ? 'All 4 right — Jackknife stopped' : `${QS.length - miss} of ${QS.length} right`}</div>
      <p class="small">{miss === 0 ? 'Mirrors when you brake hard; release the brakes; empty rigs and bobtails need more room.' : 'Try the scenario again, then retry for the stamp.'}</p>
      <div><button class="btn sm" onClick={() => { setI(0); setMiss(0); setPick(null); }}>Try again</button></div></div>
  );
  const ok = pick !== null && !!q.opts[pick].ok;
  return (
    <div class="stack">
      <span class="small muted num">Decision {i + 1} of {QS.length}</span>
      {q.a !== undefined && <Scene a={a} mirror={!!q.mirror || (pick !== null && !!q.opts[pick].ok) || pick !== null} />}
      <strong>{q.q}</strong>
      <div class="stack" role="group" aria-label="Choices" style={{ gap: '8px' }}>{q.opts.map((o, k) => (
        <button class="btn" style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick !== null && o.ok ? { borderColor: 'var(--ok)', background: 'var(--ok-soft)', opacity: 1 } : pick === k ? { borderColor: 'var(--red)', background: 'var(--red-soft)', opacity: 1 } : {}) }}
          disabled={pick !== null} onClick={() => { setPick(k); if (!o.ok) setMiss(miss + 1); onEvidence({ concepts, ok: !!o.ok }); }}>
          {pick !== null && o.ok ? '✓ ' : pick === k ? '✕ ' : ''}{o.t}</button>
      ))}</div>
      {pick !== null && (
        <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
          <div class="verdict">{ok ? 'Right' : 'Not this one'}</div>
          <p class="small">{q.opts[pick].why}</p>
          <p class="small"><strong>Handbook:</strong> {q.rule} <span class="plate">p. {q.page}</span></p>
          <div><button class="btn primary sm" onClick={() => { if (i + 1 === QS.length && miss === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === QS.length ? 'Finish' : 'Next decision'}</button></div>
        </div>
      )}
    </div>
  );
}

const STEPS = ['Hard braking', 'Mirror check', 'Your move'];

export default function TrailerSkid(props: WidgetProps) {
  const { reducedMotion } = props;
  const [mode, setMode] = useState<'try' | 'check'>('try');
  const [step, setStep] = useState(0);
  const [looked, setLooked] = useState<boolean | null>(null);
  const [act, setAct] = useState<Act | null>(null);
  const chosen = ACTS.find((x) => x.id === act);
  const target = step === 0 ? 0 : step === 1 ? 8 : looked === false ? 30 : chosen ? chosen.a : 16;
  const a = useTween(target, reducedMotion);
  const reset = () => { setStep(0); setLooked(null); setAct(null); };

  return (
    <div class="stack">
      <div class="tabs" role="tablist">
        <button role="tab" aria-selected={mode === 'try'} onClick={() => setMode('try')}>Scenario</button>
        <button role="tab" aria-selected={mode === 'check'} onClick={() => setMode('check')}>Challenge (4)</button>
      </div>
      {mode === 'try' && (
        <div class="stack">
          <ol class="row small" aria-label="Steps" style={{ listStyle: 'none', padding: 0, margin: 0, gap: '6px' }}>{STEPS.map((s, k) => (
            <li class="chip" aria-current={k === Math.min(step, 2) ? 'step' : undefined} style={{ background: k <= step ? 'var(--accent-soft)' : 'var(--surface-2)', color: k <= step ? 'var(--accent)' : 'var(--ink-2)' }}>{k + 1}. {s}</li>
          ))}</ol>
          <Scene a={a} mirror={step >= 2 && looked !== false || step === 0} />
          <div aria-live="polite" class="stack">
            {step === 0 && <>
              <p class="small">Your trailer is <strong>empty</strong>. Trailer jackknifes happen more often when the trailer is empty or lightly loaded. <span class="plate">p. 6-3</span> A car ahead stops suddenly.</p>
              <div><button class="btn primary" onClick={() => setStep(1)}>Brake hard</button></div>
            </>}
            {step === 1 && <>
              <p class="small">You are braking hard. The trailer wheels start to lock. What do you do with your eyes?</p>
              <div class="row">
                <button class="btn primary" onClick={() => { setLooked(true); setStep(2); }}>Glance in the mirror</button>
                <button class="btn" onClick={() => { setLooked(false); setStep(2); }}>Keep eyes only ahead</button>
              </div>
            </>}
            {step === 2 && looked === false && (
              <div class="feedback bad" role="status"><div class="verdict">Too late to see it</div>
                <p class="small">You never checked. The trailer has already left your lane — now stopping the jackknife is very difficult. Whenever you brake hard, glance at the mirrors to make sure the trailer is still where it belongs. <span class="plate">p. 6-3</span></p>
                <div><button class="btn sm" onClick={() => { setLooked(true); }}>Rewind: glance in the mirror</button></div></div>
            )}
            {step === 2 && looked && !act && <>
              <p class="small">The mirror shows the trailer <strong>swinging out</strong> of line. Choose your move:</p>
              <div class="grid2">{ACTS.map((x) => <button class="btn" onClick={() => setAct(x.id)}>{x.t}</button>)}</div>
            </>}
            {step === 2 && looked && chosen && (
              <div class={`feedback ${chosen.ok ? 'good' : 'bad'}`} role="status">
                <div class="verdict">{chosen.ok ? '✓ ' : '✕ '}{chosen.head}</div>
                <p class="small">{chosen.why} <span class="plate">p. 6-3</span></p>
                <div class="row"><button class="btn sm" onClick={() => setAct(null)}>Try another move</button><button class="btn sm" onClick={reset}>Start over</button></div>
              </div>
            )}
          </div>
          <div class="card tint small"><strong>Stop a trailer skid:</strong> 1) recognize it — in the mirrors; 2) stop using the brake so the trailer wheels grip again. Never use the hand valve to straighten the rig. <span class="plate">p. 6-3</span></div>
        </div>
      )}
      {mode === 'check' && <Challenge {...props} />}
    </div>
  );
}
