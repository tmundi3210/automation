import { useEffect, useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';
import { OtherBrake, PartId, SCam } from './ab01-foundation-brake.art';

export const meta: WidgetMeta = {
  id: 'ab01-foundation-brake', title: 'S-cam drum brake you can work', lesson: 'AB-01', anchor: /^foundation brakes$/i,
  summary: 'Press the pedal and follow the air from brake chamber to push rod, slack adjuster, S-cam and shoes; compare wedge, disc and CamLaster brakes. Then order the chain and name the parts.',
  stamp: { id: 'cam-turner', name: 'Cam turner', rule: 'Order the S-cam chain, name 4 parts and answer 4 brake-type checks with no mistakes.' },
};

/** S-cam chain, p. 5-2 (AB-01 “How an S-cam brake works”). */
const CHAIN = [
  { t: 'Air goes into the brake chamber', d: 'You push the brake pedal. Air goes into each brake chamber.' },
  { t: 'Air pushes the push rod out', d: 'The air pushes the push rod out of the chamber.' },
  { t: 'Push rod moves the slack adjuster', d: 'The push rod moves the slack adjuster.' },
  { t: 'Slack adjuster twists the camshaft; the S-cam turns', d: 'The slack adjuster twists the brake camshaft, which turns the S-cam (shaped like the letter “S”).' },
  { t: 'S-cam spreads the shoes; linings press the drum', d: 'The S-cam forces the shoes apart and presses them against the inside of the drum. Friction slows you — and makes heat.' },
  { t: 'Pedal up: S-cam turns back, return spring pulls the shoes off', d: 'When you let up, the S-cam turns back and a return spring pulls the shoes away from the drum, so the wheels roll freely.' },
];
const PARTS: { id: PartId; name: string; d: string }[] = [
  { id: 'chamber', name: 'Brake chamber', d: 'The round can that air flows into; the air pushes out a rod to work the brake.' },
  { id: 'rod', name: 'Push rod', d: 'The rod that comes out of the brake chamber when air enters it.' },
  { id: 'slack', name: 'Slack adjuster', d: 'The arm that links the push rod to the brake camshaft. The push rod moves it, and it twists the camshaft. S-cam brakes need this outside slack adjuster.' },
  { id: 'nut', name: 'Adjusting nut', d: 'The nut on the slack adjuster, shown in the handbook’s drum brake drawing (Figure 5.2).' },
  { id: 'camshaft', name: 'Camshaft', d: 'The brake camshaft: twisted by the slack adjuster, it turns the S-cam inside the drum.' },
  { id: 'scam', name: 'S-cam', d: 'An S-shaped cam that spreads the brake shoes against the drum.' },
  { id: 'roller', name: 'Cam roller', d: 'Where the S-cam pushes on the end of each shoe (Figure 5.2).' },
  { id: 'shoe', name: 'Brake shoe', d: 'Shoes and linings are pressed against the inside of the drum to make friction.' },
  { id: 'lining', name: 'Lining', d: 'The friction material on the shoe that rubs the drum. Friction slows you and makes heat.' },
  { id: 'spring', name: 'Return spring', d: 'Pulls the shoes away from the drum when you let up, so the wheels roll freely.' },
  { id: 'drum', name: 'Brake drum', d: 'On each end of the axle; the wheel bolts to it and the brake works inside it. Too much heat can make the brakes stop working.' },
  { id: 'axle', name: 'Axle', d: 'The drums sit on each end of the axles.' },
];
type Kind = 'scam' | 'wedge' | 'disc' | 'camlaster';
const KINDS: { k: Kind; name: string; how: string; adj: string }[] = [
  { k: 'scam', name: 'S-cam', how: 'The most common foundation brake.', adj: 'Needs an outside slack adjuster.' },
  { k: 'wedge', name: 'Wedge', how: 'The chamber push rod drives a wedge directly between the ends of 2 brake shoes, spreading them against the drum. May have 1 or 2 brake chambers.', adj: 'May be self-adjusting or need manual adjustment.' },
  { k: 'disc', name: 'Disc', how: 'Air works a brake chamber and slack adjuster, like an S-cam — but they turn a “power screw” that clamps the disc (rotor) between the pads of a caliper, like a big C-clamp.', adj: 'Less common than S-cam brakes.' },
  { k: 'camlaster', name: 'CamLaster', how: 'A special cam slides the shoes down a sloped ramp so they touch the drum evenly.', adj: 'Built-in (internal) adjustment keeps it adjusted all the time — no outside slack adjuster.' },
];
const segOn = { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' };
const grid = (min: number) => ({ display: 'grid', gridTemplateColumns: `repeat(auto-fit, minmax(${min}px, 1fr))`, gap: '6px' });

function Explore({ reducedMotion }: { reducedMotion: boolean }) {
  const [kind, setKind] = useState<Kind>('scam');
  const [stage, setStage] = useState(0);
  const [playing, setPlaying] = useState(false);
  const [released, setReleased] = useState(false);
  const [outAdj, setOutAdj] = useState(false);
  const [hi, setHi] = useState<PartId | null>(null);
  const [applied, setApplied] = useState(false);
  useEffect(() => {
    if (!playing) return;
    if (stage >= 5) { setPlaying(false); return; }
    const id = setTimeout(() => setStage(stage + 1), 550);
    return () => clearTimeout(id);
  }, [playing, stage]);
  const press = () => { setReleased(false); if (reducedMotion) setStage(5); else { setStage(Math.max(stage, 1)); setPlaying(true); } };
  const next = () => { setReleased(false); setPlaying(false); setStage(Math.min(5, stage + 1)); };
  const release = () => { setPlaying(false); setStage(0); setReleased(stage > 0); };
  const part = PARTS.find((p) => p.id === hi);
  const k = KINDS.find((x) => x.k === kind)!;
  const cur = released ? 5 : stage - 1;
  return (
    <div class="stack">
      <div role="group" aria-label="Brake type" style={grid(76)}>{KINDS.map((x) => (
        <button class="btn sm" aria-pressed={kind === x.k} style={kind === x.k ? segOn : {}} onClick={() => { setKind(x.k); setApplied(false); }}>{x.name}</button>
      ))}</div>
      {kind === 'scam' ? <>
        <SCam stage={stage} outAdj={outAdj} labels hi={hi} reduced={reducedMotion}
          label={`S-cam drum brake, ${stage === 0 ? 'released' : `step ${stage}: ${CHAIN[stage - 1].t}`}${outAdj ? ', out of adjustment' : ''}. Labeled: brake chamber, push rod, slack adjuster, adjusting nut, camshaft, S-cam, cam rollers, brake shoes, linings, return spring, drum, axle.`} />
        <div class="row" role="group" aria-label="Brake pedal">
          <button class="btn primary sm" onClick={press} disabled={stage === 5}>Press brake pedal</button>
          <button class="btn sm" onClick={next} disabled={stage === 5}>Next link ▸</button>
          <button class="btn sm" onClick={release} disabled={stage === 0}>Let pedal up</button>
          <label class="toggle" style={{ minHeight: '40px' }}><input type="checkbox" checked={outAdj} onChange={(e) => setOutAdj((e.target as HTMLInputElement).checked)} />Out of adjustment</label>
        </div>
        <ol class="small" aria-label="The chain" style={{ margin: 0, paddingLeft: '22px', display: 'grid', gap: '2px' }}>
          {CHAIN.map((c, i) => <li key={i} style={{ padding: '2px 6px', borderRadius: '4px', background: i === cur ? 'var(--accent-soft)' : 'transparent', fontWeight: i === cur ? 700 : 400, color: i <= cur || (released && i === 5) ? 'var(--ink)' : 'var(--ink-2)' }}>{i === cur ? '▶ ' : ''}{c.t}</li>)}
        </ol>
        <div class={`card ${outAdj && stage === 5 ? 'warn' : 'tint'} small`} role="status" aria-live="polite">
          {outAdj && stage >= 3
            ? <p><strong>Out of adjustment:</strong> the slack adjuster swings much farther, yet the linings barely reach the drum. The handbook: if the brakes are out of adjustment, <strong>neither the regular brakes nor the emergency/parking (spring) brakes will work right</strong>. <span class="plate">p. 5-4</span></p>
            : stage > 0 ? <p><strong>{stage}. </strong>{CHAIN[stage - 1].d} <span class="plate">p. 5-2</span></p>
            : released ? <p><strong>6. </strong>{CHAIN[5].d} <span class="plate">p. 5-2</span></p>
            : <p>Press the pedal (or step one link at a time) and follow the air. Tap a part below to find it on the drawing.</p>}
        </div>
        <div role="group" aria-label="Find a part" style={grid(104)}>{PARTS.map((p) => (
          <button class="btn sm" aria-pressed={hi === p.id} style={{ paddingInline: '4px', ...(hi === p.id ? segOn : {}) }} onClick={() => setHi(hi === p.id ? null : p.id)}>{p.name}</button>
        ))}</div>
        {part && <p class="card flat small" style={{ margin: 0 }}><strong>{part.name}:</strong> {part.d} <span class="plate">p. 5-2</span></p>}
      </> : <>
        <OtherBrake kind={kind} applied={applied} label={`${k.name} brake, ${applied ? 'applied' : 'released'}. ${k.how}`} />
        <div class="row"><button class="btn primary sm" aria-pressed={applied} onClick={() => setApplied(!applied)}>{applied ? 'Let pedal up' : 'Press brake pedal'}</button></div>
        <div class="card tint small"><p><strong>{k.name}:</strong> {k.how} <strong>Adjustment:</strong> {k.adj} <span class="plate">p. 5-3</span></p>
          <p class="muted">Wedge and disc brakes are less common than S-cam brakes.</p></div>
      </>}
    </div>
  );
}

const ORDER_SHOWN = [3, 0, 5, 2, 4, 1];
const IDS: { id: PartId; opts: PartId[] }[] = [
  { id: 'slack', opts: ['camshaft', 'slack', 'rod', 'chamber'] },
  { id: 'chamber', opts: ['chamber', 'drum', 'nut', 'axle'] },
  { id: 'scam', opts: ['roller', 'spring', 'scam', 'axle'] },
  { id: 'spring', opts: ['lining', 'spring', 'camshaft', 'shoe'] },
];
const NAME = (id: PartId) => PARTS.find((p) => p.id === id)!.name;
interface Q { q: string; opts: string[]; a: number; why: string; bad: string; page: string; adj?: boolean }
const QS: Q[] = [
  { q: 'Which brake uses a “power screw” to clamp the rotor between the pads of a caliper?', opts: ['Wedge brake', 'Disc brake', 'CamLaster brake'], a: 1, page: '5-3',
    why: 'Air disc brakes: the chamber and slack adjuster turn a power screw that clamps the disc between the pads, like a big C-clamp.', bad: 'A wedge brake has no screw — its push rod drives a wedge between the shoes.' },
  { q: 'Which brake keeps itself adjusted with a built-in (internal) system, so it needs no outside slack adjuster?', opts: ['S-cam brake', 'Disc brake', 'CamLaster brake'], a: 2, page: '5-3',
    why: 'The CamLaster has a completely internal adjustment system. S-cam brakes need an outside slack adjuster.', bad: 'S-cam and disc brakes both use an outside slack adjuster.' },
  { q: 'In a wedge brake, what does the chamber push rod do?', opts: ['Twists a camshaft', 'Drives a wedge between the ends of 2 shoes', 'Turns a power screw'], a: 1, page: '5-3',
    why: 'The push rod pushes a wedge directly between the ends of 2 brake shoes, shoving them against the drum. It may have 1 or 2 chambers.', bad: 'A camshaft is the S-cam design; a power screw is the disc design.' },
  { q: 'Your S-cam brakes are out of adjustment. What happens?', opts: ['Only the parking brake gets weaker', 'The spring brakes make up for it', 'Neither the regular brakes nor the emergency/parking brakes work right'], a: 2, page: '5-4', adj: true,
    why: 'Spring brake power depends on brake adjustment. Out of adjustment, neither the regular brakes nor the emergency/parking brakes will work right.', bad: 'You would count on brakes that barely touch the drum — the springs cannot make up for bad adjustment.' },
];

function Challenge({ onEvidence, onChallenge, concepts, reducedMotion }: WidgetProps) {
  const [phase, setPhase] = useState<'order' | 'id' | 'q' | 'done'>('order');
  const [built, setBuilt] = useState<number[]>([]);
  const [wrong, setWrong] = useState<number | null>(null);
  const [i, setI] = useState(0);
  const [pick, setPick] = useState<number | null>(null);
  const [misses, setMisses] = useState(0);
  const ev = (ok: boolean) => { onEvidence({ concepts, ok }); if (!ok) setMisses((m) => m + 1); };
  const reset = () => { setPhase('order'); setBuilt([]); setWrong(null); setI(0); setPick(null); setMisses(0); };
  const nextQ = (len: number, then: 'q' | 'done') => { setPick(null); if (i + 1 < len) setI(i + 1); else { setI(0); if (then === 'done' && misses === 0) onChallenge?.(); setPhase(then); } };
  const btnStyle = (c: number, a: number) => pick !== null && c === a ? { borderColor: 'var(--ok)', background: 'var(--ok-soft)', opacity: 1 } : pick === c ? { borderColor: 'var(--red)', background: 'var(--red-soft)', opacity: 1 } : {};
  const total = CHAIN.length + IDS.length + QS.length;

  if (phase === 'order') {
    const done = built.length === CHAIN.length;
    const tap = (c: number) => { const ok = c === built.length; ev(ok); if (ok) { setBuilt([...built, c]); setWrong(null); } else setWrong(c); };
    const stage = done ? 0 : Math.min(5, built.length);
    return (
      <div class="stack">
        <span class="small muted num">Part 1 of 3 · the chain ({built.length}/{CHAIN.length})</span>
        <strong>You push the brake pedal. Tap what happens, in order.</strong>
        <SCam stage={stage} outAdj={false} labels reduced={reducedMotion} label={`Brake built to step ${stage} of the chain${wrong !== null ? '; the chain has stalled' : ''}.`} />
        <div role="group" aria-label="Steps to place" style={{ display: 'grid', gap: '6px' }}>{ORDER_SHOWN.map((c) => {
          const used = built.includes(c);
          return <button class="btn sm" disabled={used || done} style={{ justifyContent: 'flex-start', textAlign: 'left', ...(used ? { opacity: 0.55 } : wrong === c ? { borderColor: 'var(--red)', background: 'var(--red-soft)' } : {}) }} onClick={() => tap(c)}>
            {used ? `${c + 1}. ` : ''}{CHAIN[c].t}{used ? ' ✓' : wrong === c ? ' ✗' : ''}</button>;
        })}</div>
        {wrong !== null && !done && <div class="feedback bad" role="status"><div class="verdict">The chain stalls at step {built.length}</div>
          <p class="small">“{CHAIN[wrong].t}” can’t happen yet. Next comes: <strong>{CHAIN[built.length].t}</strong>. {CHAIN[built.length].d} <span class="plate">p. 5-2</span></p></div>}
        {done && <div class="feedback good" role="status"><div class="verdict">Pedal → chamber → push rod → slack adjuster → camshaft/S-cam → shoes</div>
          <p class="small">And the return spring pulls the shoes off when you let up. <span class="plate">p. 5-2</span></p>
          <button class="btn primary sm" onClick={() => { setWrong(null); setPhase('id'); }}>Next: name the parts</button></div>}
      </div>
    );
  }
  if (phase === 'id') {
    const it = IDS[i];
    const a = it.opts.indexOf(it.id);
    return (
      <div class="stack">
        <span class="small muted num">Part 2 of 3 · part {i + 1} of {IDS.length}</span>
        <strong>What is the part in the dashed ring?</strong>
        <SCam stage={pick !== null ? 5 : 0} outAdj={false} labels={false} mark={it.id} hi={pick !== null ? it.id : null} reduced={reducedMotion} label="S-cam brake without labels; one part is circled." />
        <div role="group" aria-label="Part names" style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '6px' }}>{it.opts.map((o, c) => (
          <button class="btn sm" disabled={pick !== null} aria-pressed={pick === c} style={btnStyle(c, a)} onClick={() => { setPick(c); ev(c === a); }}>{NAME(o)}{pick !== null && c === a ? ' ✓' : pick === c ? ' ✗' : ''}</button>
        ))}</div>
        {pick !== null && <div class={`feedback ${pick === a ? 'good' : 'bad'}`} role="status"><div class="verdict">{pick === a ? 'Right' : `No — it is the ${NAME(it.id).toLowerCase()}`}</div>
          <p class="small"><strong>{NAME(it.id)}:</strong> {PARTS.find((p) => p.id === it.id)!.d} <span class="plate">p. 5-2</span></p>
          <button class="btn primary sm" onClick={() => nextQ(IDS.length, 'q')}>{i + 1 === IDS.length ? 'Next: brake types' : 'Next part'}</button></div>}
      </div>
    );
  }
  if (phase === 'q') {
    const q = QS[i];
    return (
      <div class="stack">
        <span class="small muted num">Part 3 of 3 · check {i + 1} of {QS.length}</span>
        <strong>{q.q}</strong>
        {q.adj && pick !== null && <SCam stage={5} outAdj labels reduced={reducedMotion} label="Out-of-adjustment S-cam brake, pedal down: the linings barely reach the drum." />}
        <div role="group" aria-label="Answers" style={{ display: 'grid', gap: '6px' }}>{q.opts.map((o, c) => (
          <button class="btn sm" disabled={pick !== null} aria-pressed={pick === c} style={{ justifyContent: 'flex-start', textAlign: 'left', ...btnStyle(c, q.a) }} onClick={() => { setPick(c); ev(c === q.a); }}>{o}{pick !== null && c === q.a ? ' ✓' : pick === c ? ' ✗' : ''}</button>
        ))}</div>
        {pick !== null && <div class={`feedback ${pick === q.a ? 'good' : 'bad'}`} role="status"><div class="verdict">{pick === q.a ? 'Right' : `No — ${q.opts[q.a]}`}</div>
          <p class="small">{q.why} <span class="plate">p. {q.page}</span></p>
          {pick !== q.a && <p class="small"><strong>Consequence:</strong> {q.bad}</p>}
          <button class="btn primary sm" onClick={() => nextQ(QS.length, 'done')}>{i + 1 === QS.length ? 'Finish' : 'Next'}</button></div>}
      </div>
    );
  }
  return (
    <div class={`feedback ${misses === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{misses === 0 ? 'Clean run — stamp earned: Cam turner' : `${misses} mistake${misses > 1 ? 's' : ''} in ${total} answers — need a clean run for the stamp`}</div>
      <button class="btn sm" onClick={reset}>Try again</button></div>
  );
}

export default function FoundationBrake(props: WidgetProps) {
  const [mode, setMode] = useState<'explore' | 'challenge'>('explore');
  return (
    <div class="stack">
      <div class="tabs" role="tablist"><button role="tab" aria-selected={mode === 'explore'} onClick={() => setMode('explore')}>Work the brake</button><button role="tab" aria-selected={mode === 'challenge'} onClick={() => setMode('challenge')}>Challenge</button></div>
      {mode === 'explore' ? <Explore reducedMotion={props.reducedMotion} /> : <Challenge {...props} />}
    </div>
  );
}
