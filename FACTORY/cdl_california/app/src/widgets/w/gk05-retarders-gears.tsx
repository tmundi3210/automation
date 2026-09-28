import { useState } from 'preact/hooks';
import type { WidgetMeta, WidgetProps } from '../registry';

export const meta: WidgetMeta = {
  id: 'gk05-retarders-gears', title: 'Retarder switch & downhill gear', lesson: 'GK-05', anchor: /retarders/i,
  summary: 'Pick a road surface and switch the retarder on or off to see what the drive wheels do. Then choose your gear before a downgrade.',
  stamp: { id: 'right-gear-grip', name: 'Right gear, right grip', rule: 'Answer all 6 retarder and downhill-gear checks with no mistakes.' },
};

type Surf = 'dry' | 'wet' | 'icy' | 'snowy';
const SURF: { id: Surf; label: string; icon: string; slick: boolean }[] = [
  { id: 'dry', label: 'Dry', icon: '☀', slick: false },
  { id: 'wet', label: 'Wet', icon: '💧', slick: true },
  { id: 'icy', label: 'Icy', icon: '❄', slick: true },
  { id: 'snowy', label: 'Snow-covered', icon: '❅', slick: true },
];
const TYPES = ['Exhaust', 'Engine', 'Hydraulic', 'Electric'];

/* ---------- side view: truck on the chosen surface */
function RoadView({ surf, on, footOff }: { surf: Surf; on: boolean; footOff: boolean }) {
  const slick = SURF.find((s) => s.id === surf)!.slick;
  const active = on && footOff;
  const skid = active && slick;
  /** Asphalt = ink at 30% over the verge: mid-grey in light AND dark themes. Each surface then adds a tinted layer on top. */
  const film = surf === 'wet' ? { fill: 'var(--blue)', o: 0.3 } : surf === 'icy' ? { fill: 'var(--blue-soft)', o: 0.7 } : surf === 'snowy' ? { fill: 'var(--surface)', o: 0.8 } : null;
  const onRoad = { fill: 'var(--ink)', stroke: 'var(--surface)', 'stroke-width': 3, 'paint-order': 'stroke' } as const;
  const state = skid ? 'Drive wheels locked and sliding: skid' : active ? 'Retarder slowing the drive wheels' : on ? 'Retarder armed; works when your foot is fully off the accelerator' : 'Retarder off: service brakes do the slowing';
  const wheel = (cx: number, drive: boolean) => (
    <g>
      <circle cx={cx} cy="128" r="15" fill="var(--ink)" stroke={drive && active ? (skid ? 'var(--red)' : 'var(--accent)') : 'var(--ink)'} stroke-width={drive && active ? 4 : 1} />
      <circle cx={cx} cy="128" r="6" fill="var(--surface-2)" />
      {drive && active && !skid && <path d={`M ${cx + 20} 118 a 20 20 0 0 1 0 20`} fill="none" stroke="var(--accent)" stroke-width="2.5" />}
    </g>
  );
  return (
    <svg viewBox="0 0 360 190" width="100%" style={{ maxWidth: '430px', marginInline: 'auto', display: 'block' }} role="img" aria-label={`Side view on a ${surf} road. ${state}.`}>
      <rect x="0" y="0" width="360" height="190" fill="var(--surface-2)" />
      <rect x="0" y="143" width="360" height="47" fill="var(--ink)" fill-opacity="0.3" />
      {film && <rect x="0" y="143" width="360" height="47" fill={film.fill} fill-opacity={film.o} />}
      <line x1="0" y1="143" x2="360" y2="143" stroke="var(--ink)" stroke-width="1.5" />
      {surf === 'wet' && [30, 110, 190, 270, 330].map((x) => <ellipse cx={x} cy="160" rx="16" ry="3" fill="var(--surface)" opacity=".6" />)}
      {surf === 'icy' && [20, 90, 170, 250, 320].map((x) => <line x1={x} y1="152" x2={x + 30} y2="148" stroke="var(--blue)" stroke-width="2.5" stroke-linecap="round" />)}
      {surf === 'snowy' && [16, 60, 104, 148, 192, 236, 280, 324].map((x) => <circle cx={x} cy="152" r="4" fill="var(--ink-2)" opacity=".35" />)}
      {/* truck, heading right */}
      <rect x="60" y="56" width="170" height="62" rx="3" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
      <path d="M 232 118 L 232 70 L 272 70 L 296 94 L 296 118 Z" fill="var(--accent)" stroke="var(--ink)" stroke-width="2" />
      <rect x="244" y="76" width="22" height="16" fill="var(--surface)" stroke="var(--ink)" />
      {wheel(90, true)}{wheel(124, true)}{wheel(276, false)}
      {skid && <g><line x1="40" y1="145" x2="106" y2="145" stroke="var(--red)" stroke-width="4" /><text x="72" y="44" font-size="15" font-weight="700" fill="var(--red)" text-anchor="middle">✕ SKID</text></g>}
      {active && !skid && <text x="107" y="44" font-size="14" font-weight="700" fill="var(--accent)" text-anchor="middle">drive wheels slowed</text>}
      <text x="107" y="178" font-size="14" text-anchor="middle" {...onRoad}>drive wheels</text>
      <text x="276" y="178" font-size="14" text-anchor="middle" {...onRoad} font-weight="700">{SURF.find((s) => s.id === surf)!.label} road</text>
    </svg>
  );
}

/* ---------- hill: gear choice before the downgrade */
type Gear = 'lower' | 'same' | 'higher';
function HillView({ gear, when }: { gear: Gear | null; when: 'before' | 'during' }) {
  const good = gear === 'lower' && when === 'before';
  const heat = gear === null ? 0 : good ? 0.25 : gear === 'lower' ? 0.5 : gear === 'same' ? 0.8 : 1;
  return (
    <svg viewBox="0 0 360 200" width="100%" style={{ maxWidth: '430px', marginInline: 'auto', display: 'block' }} role="img" aria-label={`Hill profile. ${gear ? (good ? 'Lower gear chosen before the grade: speed held without hard braking.' : 'Hard braking needed: brakes heat up and can lose braking power.') : 'Choose a gear.'}`}>
      <rect x="0" y="0" width="360" height="200" fill="var(--surface-2)" />
      <path d="M 0 140 L 110 60 L 160 60 L 360 140 L 360 200 L 0 200 Z" fill="var(--surface)" stroke="var(--ink)" stroke-width="2" />
      <text x="24" y="136" font-size="14" fill="var(--ink)">climb</text>
      <text x="226" y="130" font-size="14" fill="var(--ink)">downgrade</text>
      <line x1="160" y1="60" x2="160" y2="28" stroke="var(--accent)" stroke-width="2" stroke-dasharray="4 3" />
      <text x="154" y="20" font-size="14" text-anchor="end" fill="var(--accent)" font-weight="700">downshift BEFORE →</text>
      <g transform={when === 'before' ? 'translate(134 52)' : 'translate(250 87) rotate(22)'}>
        <rect x="-18" y="-8" width="36" height="16" rx="3" fill="var(--accent)" stroke="var(--ink)" />
      </g>
      {/* brake heat gauge, in the ground band */}
      <text x="10" y="186" font-size="14" font-weight="700" fill="var(--ink)">brake heat</text>
      <rect x="96" y="174" width="110" height="14" rx="7" fill="var(--surface-2)" stroke="var(--ink)" />
      <rect x="96" y="174" width={110 * heat} height="14" rx="7" fill={heat >= 0.8 ? 'var(--red)' : heat >= 0.5 ? 'var(--amber)' : 'var(--ok)'} />
      <text x="214" y="186" font-size="14" fill="var(--ink)">{heat >= 0.8 ? 'hot, can fade' : heat >= 0.5 ? 'rising' : heat > 0 ? 'cool' : 'pick a gear'}</text>
    </svg>
  );
}

/* ---------- challenge */
interface Opt { t: string; ok?: boolean; why: string }
interface Q { q: string; opts: Opt[]; rule: string; page: string; surf?: Surf; ret?: boolean[] }
const OFF_RULE = 'When the drive wheels have poor traction, the retarder can make them skid. Turn it off when the road is wet, icy, or snow-covered.';
const QS: Q[] = [
  { q: 'Light rain has made the road wet. What do you do with the retarder?', surf: 'wet', ret: [false, true, true], page: '2-11', rule: OFF_RULE,
    opts: [{ t: 'Turn it off', ok: true, why: 'Right — on a wet road the retarder can skid the drive wheels.' }, { t: 'Leave it on to help prevent skids', why: 'A retarder does not prevent skids — it can cause one on poor traction.' }, { t: 'Set it to its highest power', why: 'More retarder force on slick drive wheels makes a skid more likely.' }] },
  { q: 'The road is dry and you are slowing for traffic. Is using the retarder OK?', surf: 'dry', ret: [false, true, true], page: '2-11', rule: 'A retarder helps slow the vehicle, reducing the need for your brakes: less brake wear and another way to slow down. (Retarders can be noisy — know where their use is allowed.)',
    opts: [{ t: 'No — retarders are only for icy roads', why: 'Backwards: icy is exactly when it must be off.' }, { t: 'Only if you also hold the service brake down', why: 'The retarder works each time your foot is fully off the accelerator; it does not need the brake pedal.' }, { t: 'Yes — it reduces brake wear and gives another way to slow', ok: true, why: 'Right — on good traction the retarder is a help.' }] },
  { q: 'Snow covers the road. Why must the retarder be off?', surf: 'snowy', ret: [true, true, true], page: '2-11', rule: OFF_RULE,
    opts: [{ t: 'It overheats the engine in the cold', why: 'Not the handbook’s reason. The danger is a drive-wheel skid.' }, { t: 'It can make the drive wheels skid', ok: true, why: 'Right — all of its force goes to the drive wheels, which can stop turning and slide.' }, { t: 'It stops working below freezing', why: 'Not the handbook’s reason. It still works — and that is what can skid the drive wheels.' }] },
  { q: 'When a retarder is switched on, which wheels does it brake?', page: '2-11', rule: 'A retarder applies braking only to the drive wheels, each time your foot comes fully off the accelerator.',
    opts: [{ t: 'All wheels', why: 'That is the service brakes. The retarder works on the drive wheels only.' }, { t: 'Only the drive wheels', ok: true, why: 'Right — which is why slick drive wheels can skid.' }, { t: 'Only the steering wheels', why: 'The steering (front) wheels are not driven by the engine.' }] },
  { q: 'Which is NOT one of the 4 basic types of retarders?', page: '2-11', rule: 'The 4 basic types are exhaust, engine, hydraulic, and electric.',
    opts: [{ t: 'Exhaust', why: 'Exhaust is one of the 4 types.' }, { t: 'Air', ok: true, why: 'Right — “air” is not a retarder type.' }, { t: 'Hydraulic', why: 'Hydraulic is one of the 4 types.' }] },
  { q: 'You climbed this long hill in a low gear. Before you start down the other side, which gear do you choose?', page: '2-11', rule: 'Before starting down a hill, slow down and shift to a gear that controls your speed without hard braking — usually lower than the gear you would need to climb the same hill. Hard braking can overheat the brakes so they lose braking power.',
    opts: [{ t: 'One gear higher than the climbing gear', why: 'Too high: you would need hard braking, and hot brakes lose braking power.' }, { t: 'The same gear you climbed in', why: 'The handbook says usually lower than the climbing gear.' }, { t: 'Usually a lower gear than the climbing gear, chosen before starting down', ok: true, why: 'Right — pick it before the grade, not partway down.' }] },
];

function Challenge({ onEvidence, onChallenge, concepts }: WidgetProps) {
  const [i, setI] = useState(0), [pick, setPick] = useState<number | null>(null), [miss, setMiss] = useState(0);
  const q = QS[i];
  if (!q) return (
    <div class={`feedback ${miss === 0 ? 'good' : 'bad'}`} role="status"><div class="verdict">{miss === 0 ? `${QS.length} of ${QS.length} right — stamp earned: Right gear, right grip` : `${Math.max(0, QS.length - miss)} of ${QS.length} right — need all ${QS.length} for the stamp`}</div>
      <p class="small">{miss === 0 ? 'Retarder off on wet, icy, or snowy roads; lower gear before the downgrade.' : 'Try the surfaces again, then retry for the stamp.'}</p>
      <div><button class="btn sm" onClick={() => { setI(0); setMiss(0); setPick(null); }}>Try again</button></div></div>
  );
  const ok = pick !== null && !!q.opts[pick].ok;
  const surf = q.surf ? SURF.find((s) => s.id === q.surf)! : null;
  return (
    <div class="stack">
      <span class="small muted num">Question {i + 1} of {QS.length}</span>
      {surf && <span class="chip" style={{ alignSelf: 'flex-start', background: 'var(--surface-2)', color: 'var(--ink)' }}><span aria-hidden="true">{surf.icon}</span> {surf.label} road</span>}
      {surf && pick !== null && <RoadView surf={surf.id} on={q.ret ? q.ret[pick] : true} footOff={true} />}
      <strong>{q.q}</strong>
      <div class="stack" role="group" aria-label="Answer choices" style={{ gap: '8px' }}>{q.opts.map((o, k) => (
        <button class="btn" style={{ justifyContent: 'flex-start', textAlign: 'left', ...(pick !== null && o.ok ? { borderColor: 'var(--ok)', background: 'var(--ok-soft)', opacity: 1 } : pick === k ? { borderColor: 'var(--red)', background: 'var(--red-soft)', opacity: 1 } : {}) }}
          disabled={pick !== null} onClick={() => { setPick(k); if (!o.ok) setMiss(miss + 1); onEvidence({ concepts, ok: !!o.ok }); }}>
          {pick !== null && o.ok ? '✓ ' : pick === k ? '✕ ' : ''}{o.t}</button>
      ))}</div>
      {pick !== null && (
        <div class={`feedback ${ok ? 'good' : 'bad'}`} role="status">
          <div class="verdict">{ok ? 'Right' : 'Not this one'}</div>
          <p class="small">{q.opts[pick].why}</p>
          <p class="small"><strong>Handbook:</strong> {q.rule} <span class="plate">p. {q.page}</span></p>
          <div><button class="btn primary sm" onClick={() => { if (i + 1 === QS.length && miss === 0) onChallenge?.(); setPick(null); setI(i + 1); }}>{i + 1 === QS.length ? 'Finish' : 'Next question'}</button></div>
        </div>
      )}
    </div>
  );
}

const seg = (on: boolean) => (on ? { background: 'var(--accent)', color: 'var(--accent-ink)', borderColor: 'var(--accent)' } : {});

export default function RetardersGears(props: WidgetProps) {
  const [mode, setMode] = useState<'ret' | 'hill' | 'check'>('ret');
  const [surf, setSurf] = useState<Surf>('dry');
  const [on, setOn] = useState(true);
  const [footOff, setFootOff] = useState(true);
  const [type, setType] = useState(0);
  const [gear, setGear] = useState<Gear | null>(null);
  const [when, setWhen] = useState<'before' | 'during'>('before');
  const [auto, setAuto] = useState(false);
  const s = SURF.find((x) => x.id === surf)!;
  const skid = on && footOff && s.slick;

  let msg: string;
  if (s.slick) msg = on ? (footOff ? `Skid! On ${surf === 'icy' ? 'an' : 'a'} ${s.label.toLowerCase()} road the drive wheels have poor traction. All the retarder’s force goes to them, so they stop turning and slide. Turn the retarder OFF.` : 'Armed: the moment your foot comes fully off the accelerator it will brake the slick drive wheels. Turn it OFF on this road.') : `Correct setting: retarder OFF on ${surf === 'icy' ? 'an' : 'a'} ${s.label.toLowerCase()} road, so it cannot skid the drive wheels.`;
  else msg = on ? (footOff ? 'Good traction: the retarder slows the drive wheels, so you need your service brakes less — less brake wear and another way to slow down.' : 'Armed: it works each time your foot comes fully off the accelerator.') : 'Allowed, but on a dry road the retarder could be helping: less brake wear, another way to slow down.';

  return (
    <div class="stack">
      <div class="tabs" role="tablist">
        <button role="tab" aria-selected={mode === 'ret'} onClick={() => setMode('ret')}>Retarder</button>
        <button role="tab" aria-selected={mode === 'hill'} onClick={() => setMode('hill')}>Hill gear</button>
        <button role="tab" aria-selected={mode === 'check'} onClick={() => setMode('check')}>Challenge (6)</button>
      </div>

      {mode === 'ret' && (
        <div class="stack">
          <div role="group" aria-label="Road surface" style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(130px, 1fr))', gap: '8px', maxWidth: '560px' }}>{SURF.map((x) => (
            <button class="btn" aria-pressed={x.id === surf} style={{ padding: '6px 8px', ...seg(x.id === surf) }} onClick={() => setSurf(x.id)}><span aria-hidden="true">{x.icon}</span> {x.label}</button>
          ))}</div>
          <div class="row">
            <label class="toggle" style={{ minHeight: '36px' }}><input type="checkbox" checked={on} onChange={(e) => setOn((e.target as HTMLInputElement).checked)} />Retarder {on ? 'ON' : 'OFF'}</label>
            <label class="toggle" style={{ minHeight: '36px' }}><input type="checkbox" checked={footOff} onChange={(e) => setFootOff((e.target as HTMLInputElement).checked)} />Foot fully off accelerator</label>
          </div>
          <RoadView surf={surf} on={on} footOff={footOff} />
          <div class={`feedback ${skid ? 'bad' : (s.slick ? !on : on) ? 'good' : ''}`} role="status" style={!skid && !(s.slick ? !on : on) ? { background: 'var(--surface-2)' } : {}}>
            <div class="verdict">{skid ? '✕ Drive wheels skid' : s.slick ? (on ? '! Turn it off' : '✓ Off — right call') : on ? '✓ On — helping' : 'Off — OK, but not helping'}</div>
            <p class="small">{msg} <span class="plate">p. 2-11</span></p>
          </div>
          <div class="card flat stack" style={{ gap: '8px' }}>
            <strong class="small">4 basic retarder types</strong>
            <div class="row" role="group" aria-label="Retarder type">{TYPES.map((t, k) => <button class="btn sm" aria-pressed={k === type} style={seg(k === type)} onClick={() => setType(k)}>{t}</button>)}</div>
            <p class="small" aria-live="polite"><strong>{TYPES[type]} retarder</strong> — like every type: the driver can turn it on or off (some can be adjusted in strength); when on, it brakes only the drive wheels whenever your foot is fully off the accelerator; it can be noisy, so know where its use is allowed. <span class="plate">p. 2-11</span></p>
          </div>
        </div>
      )}

      {mode === 'hill' && (
        <div class="stack">
          <div class="row" role="group" aria-label="Transmission"><button class="btn sm" aria-pressed={!auto} style={seg(!auto)} onClick={() => setAuto(false)}>Manual</button><button class="btn sm" aria-pressed={auto} style={seg(auto)} onClick={() => setAuto(true)}>Automatic</button></div>
          <p class="small">You climbed this hill in a low gear. The downgrade on the other side is just as long. {auto ? 'Which range do you select?' : 'Which gear do you pick for the way down?'}</p>
          <div class="stack" role="group" aria-label="Gear choice" style={{ gap: '8px' }}>
            {(['higher', 'same', 'lower'] as Gear[]).map((g) => <button class="btn" aria-pressed={gear === g} style={{ justifyContent: 'flex-start', textAlign: 'left', ...seg(gear === g) }} onClick={() => setGear(g)}>
              {auto ? { higher: 'A high range (let it upshift)', same: 'Leave it in drive — it will pick', lower: 'A low range, for engine braking' }[g] : { higher: 'A higher gear than the climbing gear', same: 'The same gear I climbed in', lower: 'Usually a lower gear than the climbing gear' }[g]}</button>)}
          </div>
          <div class="row" role="group" aria-label="When to shift"><button class="btn sm" aria-pressed={when === 'before'} style={seg(when === 'before')} onClick={() => setWhen('before')}>Shift before starting down</button><button class="btn sm" aria-pressed={when === 'during'} style={seg(when === 'during')} onClick={() => setWhen('during')}>Shift partway down</button></div>
          <HillView gear={gear} when={when} />
          {gear && (
            <div class={`feedback ${gear === 'lower' && when === 'before' ? 'good' : 'bad'}`} role="status">
              <div class="verdict">{gear === 'lower' && when === 'before' ? '✓ Speed under control' : '✕ Hard braking ahead'}</div>
              <p class="small">{gear !== 'lower'
                ? 'This gear cannot hold your speed, so you would need hard braking. Hard braking can overheat the brakes so they lose braking power.'
                : when === 'during' ? 'Right gear, wrong moment. The handbook’s key word is BEFORE: reduce speed and pick the gear before starting down the hill.'
                : auto ? 'A low range gives more engine braking and keeps the transmission from shifting up past that gear (unless the engine goes over governor rpm).'
                : 'A gear usually lower than the climbing gear lets you control speed without hard braking.'} <span class="plate">p. 2-11</span></p>
            </div>
          )}
          <div class="card tint small"><strong>Downshift BEFORE</strong> starting down a hill (and before entering a curve). Pick a gear that controls speed without hard braking — usually lower than the gear to climb the same hill. <span class="plate">p. 2-11</span></div>
        </div>
      )}

      {mode === 'check' && <Challenge {...props} />}
    </div>
  );
}
