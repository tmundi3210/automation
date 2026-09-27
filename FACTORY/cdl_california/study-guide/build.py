#!/usr/bin/env python3
"""Build the GK + Combination study guide (HTML) from the verified app content.

Facts come from app/src/content/content.json (parsed from source/lessons + verified
enrichment). The hand-written explainers below restate those same lesson facts with
worked examples; every number in them appears in the lessons' Numbers tables.
Output: study-guide/guide.html  (render to PDF with app/scripts/study-guide-pdf.mjs)
"""
import html, json, re, sys, hashlib
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
C = json.loads((ROOT / 'app/src/content/content.json').read_text())
FONTS = ROOT / 'app/node_modules/@fontsource'
OUT = Path(__file__).resolve().parent / 'guide.html'
PAGEMAP = json.loads(sys.argv[1]) if len(sys.argv) > 1 else {}

LESSONS = [l for l in C['lessons'] if l['test'] in ('GK', 'CV')]
LBY = {l['id']: l for l in LESSONS}


def clean(h: str) -> str:
    """App HTML -> print HTML: drop glossary buttons, keep bold and the CA tag."""
    h = re.sub(r'<button[^>]*>(.*?)</button>', r'\1', h)
    h = re.sub(r'<span class="ca-tag"[^>]*>CA</span>', '<span class="ca">CA</span>', h)
    return h


def esc(s: str) -> str:
    return html.escape(s, quote=False)


def pages(p):
    return ('p. ' if len(p) == 1 else 'pp. ') + ', '.join(p) if p else ''


# ---------------------------------------------------------------- priority order
mm = [m for m in C['mostMissed'] if m['test'] in ('GK', 'CV')]
mm_count = Counter()
for m in mm:
    mm_count[m['lesson']] += 1
W = {'High': 0, 'Medium': 1}


def prio(lid):
    l = LBY[lid]
    return (W.get(l['weight'], 2), -mm_count[lid], l['num'])


GK_ORDER = sorted([l['id'] for l in LESSONS if l['test'] == 'GK'], key=prio)
CV_ORDER = sorted([l['id'] for l in LESSONS if l['test'] == 'CV'], key=prio)

# ---------------------------------------------------------------- SVG diagrams
SVG_STOP = '''<svg viewBox="0 0 640 150" role="img" aria-label="Stopping distance at 55 mph: 142 ft perception + 61 ft reaction + 216 ft braking = 419 ft">
<g font-family="Atkinson Hyperlegible" font-size="13">
<rect x="10" y="40" width="217" height="44" fill="#fdf0c7" stroke="#563d00"/>
<rect x="227" y="40" width="93" height="44" fill="#dde9f7" stroke="#1f5fa8"/>
<rect x="320" y="40" width="310" height="44" fill="#f9dedc" stroke="#b3261e"/>
<text x="118" y="60" text-anchor="middle" font-weight="700">Perception 142 ft</text>
<text x="118" y="77" text-anchor="middle">1¾ s: you see it and realise</text>
<text x="273" y="60" text-anchor="middle" font-weight="700">Reaction 61 ft</text>
<text x="273" y="77" text-anchor="middle">foot to pedal</text>
<text x="475" y="60" text-anchor="middle" font-weight="700">Braking about 216 ft</text>
<text x="475" y="77" text-anchor="middle">brakes working (dry road, good brakes)</text>
<line x1="10" y1="104" x2="630" y2="104" stroke="#14201a" stroke-width="1.5"/>
<path d="M10 98v12M630 98v12" stroke="#14201a" stroke-width="1.5"/>
<text x="320" y="128" text-anchor="middle" font-weight="700" font-size="15">Total at 55 mph: at least 419 ft (longer than a football field at 60 mph)</text>
<text x="10" y="24" font-weight="700" fill="#0b5d3b">You are still moving at full speed during the first two boxes</text>
</g></svg>'''

SVG_TRI = '''<svg viewBox="0 0 660 352" role="img" aria-label="Warning triangle placement for divided highway, two-lane road and hill or curve">
<g font-family="Atkinson Hyperlegible" font-size="12">
<text x="10" y="18" font-weight="700" fill="#0b5d3b">1. Divided or one-way highway: 10 ft, 100 ft, 200 ft behind you (toward traffic)</text>
<rect x="10" y="30" width="640" height="56" fill="#e8ede5"/>
<line x1="10" y1="58" x2="650" y2="58" stroke="#fff" stroke-width="2" stroke-dasharray="14 10"/>
<rect x="560" y="64" width="80" height="18" fill="#0b5d3b"/><text x="600" y="77" fill="#fff" text-anchor="middle">your truck</text>
<text x="480" y="44">traffic comes this way →</text>
<g fill="#b3261e"><path d="M540 80l6-11 6 11z"/><path d="M440 80l6-11 6 11z"/><path d="M340 80l6-11 6 11z"/></g>
<text x="546" y="100" text-anchor="middle">10 ft</text><text x="446" y="100" text-anchor="middle">100 ft</text><text x="346" y="100" text-anchor="middle">200 ft</text>

<text x="10" y="132" font-weight="700" fill="#0b5d3b">2. Two-lane road with traffic both ways (or undivided): 1 within 10 ft of a corner, 100 ft behind, 100 ft ahead</text>
<rect x="10" y="144" width="640" height="56" fill="#e8ede5"/>
<line x1="10" y1="172" x2="650" y2="172" stroke="#e7a600" stroke-width="2"/>
<rect x="290" y="178" width="80" height="18" fill="#0b5d3b"/><text x="330" y="191" fill="#fff" text-anchor="middle">your truck</text>
<g fill="#b3261e"><path d="M272 196l6-11 6 11z"/><path d="M180 196l6-11 6 11z"/><path d="M470 196l6-11 6 11z"/></g>
<text x="278" y="214" text-anchor="middle">≤10 ft</text><text x="186" y="214" text-anchor="middle">100 ft behind</text><text x="476" y="214" text-anchor="middle">100 ft ahead</text>

<text x="10" y="246" font-weight="700" fill="#0b5d3b">3. Hill or curve hides you within 500 ft: move the rear triangle back 100–500 ft so drivers see it in time</text>
<path d="M10 316 Q 200 250 400 300 T 650 300" fill="none" stroke="#4b5a51" stroke-width="18" opacity=".25"/>
<rect x="560" y="286" width="70" height="16" fill="#0b5d3b"/><text x="595" y="298" fill="#fff" text-anchor="middle">truck</text>
<path d="M150 290l6-11 6 11z" fill="#b3261e"/>
<text x="156" y="274" text-anchor="middle">rear triangle: before the hill crest / curve (100–500 ft)</text>
<text x="10" y="346" fill="#4b5a51">Place all triangles within 10 minutes of stopping. Carry them held in front of you, between you and oncoming traffic.</text>
</g></svg>'''

SVG_RR = '''<svg viewBox="0 0 640 120" role="img" aria-label="Stop 15 to 50 feet from the nearest rail">
<g font-family="Atkinson Hyperlegible" font-size="13">
<rect x="10" y="30" width="620" height="50" fill="#e8ede5"/>
<g stroke="#4b5a51" stroke-width="3"><line x1="560" y1="24" x2="560" y2="86"/><line x1="585" y1="24" x2="585" y2="86"/></g>
<rect x="260" y="36" width="290" height="38" fill="#dcebe1" stroke="#0b5d3b" stroke-dasharray="5 4"/>
<text x="405" y="60" text-anchor="middle" font-weight="700" fill="#0b5d3b">STOP ZONE: 15–50 ft from the nearest rail</text>
<rect x="150" y="44" width="100" height="22" fill="#0b5d3b"/><text x="200" y="59" fill="#fff" text-anchor="middle">truck →</text>
<text x="555" y="102" text-anchor="end">nearest rail</text>
<text x="10" y="18">Closer than 15 ft = the train may hit you. Farther than 50 ft = you can't see well enough.</text>
<text x="10" y="112">Clear a single track: at least 14 s · a double track: more than 15 s · never shift gears on the tracks</text>
</g></svg>'''

SVG_AIR = '''<svg viewBox="0 0 660 300" role="img" aria-label="Combination air brake system: tractor tanks, tractor protection valve, red emergency line and blue service line to the trailer tank and relay valve">
<g font-family="Atkinson Hyperlegible" font-size="12">
<rect x="10" y="20" width="250" height="250" rx="10" fill="#f3f5f0" stroke="#4b5a51"/>
<text x="135" y="40" text-anchor="middle" font-weight="700">TRACTOR</text>
<rect x="30" y="60" width="90" height="36" rx="18" fill="#fff" stroke="#14201a"/><text x="75" y="83" text-anchor="middle">air tanks</text>
<rect x="30" y="200" width="100" height="44" fill="#fff" stroke="#14201a"/><text x="80" y="218" text-anchor="middle">foot brake /</text><text x="80" y="234" text-anchor="middle">hand valve</text>
<path d="M186 60 l14 0 l10 10 v14 l-10 10 h-14 l-10 -10 v-14 z" fill="#b3261e"/><text x="193" y="118" text-anchor="middle" font-size="11">red 8-sided knob</text>
<rect x="160" y="140" width="84" height="40" fill="#fff" stroke="#b3261e" stroke-width="2"/><text x="202" y="156" text-anchor="middle" font-size="11">tractor</text><text x="202" y="171" text-anchor="middle" font-size="11">protection valve</text>
<line x1="120" y1="78" x2="160" y2="160" stroke="#14201a" stroke-width="2"/>
<rect x="400" y="20" width="250" height="250" rx="10" fill="#f3f5f0" stroke="#4b5a51"/>
<text x="525" y="40" text-anchor="middle" font-weight="700">TRAILER</text>
<rect x="430" y="120" width="90" height="36" rx="18" fill="#fff" stroke="#14201a"/><text x="475" y="143" text-anchor="middle">trailer tank</text>
<rect x="540" y="190" width="90" height="40" fill="#fff" stroke="#14201a"/><text x="585" y="207" text-anchor="middle">relay valve</text><text x="585" y="222" text-anchor="middle">→ brakes</text>
<line x1="244" y1="160" x2="430" y2="138" stroke="#b3261e" stroke-width="5"/>
<text x="330" y="120" text-anchor="middle" fill="#b3261e" font-weight="700">RED = emergency</text>
<text x="330" y="135" text-anchor="middle" fill="#b3261e">(supply) line</text>
<line x1="130" y1="222" x2="540" y2="210" stroke="#1f5fa8" stroke-width="5"/>
<text x="330" y="244" text-anchor="middle" fill="#1f5fa8" font-weight="700">BLUE = service (control) line</text>
<text x="330" y="259" text-anchor="middle" fill="#1f5fa8">carries the brake signal</text>
<line x1="520" y1="150" x2="560" y2="190" stroke="#14201a" stroke-width="2"/>
<text x="475" y="175" text-anchor="middle" font-size="11">fills the tank + controls</text><text x="475" y="189" text-anchor="middle" font-size="11">trailer emergency brakes</text>
<text x="330" y="290" text-anchor="middle" fill="#4b5a51">Glad hands join the lines (held at 90°). Blue to blue, red to red.</text>
</g></svg>'''

SVG_KINGPIN = '''<svg viewBox="0 0 660 190" role="img" aria-label="Kingpin base, shank and head; jaws must close around the shank; no gap between fifth-wheel plates">
<g font-family="Atkinson Hyperlegible" font-size="12">
<rect x="40" y="20" width="260" height="22" fill="#d3dccf" stroke="#4b5a51"/><text x="170" y="36" text-anchor="middle">upper fifth wheel (trailer glide plate)</text>
<rect x="40" y="42" width="260" height="20" fill="#e8ede5" stroke="#4b5a51"/><text x="170" y="57" text-anchor="middle">lower fifth wheel (tractor)</text>
<rect x="158" y="62" width="24" height="12" fill="#14201a"/><rect x="163" y="74" width="14" height="30" fill="#4b5a51"/><rect x="156" y="104" width="28" height="14" fill="#14201a"/>
<path d="M140 80h20v18h-20zM180 80h20v18h-20z" fill="#0b5d3b"/>
<text x="210" y="72">base</text><text x="210" y="93" font-weight="700" fill="#0b5d3b">shank ← jaws HERE</text><text x="210" y="116">head (NOT here)</text>
<text x="360" y="40" font-weight="700" fill="#0b5d3b">After coupling, check:</text>
<text x="360" y="60">• NO space between upper and lower fifth wheel</text>
<text x="360" y="80">• jaws closed around the SHANK of the kingpin</text>
<text x="360" y="100">• locking lever in "locked"; safety latch over it</text>
<text x="360" y="120">• landing gear ALL the way up, crank secured</text>
<text x="360" y="140">• trailer height: low enough to be raised slightly</text>
<text x="40" y="182" fill="#4b5a51">Before backing under: fifth wheel tilted DOWN toward the rear, jaws open.</text>
</g></svg>'''

SVG_TURN = '''<svg viewBox="0 0 640 190" role="img" aria-label="Button hook right turn is correct; jug handle turn is wrong">
<g font-family="Atkinson Hyperlegible" font-size="12">
<text x="160" y="18" text-anchor="middle" font-weight="700" fill="#1d7a45">✓ BUTTON HOOK (correct)</text>
<text x="480" y="18" text-anchor="middle" font-weight="700" fill="#b3261e">✗ JUG HANDLE (wrong)</text>
<g fill="#e8ede5"><rect x="20" y="30" width="110" height="150"/><rect x="130" y="100" width="170" height="80"/><rect x="340" y="30" width="110" height="150"/><rect x="450" y="100" width="170" height="80"/></g>
<path d="M95 180 V110 Q 100 60 150 64 H300" fill="none" stroke="#1d7a45" stroke-width="4"/>
<path d="M60 180 V40" fill="none" stroke="#4b5a51" stroke-width="2" stroke-dasharray="4 4"/>
<text x="140" y="130">rear stays near the curb;</text><text x="140" y="146">swing wide as you FINISH</text>
<path d="M430 180 V140 Q 430 110 390 100 Q 360 90 390 70 Q 420 60 460 64 H620" fill="none" stroke="#b3261e" stroke-width="4"/>
<text x="460" y="130">swings left FIRST →</text><text x="460" y="146">cars pass on your right</text>
</g></svg>'''

SVG_RA = '''<svg viewBox="0 0 640 140" role="img" aria-label="Rearward amplification: 1.0 lowest, 2.0 means twice as likely, triples 3.5 highest">
<g font-family="Atkinson Hyperlegible" font-size="12">
<text x="10" y="16" font-weight="700" fill="#0b5d3b">Rearward amplification: how much harder the LAST trailer is thrown than the tractor</text>
<rect x="200" y="30" width="100" height="22" fill="#dcebe1"/><text x="10" y="46">5-axle tractor-semi, 45-ft</text><text x="306" y="46" font-weight="700">1.0 (lowest)</text>
<rect x="200" y="62" width="200" height="22" fill="#fdf0c7"/><text x="10" y="78">A rig rated 2.0</text><text x="406" y="78" font-weight="700">2.0 → last trailer tips 2× as easily</text>
<rect x="200" y="94" width="350" height="22" fill="#f9dedc"/><text x="10" y="110">Triples (27-ft trailers)</text><text x="556" y="110" font-weight="700">3.5 (highest)</text>
<text x="10" y="134" fill="#4b5a51">Triples aren't legal in California, but the 3.5 number can still be on the test.</text>
</g></svg>'''

# ---------------------------------------------------------------- hand-written parts


def box(kind, title, body):
    return f'<div class="box {kind}"><div class="bt">{title}</div>{body}</div>'


def ex(body):
    return box('ex', 'Worked example', body)


def trap(body):
    return box('tr', 'Exam trap', body)


def hook(body):
    return box('hk', 'Memory hook', body)


def tbl(head, rows, cls=''):
    th = ''.join(f'<th>{h}</th>' for h in head)
    tr = ''.join('<tr>' + ''.join(f'<td>{c}</td>' for c in r) + '</tr>' for r in rows)
    return f'<table class="{cls}"><thead><tr>{th}</tr></thead><tbody>{tr}</tbody></table>'


CONFUSING_NUMBERS = [
    ('Weight lines (who needs a CDL)', [
        ('Single vehicle needs a CDL', '<b>26,001 lb</b> GVWR or more', '26,000 is the trap'),
        ('Combination needs a CDL', '<b>26,001 lb</b> GCWR or more', ''),
        ('Towed unit that makes it Class A', '<b>10,001 lb</b> GVWR or more', '10,000 is the trap'),
        ('Passengers <span class="ca">CA</span>', 'more than <b>10</b> people <b>including the driver</b>', 'federal/other books: 16+'),
        ('3-axle vehicle <span class="ca">CA</span>', 'over <b>6,000 lb</b> → CDL (Class B)', ''),
        ('Tank endorsement (N)', 'each tank over <b>119 gal</b> and total <b>1,000 gal</b> or more', ''),
    ]),
    ('Feet', [
        ('Dim high beams', 'within <b>500 ft</b> of oncoming AND when following', '300 ft is the car-book trap'),
        ('Low beams / high beams show', 'about <b>250 ft</b> / <b>350–500 ft</b>', ''),
        ('Railroad stop (no stop line)', '<b>15–50 ft</b> from nearest rail', ''),
        ('Triangles, divided highway', '<b>10, 100, 200 ft</b> behind', ''),
        ('Triangles, two-lane road', '≤10 ft from corner, <b>100 ft behind, 100 ft ahead</b>', ''),
        ('Hill/curve hides you within', '<b>500 ft</b> → rear triangle <b>100–500 ft</b> back', ''),
        ('Mirror must show behind you', 'at least <b>200 ft</b>', ''),
        ('Following listed vehicles (CA)', 'at least <b>300 ft</b>; caravan gap <b>100 ft</b>', ''),
        ('Headlights on when you can’t see', 'a person/vehicle at <b>1,000 ft</b>', ''),
    ]),
    ('Seconds', [
        ('Look ahead', '<b>12–15 s</b> (≈1 block in town, ≈¼ mile on highway)', ''),
        ('Perception / reaction', '<b>1¾ s</b> / <b>¾–1 s</b>', ''),
        ('Following distance', '<b>1 s per 10 ft</b> of length, <b>+1 s</b> over 40 mph', ''),
        ('Clear railroad tracks', 'single: <b>at least 14 s</b> · double: <b>more than 15 s</b>', ''),
        ('Snub braking application', 'about <b>3 s</b>', ''),
        ('Stab braking: wheels restart in', 'up to <b>1 s</b>', ''),
    ]),
    ('Speeds', [
        ('CA max for 3+ axle trucks & anything towing', '<b>55 mph</b>', ''),
        ('Following distance gets +1 s', 'over <b>40 mph</b>', ''),
        ('Off the road: don’t brake until', 'about <b>20 mph</b>', ''),
        ('Downgrade snub braking', 'brake to <b>5 mph below</b> safe speed, release, repeat', ''),
        ('Excessive speeding (serious violation)', '<b>15 mph</b> or more over the limit', ''),
        ('Hydroplaning can start at', 'as low as <b>30 mph</b>', ''),
        ('Wet road', 'slow by about <b>⅓</b> (55 → about 35); packed snow: by <b>half</b> or more', ''),
    ]),
    ('Time, miles and deadlines', [
        ('Place warning triangles', 'within <b>10 minutes</b>', ''),
        ('First cargo check', 'within the first <b>50 miles</b>', ''),
        ('Later cargo checks', 'every <b>3 hours or 150 miles</b>, and after every break', ''),
        ('Hot-weather tire checks', 'every <b>2 hours or 100 miles</b>', 'same numbers as rest breaks'),
        ('No alcohol before duty', '<b>4 hours</b>', ''),
        ('Any alcohol under .04', 'out of service <b>24 hours</b>', ''),
        ('SR 1 crash report to DMV <span class="ca">CA</span>', 'within <b>10 days</b> (injury, death, or over $1,000)', ''),
        ('Tell employer about a crash in their vehicle', 'within <b>5 days</b>', ''),
        ('Report a conviction (DMV / employer)', 'within <b>30 days</b>', ''),
        ('Tell employer about a suspension', '<b>next business day</b> (p. 1-4) — or 2 business days (p. 1-16)', 'handbook says both'),
    ]),
    ('Hours of service', [
        ('California intrastate <span class="ca">CA</span>', '<b>12 h</b> driving, <b>16 h</b> on duty, 80 h in 8 days', ''),
        ('Federal', '11 h driving, 14th hour, 60/7 or 70/8', 'the trap on a CA question'),
        ('Off-duty rest', '<b>10 h</b> in a row', ''),
    ]),
    ('Air pressure (psi)', [
        ('Tractor protection valve closes / red knob pops out', '<b>20–45 psi</b>', '25–40 is the trap'),
        ('Pressure build-up check (GK inspection)', '<b>50 → 90 psi within 3 min</b>', ''),
        ('Governor cut-out', 'usually <b>120–140 psi</b>', ''),
        ('Leakage per minute, brakes released', '<b>3 psi</b> (2 vehicles) · <b>5 psi</b> (3+)', ''),
        ('Leakage per minute, foot brake held', '<b>4 psi</b> (2 vehicles) · <b>6 psi</b> (3+)', ''),
    ]),
    ('Tires, cargo, other', [
        ('Tread depth', 'front <b>4/32 in</b> · all others <b>2/32 in</b>', 'swapping them is the trap'),
        ('Tie-downs', 'at least <b>1 per 10 ft</b>, never fewer than <b>2</b>', ''),
        ('Tie-down strength (total WLL)', 'at least <b>½</b> the cargo weight', '1½× is the trap'),
        ('Steering play', 'more than <b>10°</b> (≈2 in on a 20-in wheel) = problem', ''),
        ('Missing spring leaves', '<b>¼ or more</b> = out of service', ''),
        ('Warning triangles to carry', '<b>3</b> red reflective', ''),
        ('Rollover: fully loaded vs empty', '<b>10×</b> more likely', '5× is the website trap'),
        ('Placard size', 'at least <b>9.84 in</b> (250 mm)', '10¾ in is the old-book trap'),
    ]),
]

PAIRS = [
    ('Empty vs loaded: which stops longer?', '<b>Empty</b> truck/rig needs a <b>longer</b> stopping distance (less weight = less traction; stiff springs, wheels lock). A <b>bobtail</b> tractor also takes longer than a fully loaded rig.', 'Loaded is heavier, so it takes longer.'),
    ('Right turn: button hook vs jug handle', '<b>Button hook</b>: go straight into the intersection and turn wide <b>as you finish</b>; rear stays near the curb.', 'Swing left first (jug handle) — cars squeeze in on your right.'),
    ('Drive-wheel skid vs front-wheel skid vs trailer jackknife', 'Drive-wheel braking skid → <b>stop braking</b>, then countersteer. Front-wheel skid → <b>let it slow</b>; stop turning/braking so hard. Trailer skid → <b>release the brakes</b>.', 'Brake harder / use the trailer hand valve to straighten.'),
    ('Controlled vs stab braking vs ABS', 'Controlled: as hard as possible <b>without locking</b>. Stab: full → release when wheels lock → reapply (<b>only without ABS</b>). With ABS: <b>brake normally</b>; full braking only with working ABS on <b>all</b> axles in an emergency.', 'Pump the brakes with ABS; ABS shortens stops.'),
    ('Red vs blue vs yellow', '<b>Red</b> line = emergency (supply); <b>red 8-sided knob</b> = trailer air supply. <b>Blue</b> line = service (control). <b>Yellow diamond</b> knob = parking brakes.', 'Swap colours or knobs.'),
    ('Emergency line break vs service line break', 'Emergency (red) line loses air → trailer emergency brakes <b>come on</b>, TPV closes. Service (blue) line breaks → <b>nothing until you brake</b>; then air escapes fast.', 'Service line break sets the brakes at once.'),
    ('Crossed air lines', 'Trailer <b>with</b> spring brakes → brakes <b>won’t release</b>. Old trailer <b>without</b> spring brakes → you could drive off with <b>no trailer brakes</b>.', ''),
    ('Shut-off valves', 'All <b>open</b> — except the rear of the <b>last</b> trailer: <b>closed</b>. No air at the back? Check they are open.', 'All closed / all open.'),
    ('Trailer hand valve (Johnson bar)', 'Combination test: <b>only for testing trailer brakes</b>. Never to park, never while driving, never to fix a skid. (GK p. 2-9 also mentions holding a rig from rolling back when starting — on the Combination test answer "testing only".)', 'Use it to park or to straighten a jackknife.'),
    ('Engine braking vs retarder vs service brakes', 'Downhill: <b>engine braking</b> in a low gear is the main control; brakes only help. Retarder <b>off</b> on wet, icy or snowy roads (it can skid the drive wheels).', 'Rely on the brakes; retarder on ice.'),
    ('Backing', 'Back toward the <b>driver’s (left) side</b>, lowest reverse gear, with a helper ("STOP" signal). Trailer: turn the wheel <b>opposite</b>, then follow it.', 'Turn the wheel the way you want the trailer to go.'),
    ('Convex (fisheye/spot) mirror', 'Wider view, but things look <b>smaller and farther away</b> than they are.', 'Bigger / closer.'),
    ('Tailgater behind you', '<b>Increase your following distance</b> ahead; avoid quick changes. No speeding up, no brake-light tricks.', 'Speed up or flash brake lights.'),
    ('Fire extinguishers', '<b>B:C</b> = electrical + burning liquids. <b>A:B:C</b> adds wood, paper, cloth. Never water on electrical or gasoline fires; a burning tire needs lots of water. Aim at the <b>base</b>, stand upwind.', 'Aim at the flames; open the hood.'),
    ('HazMat without an H', 'Allowed <b>only if the load does not need placards</b>. Placards needed → H. Tank + placarded HazMat → <b>X</b>.', ''),
    ('Hot tires / radiator', 'Hot tire: <b>never</b> let air out. Radiator cap: only when cool enough to <b>touch bare-handed</b>, turn slowly to the first stop.', 'Let air out of hot tires.'),
]

HARD = []  # (id, title, html)

HARD.append(('h-class', 'Which license class do I need?', f'''
<p>Ask these questions <b>in order</b>. Stop at the first "yes".</p>
<ol class="flow">
<li><b>Combination</b> (towing) with GCWR <b>26,001 lb or more</b>, and the towed unit's GVWR is <b>10,001 lb or more</b>? → <b>Class A</b>.<br><span class="ca">CA</span> Towed unit 10,001+ lb but GCWR under 26,001 lb → Class A with <b>Restriction 88</b>.</li>
<li>Power unit GVWR <b>26,001 lb or more</b> (alone, or towing something under 10,001 lb)? → <b>Class B</b>. <span class="ca">CA</span> So is a 3-axle vehicle over 6,000 lb.</li>
<li>Built or used for <b>more than 10 people including the driver</b> for pay, profit or a nonprofit (or a farm labor vehicle for 10+)? → <b>Class B</b> in the handbook's chart.</li>
<li>None of the above but hauling <b>placarded HazMat</b>? → <b>Class C</b>. Otherwise no CDL.</li>
</ol>
{ex(tbl(['Vehicle', 'Answer', 'Why'], [
    ['Dump truck, GVWR 33,000 lb, no trailer', '<b>B</b>', 'single vehicle ≥ 26,001'],
    ['Same truck towing a 12,000-lb GVWR trailer', '<b>A</b>', 'combination ≥ 26,001 and towed ≥ 10,001'],
    ['Same truck towing an 8,000-lb GVWR trailer', '<b>B</b>', 'towed unit under 10,001'],
    ['Pickup (GVWR 12,000) + 11,000-lb trailer = GCWR 23,000', '<b>A with Restriction 88</b> <span class="ca">CA</span>', 'towed ≥ 10,001 but GCWR under 26,001'],
    ['Nonprofit church van built for 12 incl. driver (not a vanpool)', '<b>CDL needed</b>', 'more than 10 people incl. driver, nonprofit use'],
    ['Pickup carrying placarded HazMat', '<b>C</b> + H', 'only reason for a CDL is the placards'],
]))}
{trap('Every line is <b>one pound above</b> a round number: 26,<b>001</b> and 10,<b>001</b>. "26,000" and "10,000" are the wrong answers.')}
'''))

HARD.append(('h-stop', 'Stopping distance', f'''
<p>Total stopping distance = <b>perception distance + reaction distance + braking distance</b>.</p>
{SVG_STOP}
<ul>
<li><b>Speed hurts fast.</b> Double your speed (20 → 40 mph) → about <b>4×</b> the braking distance and impact. Triple it → <b>9×</b>. Four times → <b>16×</b>.</li>
<li><b>Empty trucks need MORE distance</b> than loaded ones: less weight = less traction, so wheels lock more easily.</li>
<li>Wet road can <b>double</b> stopping distance → slow by about ⅓ (55 → about 35 mph). Packed snow → slow by half or more. Ice → crawl, and stop driving as soon as it's safe.</li>
<li>Bridges freeze before the road. Black ice looks like a wet road.</li>
<li>Rule of thumb: you must be able to <b>stop within the distance you can see</b> ahead.</li>
</ul>
{ex('<p>Call your braking distance at 20 mph “one unit”. At 40 mph (2× the speed) it is not 2 units but about <b>4 units</b>. At 60 mph (3×) it is about <b>9 units</b>; at 80 mph (4×) about <b>16</b>.</p><p>Add the perception and reaction distance on top, and a truck at 55 mph needs at least <b>419 ft</b> — more than a football field at 60 mph.</p>')}
{trap('"A loaded truck takes longer to stop." <b>False on the test</b> — the empty one does.')}
'''))

HARD.append(('h-follow', 'Following distance (the seconds rule)', f'''
<p><b>At least 1 second for every 10 ft of vehicle length</b> below 40 mph. <b>Over 40 mph, add 1 more second.</b></p>
<p>How to count: when the car ahead passes a landmark (a sign, a shadow), count "one thousand-and-one, one thousand-and-two…" until you reach it.</p>
{ex(tbl(['Your vehicle', 'Speed', 'Math', 'Seconds'], [
    ['40-ft truck', '35 mph', '40 ÷ 10 = 4', '<b>4 s</b>'],
    ['40-ft truck', '55 mph', '4 + 1 (over 40)', '<b>5 s</b>'],
    ['30-ft truck', '55 mph', '3 + 1', '<b>4 s</b>'],
    ['50-ft truck', '60 mph', '5 + 1', '<b>6 s</b>'],
    ['60-ft rig', '30 mph', '60 ÷ 10 = 6', '<b>6 s</b>'],
    ['60-ft rig', '55 mph', '6 + 1', '<b>7 s</b>'],
]))}
<ul><li>Slippery road → leave <b>more</b> than the rule.</li>
<li>Someone tailgates you → <b>increase</b> your space ahead (so you can brake gently), avoid quick changes, don't speed up, no brake-light tricks.</li>
<li>The space <b>ahead</b> is the most important space.</li></ul>
'''))

HARD.append(('h-tri', 'Warning triangles after a stop', f'''
{SVG_TRI}
{hook('<b>10 minutes</b> to place them. Divided highway counts up: <b>10 – 100 – 200</b>, all behind. Two-lane road: one at the truck, <b>100 behind, 100 ahead</b> (traffic comes from both sides).')}
<ul><li>Stopped on the roadside: turn on your <b>4-way flashers</b>; don't rely on taillights.</li>
<li>You must carry <b>3</b> red reflective triangles (plus a charged fire extinguisher and spare fuses unless you have circuit breakers).</li></ul>
'''))

HARD.append(('h-rr', 'Railroad crossings', f'''
{SVG_RR}
<ul>
<li><b>Passive</b> crossing = no signals — you decide if it's safe. <b>Active</b> crossing = flashing lights, bells and/or gates.</li>
<li>Round black-on-yellow sign = advance warning: slow down, look, listen, be ready to stop. Crossbuck = yield to trains.</li>
<li>Vehicles carrying <b>passengers or HazMat</b> must stop at crossings.</li>
<li>Gates: stay stopped until the gates are <b>up AND the lights stop flashing</b>.</li>
<li><b>Never shift gears</b> while crossing.</li>
<li>Low units (lowboy, car carrier, moving van, possum-belly livestock trailer) can hang up on raised crossings.</li>
<li>Stuck on the tracks → <b>get out and away first</b>, then call 9-1-1 and give the crossing's <b>DOT number</b>.</li>
</ul>
{trap('The "15 ft" and "50 ft" are distances to the <b>nearest rail</b>, and only when there is no stop line. Clearing times: single track <b>at least 14 s</b>, double <b>more than 15 s</b>.')}
'''))

HARD.append(('h-mtn', 'Mountain driving and long downgrades', f'''
<ol>
<li><b>Pick your gear before</b> you start down — usually a <b>lower</b> gear than you used to climb. (You can't safely downshift once you're gaining speed.)</li>
<li>Let <b>engine braking</b> do the main work (strongest near governed rpm in lower gears). The brakes only help.</li>
<li>Use <b>snub braking</b>: when you reach your safe speed, brake firmly enough to feel a real slowdown until you are <b>5 mph below</b> it, then release. Each application lasts about <b>3 seconds</b>. Repeat.</li>
</ol>
{ex('Your safe speed down the grade is <b>40 mph</b>. When you reach 40 → brake to <b>35</b> (about 3 s) → release. When you are back at 40 → do it again.')}
<ul><li><b>Brake fade</b>: overheated brakes need harder and harder pressure for the same stop. Brakes out of adjustment stop working first, so the others overheat.</li>
<li>Brakes fail on a downgrade → use an <b>escape ramp</b> (usually a few miles below the top). No ramp → least dangerous route (open field, uphill side road).</li>
<li>Safe speed depends on: <b>weight, length and steepness of the grade, road conditions, weather</b>.</li></ul>
{trap('"Ride the brakes lightly all the way down" is wrong — it overheats them.')}
'''))

HARD.append(('h-emerg', 'Emergencies: steer, stop, or leave the road?', f'''
<ul>
<li>You can almost always <b>steer around</b> an obstacle faster than you can stop a heavy truck.</li>
<li>Oncoming driver drifts into your lane → move <b>right</b>. Right is usually best (you won't hit oncoming traffic; the shoulder is often clear).</li>
<li>Both hands on the wheel. <b>Don't brake while turning</b> (wheels can lock). Turn only as much as needed. Be ready to <b>countersteer</b>.</li>
<li>Leaving the road: <b>don't brake until about 20 mph</b>, then brake gently; keep one set of wheels on the pavement if you can. To come back, <b>turn sharply</b>, and countersteer once both front tires are on the pavement.</li>
<li><b>Tire blowout</b> (bang, vibration, heavy steering = front, fishtailing = rear): <b>hold the wheel firmly, stay off the brake</b> until the truck slows, then brake gently.</li>
<li><b>Hydraulic brake failure</b> (pedal spongy or to the floor): downshift, pump the brakes, use the parking brake while holding its release.</li>
</ul>
{tbl(['Situation', 'Do this'], [
    ['Drive-wheel <b>braking</b> skid (most common)', 'Stop braking → countersteer quickly as it straightens'],
    ['Drive-wheel <b>acceleration</b> skid on ice/snow', 'Foot off the accelerator (push in the clutch if very slippery)'],
    ['Front-wheel skid (front goes straight)', 'Let it slow down: stop turning so sharply / braking so hard'],
    ['Trailer jackknife (combination)', '<b>Release the brakes</b> — never the trailer hand valve; you see it first in your <b>mirrors</b>'],
    ['Hydroplaning', 'Release the accelerator, push in the clutch, don’t brake'],
    ['Emergency stop, no ABS', 'Controlled braking, or stab braking (full → release when locked → reapply; up to 1 s to roll)'],
    ['Emergency stop, ABS', 'Brake normally; full brakes only if ABS works on <b>all</b> axles'],
])}
<p>4 causes of skids: <b>over-braking, over-steering, over-acceleration, driving too fast</b> (the cause of most serious skids).</p>
'''))

HARD.append(('h-abs', 'ABS in plain words', f'''
<ul>
<li>ABS is a computer that stops wheels from <b>locking</b> in hard braking. Its job is <b>control</b> (you can still steer), <b>not a shorter stop</b>.</li>
<li>It only switches on when a wheel is about to lock. Otherwise brakes work normally.</li>
<li><b>Yellow</b> ABS lamp: on the dash for tractors/trucks/buses; on trailers on the <b>left side, front or rear corner</b>. It lights at start-up then goes out (older systems: may stay on until over <b>5 mph</b>). Stays on = ABS problem; you still have normal brakes.</li>
<li>Required on: air-brake <b>tractors built on/after March 1, 1997</b>; other air-brake vehicles <b>and trailers/dollies on/after March 1, 1998</b>; hydraulic trucks/buses ≥10,000 lb on/after <b>March 1, 1999</b>.</li>
<li>ABS only on the tractor → watch the trailer, ease off if it swings. ABS only on the trailer → ease off if the tractor starts to jackknife.</li>
</ul>
{trap('ABS is <b>no</b> reason to drive faster or follow closer. It does not stop power skids or turning skids.')}
'''))

HARD.append(('h-dq', 'Losing your CDL: the penalty ladders', f'''
{tbl(['Offense type', '1st', '2nd', '3rd', 'Window'], [
    ['<b>Major</b> (DUI .04+, refusing a test, leaving a crash, felony with a CMV…)', '1 year (3 years if placarded HazMat)', 'life', '—', 'lifetime'],
    ['<b>Serious</b> (excessive speeding 15+ mph over, reckless driving, improper lane change, following too closely, violation tied to a deadly crash…)', '—', '60 days', '120 days', '3 years'],
    ['<b>Railroad crossing</b> (6 offenses)', '60 days', '120 days', '1 year', '3 years'],
    ['<b>Out-of-service order</b> violation', '90 days', '1 year', '3 years', '10 years'],
    ['<b>Hands-free / texting</b> (CMV)', '—', '60 days + 1 point', '120 days + 1 point', '3 years'],
])}
<ul><li>Alcohol: <b>.04</b> or more in a CMV = DUI. <b>Any</b> detectable alcohol below that = out of service <b>24 hours</b>. Driving a CMV = you already agreed to be tested (implied consent).</li>
<li>Using a CMV in a controlled-substance felony = disqualified <b>for life</b>.</li>
<li>Points: a violation in a CMV counts <b>1½ times</b>. Negligent operator = <b>4 points in 12 months, 6 in 24, 8 in 36</b>.</li>
<li>No hardship license for a CMV.</li></ul>
{hook('Serious = "<b>2 → 60, 3 → 120</b>". Out-of-service = "<b>90 days, 1, 3</b>" over 10 years. Railroad = "<b>60, 120, 1 year</b>".')}
'''))

HARD.append(('h-ca', 'California-only rules (they beat federal answers)', f'''
{tbl(['Topic', 'California answer', 'Don’t pick'], [
    ['Speed limit: 3+ axle trucks, anything towing, school bus with pupils, farm labor vehicle with passengers, explosives', '<b>55 mph</b>', '65'],
    ['Intrastate hours of service', '<b>12 h driving, 16 h on duty</b>; 80 h in 8 days; 10 h off in a row', '11/14 (federal)'],
    ['Passengers needing a CDL', 'more than <b>10 incl. driver</b>', '16+'],
    ['SR 1 crash report', 'to DMV within <b>10 days</b> (injury, death, damage over $1,000)', ''],
    ['Diesel idling (GVWR over 10,001 lb)', '<b>5 minutes</b> max; none at a school', ''],
    ['Turnouts', 'use when <b>5 or more</b> vehicles are behind you', ''],
    ['Lanes on 4+ lanes each way', 'right lane or the lane just left of it', ''],
    ['Length', 'single vehicle <b>40 ft</b>; tractor + trailer <b>65 ft</b>; tractor + semi + trailer <b>75 ft</b> (each trailer 28 ft 6 in)', ''],
    ['Width / height', '<b>102 in</b> body or load / <b>14 ft</b>', ''],
    ['Triple trailers', '<b>not legal</b> in California', ''],
])}
'''))

HARD.append(('h-cargo', 'Cargo: weight, balance and tie-downs', f'''
<ul>
<li>Check cargo securement within the first <b>50 miles</b>, then every <b>3 hours or 150 miles</b> (whichever first), and after every break.</li>
<li>Keep the load <b>low</b>; put heavier cargo <b>under</b> lighter cargo.</li>
<li>Too much weight on the steering axle → hard steering. Too little on the front → can't steer safely. Too little on drive axles → wheels spin.</li>
<li><b>Bridge formula</b>: axles that are <b>closer together</b> may carry <b>less</b> weight.</li>
<li>Legal maximum weight may still be <b>unsafe</b> in bad weather or mountains.</li>
</ul>
{ex(tbl(['Cargo', 'Minimum tie-downs', 'Why'], [
    ['8-ft crate', '<b>2</b>', 'never fewer than 2'],
    ['20-ft load', '<b>2</b>', '20 ÷ 10 = 2 (not 3!)'],
    ['30-ft load', '<b>3</b>', '30 ÷ 10 = 3'],
    ['25-ft load', '<b>3</b>', 'handbook doesn’t say how to count the extra 5 ft; adding one is the safe choice'],
]) + '<p><b>Strength:</b> total working load limit of all tie-downs ≥ <b>½</b> the cargo weight. A 10,000-lb machine needs at least <b>5,000 lb</b> total WLL: four 1,500-lb straps = 6,000 lb → OK.</p>')}
<ul><li>Header board ("headache rack") protects you from cargo sliding forward.</li>
<li>Sealed load you can't inspect → still check gross and axle weights.</li></ul>
'''))

HARD.append(('h-fire', 'Crashes, fires and alcohol', f'''
<p><b>At a crash, in this order:</b> 1) <b>Protect the area</b> (stop a second crash: move your truck to the side, 4-way flashers, triangles) → 2) notify authorities → 3) care for the injured → 4) collect information → 5) report.</p>
<ul><li>Stopping to help: park <b>away</b> from the wreck. Have a phone/CB → call before you get out.</li>
<li>Don't move a badly hurt person unless fire or traffic makes it necessary. Heavy bleeding → direct pressure.</li></ul>
<p><b>Fires:</b></p>
<ul><li>Pull off in an open area — <b>never</b> into a service station.</li>
<li>Engine fire → engine off, <b>don't open the hood</b>; spray through louvers, radiator or from underneath. Van/box cargo fire → <b>keep the doors shut</b>.</li>
<li>Stand upwind, as far away as possible; aim at the <b>base</b>.</li>
<li>Tire fires come from under-inflated tires and duals that touch; check tires for heat at every stop.</li></ul>
<p><b>Alcohol:</b> only <b>time</b> sobers you — the liver handles about ⅓ oz of alcohol per hour. One drink = 12 oz beer (5%) = 5 oz wine (12%) = 1½ oz 80-proof liquor. Alcohol hits <b>judgment and self-control</b> first.</p>
'''))

HARD.append(('h-cv-roll', 'Combination vehicles: rollovers, crack-the-whip, off-tracking', f'''
<ul>
<li>Rollovers cause <b>more than half</b> of truck-driver crash deaths. A <b>fully loaded</b> rig is <b>10×</b> more likely to roll over than an empty one.</li>
<li>Two ways to prevent rollover: keep cargo <b>as low as possible</b>, and go <b>slowly around turns</b> (and on ramps).</li>
<li><b>Crack-the-whip</b>: a quick lane change whips the rear trailer much harder than the tractor. The <b>last</b> trailer is the most likely to tip.</li>
</ul>
{SVG_RA}
<ul><li><b>Off-tracking</b> ("cheating"): in a turn, rear wheels cut <b>inside</b> the path of the front wheels. Longer rigs off-track more; the last trailer cuts in most.</li></ul>
{SVG_TURN}
<ul><li>Backing a trailer: turn the wheel the <b>opposite</b> way (top of wheel left → trailer goes right). Back straight; if you must curve, curve toward the <b>driver's side</b>.</li>
<li>Stopping: an <b>empty</b> rig and a <b>bobtail</b> tractor both take <b>longer</b> to stop than a loaded rig.</li></ul>
'''))

HARD.append(('h-air', 'Combination air brakes: what happens when…', f'''
{SVG_AIR}
{tbl(['If this happens…', '…then'], [
    ['Air pressure falls to <b>20–45 psi</b>', 'Tractor protection valve <b>closes</b>; red knob <b>pops out</b>; trailer emergency brakes come on; tractor keeps its air'],
    ['You pull the red knob out', 'Trailer air shut off → trailer emergency brakes on'],
    ['You push the red knob in', 'Air goes to the trailer (fills its tanks, releases its brakes)'],
    ['<b>Emergency (red)</b> line breaks or the trailer breaks away', 'Line loses pressure → trailer emergency brakes <b>come on</b>; TPV closes'],
    ['<b>Service (blue)</b> line comes apart', '<b>Nothing</b> until you brake; then air escapes, pressure drops fast, trailer emergency brakes may set'],
    ['Lines <b>crossed</b>, trailer has spring brakes', 'Trailer brakes <b>won’t release</b>'],
    ['Lines crossed, old trailer without spring brakes', 'You could drive away with <b>no trailer brakes</b>'],
    ['Trailer has no spring brakes and you park it', 'Emergency brakes hold only while tank air lasts → <b>chock the wheels</b>'],
])}
<ul><li>Trailer tanks are filled through the <b>emergency (supply)</b> line. Drain all tanks <b>every day</b>.</li>
<li>Glad hands: hold at <b>90°</b>, push the seals together, turn to lock. Unused lines go on <b>dummy couplers</b> (keep out dirt and water).</li>
<li>Shut-off valves: <b>open</b> except at the rear of the <b>last</b> trailer (closed).</li>
<li>Spring brakes aren't required on converter dollies and trailers built <b>before 1975</b>.</li></ul>
'''))

HARD.append(('h-couple', 'Coupling (16 steps) and uncoupling (10 steps)', f'''
{SVG_KINGPIN}
<div class="two">
<div><p><b>Coupling</b></p><ol class="steps">
<li>Inspect the <b>fifth wheel</b> (greased, tilted down toward the rear, jaws open, safety handle on automatic lock)</li>
<li>Inspect the area; <b>chock</b> the wheels</li>
<li>Position the tractor straight in front of the trailer</li>
<li>Back slowly until the fifth wheel <b>just touches</b> the trailer</li>
<li>Secure the tractor (parking brake, neutral)</li>
<li>Check <b>trailer height</b> (low enough to be raised slightly)</li>
<li><b>Connect the air lines</b></li>
<li>Supply air; check for crossed lines (engine off, apply/release trailer brakes, listen, watch the gauge)</li>
<li><b>Lock the trailer brakes</b> (pull the knob)</li>
<li>Back under the trailer in the <b>lowest reverse gear</b></li>
<li><b>Tug test</b>: pull forward gently with trailer brakes locked</li>
<li>Secure the vehicle (take the key)</li>
<li>Inspect: <b>no gap</b>, jaws around the <b>shank</b>, lever locked</li>
<li>Connect the electrical cord; check lines</li>
<li>Raise landing gear <b>all the way</b>; secure the crank; check clearances</li>
<li><b>Remove the chocks</b> (last step)</li>
</ol></div>
<div><p><b>Uncoupling</b></p><ol class="steps">
<li>Park on firm ground, tractor <b>in line</b> with the trailer</li>
<li>Lock trailer brakes, back up gently to ease jaw pressure, set parking brakes while pushing</li>
<li>Chock trailer wheels if no spring brakes (or not sure)</li>
<li>Lower the landing gear (loaded: a few extra turns, but don't lift off the fifth wheel)</li>
<li>Disconnect air lines (to dummy couplers) and cable (hang <b>plug down</b>)</li>
<li>Unlock the fifth wheel; keep legs away from the rear tractor wheels</li>
<li>Pull partly clear; stop with the <b>tractor frame still under the trailer</b></li>
<li>Secure the tractor</li>
<li>Inspect the ground and landing gear</li>
<li>Pull clear</li>
</ol></div></div>
{hook('Coupling order: <b>Look → Line up → Air → Lock → Back under → Tug → Check → Legs up → Chocks out</b>. Air lines and trailer-brake lock come <b>before</b> backing under.')}
{trap('Trailer too <b>high</b> → may not couple correctly. Too <b>low</b> → the tractor hits and damages the trailer nose. "Fifth wheel level" is wrong — it tilts <b>down toward the rear</b>.')}
'''))

HARD.append(('h-cvinsp', 'Combination inspection and brake check', f'''
<p>Use the same <b>7-step</b> walkaround as General Knowledge, plus:</p>
<ul><li>Lower fifth wheel mounted, greased, <b>no visible space</b> between plates, jaws around the <b>shank</b> (not the head); release arm seated, safety latch on.</li>
<li>Air lines and electrical cord secure, not leaking, with <b>enough slack</b> for turns.</li>
<li>Sliding fifth wheel: all locking pins present and locked; not too far forward.</li>
<li>Landing gear fully raised, crank handle secured.</li></ul>
{tbl(['Test', 'How', 'Pass'], [
    ['Air flow to all trailers', 'Push red knob in; apply trailer hand brake; open the last trailer’s emergency then service shut-off valves', 'Air comes out of both; then close them'],
    ['Tractor protection valve', 'Engine <b>off</b>, pump the brake pedal to drop pressure', 'Knob pops out at maker’s range, usually <b>20–45 psi</b>'],
    ['Trailer emergency brakes', 'Pull the knob out, tug gently', 'Trailer holds'],
    ['Trailer service brakes', 'Move slowly, apply the <b>hand valve</b>', 'You feel the trailer brakes grab'],
    ['Leakage (engine off)', 'Brakes released / foot brake held for 1 minute', '≤ 3 / 4 psi (2 vehicles) · ≤ 5 / 6 psi (3+)'],
])}
'''))

HARD.append(('h-insp', 'General vehicle inspection (7 steps)', f'''
<p>Walk up to the vehicle noticing its general condition, then:</p><ol><li>Vehicle overview (review the last inspection report)</li><li>Check the engine compartment (parking brakes on and/or wheels chocked <b>first</b>)</li><li>Start the engine and inspect inside the cab</li><li>Engine off, check lights (take the key; low beams + 4-way flashers on)</li><li>Walkaround inspection</li><li>Check signal lights</li><li>Start the engine and check brake system</li></ol>
<ul>
<li>Air pressure must build from <b>50 to 90 psi within 3 minutes</b>; governor cut-out usually <b>120–140 psi</b>. (The 85–100 psi in 45 s figure is the Air Brakes dual-system check — for a General Knowledge question, answer 50–90 in 3 minutes.)</li>
<li>Tread: front tires <b>4/32 in</b> in every major groove; all others <b>2/32 in</b>.</li>
<li>Steering play more than <b>10°</b> (about 2 in on a 20-in wheel) → hard to steer. Missing <b>¼</b> or more of spring leaves → out of service. Rust around lug nuts → may be loose. Welded wheel/rim repairs are unsafe.</li>
<li>Front clearance lights/reflectors <b>amber</b>, rear <b>red</b>.</li>
<li>Hydraulic brakes: pump <b>3</b> times, then press and hold <b>5 seconds</b> — the pedal shouldn't move (trap: pump 5, hold 3). Test service brakes at about <b>5 mph</b>.</li>
<li>ABS light should go out after start; staying on = fault.</li>
<li>Emergency equipment: fire extinguisher, spare fuses (unless breakers), <b>3</b> red reflective triangles.</li>
<li>Most important reason to inspect: <b>safety</b>.</li></ul>
'''))

HARD.append(('h-night', 'Seeing: mirrors, night, fog, winter, heat', f'''
<ul>
<li>Look <b>12–15 seconds</b> ahead. Convex mirrors: things look <b>smaller and farther</b>. Check mirrors 4 times in a lane change.</li>
<li>Use <b>high beams whenever safe and legal</b>; dim within <b>500 ft</b> of oncoming traffic <b>and</b> when following within 500 ft. Never sunglasses at night.</li>
<li>Headlights on from ½ hour after sunset to ½ hour before sunrise, when wipers are needed, or when you can't see 1,000 ft.</li>
<li>Fog: low beams + fog lights + 4-way flashers (even in daytime).</li>
<li>Fatigue: 15–20 min nap or sleep; the only cure is <b>sleep</b>. Crash-heavy hours midnight–6 a.m. Break every ~100 miles or 2 hours.</li>
<li>Winter: melting ice is <b>more</b> slippery; no spray from other vehicles = ice. Retarder <b>off</b>.</li>
<li>Heat: check tires every <b>2 h or 100 mi</b>; never let air out of hot tires; radiator cap only when cool to the bare hand. Bleeding tar is very slippery.</li>
</ul>
'''))


# ---------------------------------------------------------------- practice test
def pick_items():
    items = [i for i in C['items'].values() if i['origin'] == 'pack' and i['test'] in ('GK', 'CV')]
    quota = {}
    for lid in GK_ORDER:
        quota[lid] = 4 if LBY[lid]['weight'] == 'High' else (3 if lid in ('GK-01', 'GK-03') else 2)
    for lid in CV_ORDER:
        quota[lid] = 5
    out = {'GK': [], 'CV': []}
    for lid in GK_ORDER + CV_ORDER:
        pool = sorted([i for i in items if i['lesson'] == lid],
                      key=lambda i: hashlib.sha1(('pdf' + i['id']).encode()).hexdigest())
        # favour trap/number questions (the confusing kind), then others
        pool.sort(key=lambda i: 0 if (i.get('numeric') or 'trap_named' in (i.get('tags') or [])) else 1)
        out[LBY[lid]['test']] += pool[:quota[lid]]
    for t in out:  # mix lessons like the real test
        out[t].sort(key=lambda i: hashlib.sha1(('mix' + i['id']).encode()).hexdigest())
    return out


# ---------------------------------------------------------------- assemble
def font_face():
    f = []
    for fam, file, wt, st in [
        ('Atkinson Hyperlegible', 'atkinson-hyperlegible/files/atkinson-hyperlegible-latin-400-normal.woff2', 400, 'normal'),
        ('Atkinson Hyperlegible', 'atkinson-hyperlegible/files/atkinson-hyperlegible-latin-700-normal.woff2', 700, 'normal'),
        ('Atkinson Hyperlegible', 'atkinson-hyperlegible/files/atkinson-hyperlegible-latin-400-italic.woff2', 400, 'italic'),
        ('Barlow Condensed', 'barlow-condensed/files/barlow-condensed-latin-600-normal.woff2', 600, 'normal'),
        ('Barlow Condensed', 'barlow-condensed/files/barlow-condensed-latin-700-normal.woff2', 700, 'normal'),
    ]:
        f.append(f"@font-face{{font-family:'{fam}';src:url('{(FONTS / file).as_uri()}') format('woff2');font-weight:{wt};font-style:{st}}}")
    return '\n'.join(f)


CSS = '''
@page { size: Letter; margin: 0.6in 0.6in 0.7in 0.6in; }
:root { --ink:#14201a; --ink2:#4b5a51; --line:#d3dccf; --acc:#0b5d3b; --accs:#dcebe1; --amb:#fdf0c7; --ambi:#563d00; --red:#b3261e; --reds:#f9dedc; --blue:#1f5fa8; --blues:#dde9f7; --bg2:#f3f5f0; }
* { box-sizing: border-box; }
html { -webkit-print-color-adjust: exact; print-color-adjust: exact; }
body { font-family: 'Atkinson Hyperlegible', 'DejaVu Sans', sans-serif; font-size: 10.5pt; line-height: 1.42; color: var(--ink); margin: 0; background: #fff; }
h1, h2, h3, .disp { font-family: 'Barlow Condensed', 'Liberation Sans Narrow', sans-serif; letter-spacing: .01em; }
h1 { font-size: 30pt; color: var(--acc); margin: 0 0 6pt; line-height: 1.05; }
h2 { font-size: 19pt; color: var(--acc); margin: 16pt 0 6pt; border-bottom: 2px solid var(--acc); padding-bottom: 2pt; break-after: avoid; }
h3 { font-size: 14pt; margin: 12pt 0 4pt; break-after: avoid; }
h4 { font-size: 11pt; margin: 8pt 0 3pt; break-after: avoid; }
.part { break-before: page; }
.part > .ptag { font-family: 'Barlow Condensed'; font-weight: 700; color: #fff; background: var(--acc); display: inline-block; padding: 2pt 10pt; border-radius: 4pt; font-size: 12pt; letter-spacing: .06em; }
.part > h1 { margin-top: 6pt; }
.lead { font-size: 11.5pt; color: var(--ink2); }
p { margin: 4pt 0; } ul, ol { margin: 4pt 0 6pt; padding-left: 18pt; } li { margin: 2pt 0; }
b, strong { font-weight: 700; }
table { width: 100%; border-collapse: collapse; margin: 6pt 0 8pt; font-size: 9.6pt; break-inside: auto; }
th { background: var(--acc); color: #fff; text-align: left; padding: 3pt 5pt; font-weight: 700; }
td { border-bottom: 1px solid var(--line); padding: 3pt 5pt; vertical-align: top; }
tr { break-inside: avoid; }
tbody tr:nth-child(even) td { background: var(--bg2); }
.ca { display: inline-block; font-size: 7.5pt; font-weight: 700; color: #fff; background: var(--blue); border-radius: 3pt; padding: 0 3pt; line-height: 1.4; vertical-align: 1pt; }
.box { border-radius: 6pt; padding: 6pt 9pt; margin: 7pt 0; break-inside: avoid; border-left: 5px solid; }
.box .bt { font-family: 'Barlow Condensed'; font-weight: 700; font-size: 11pt; text-transform: uppercase; letter-spacing: .05em; margin-bottom: 2pt; }
.box.ex { background: var(--blues); border-color: var(--blue); } .box.ex .bt { color: var(--blue); }
.box.tr { background: var(--reds); border-color: var(--red); } .box.tr .bt { color: var(--red); }
.box.hk { background: var(--amb); border-color: #e7a600; } .box.hk .bt { color: var(--ambi); }
.box.ok { background: var(--accs); border-color: var(--acc); } .box.ok .bt { color: var(--acc); }
.box table { background: #fff; }
svg { width: 100%; height: auto; margin: 6pt 0; break-inside: avoid; display: block; }
.hard { break-inside: auto; }
.hard + .hard { margin-top: 10pt; }
.num { display: inline-block; min-width: 18pt; height: 18pt; line-height: 18pt; text-align: center; border-radius: 50%; background: var(--acc); color: #fff; font-weight: 700; font-size: 9pt; margin-right: 5pt; }
.mm li { margin: 3pt 0; break-inside: avoid; }
.mm .lt { color: var(--ink2); font-size: 8.5pt; }
.check { list-style: none; padding-left: 0; } .check li::before { content: "☐ "; color: var(--acc); font-weight: 700; }
.flow li { margin: 4pt 0; }
.two { display: grid; grid-template-columns: 1fr 1fr; gap: 12pt; }
.steps { font-size: 9.6pt; padding-left: 16pt; }
.cover { height: 9.3in; display: flex; flex-direction: column; justify-content: space-between; }
.cover .band { background: var(--acc); color: #fff; border-radius: 10pt; padding: 26pt 24pt; }
.cover .band h1 { color: #fff; font-size: 40pt; }
.cover .band .sub { font-size: 14pt; opacity: .92; }
.toc { list-style: none; padding: 0; } .toc li { display: flex; gap: 6pt; border-bottom: 1px dotted var(--line); padding: 3pt 0; } .toc li .t { flex: 1; } .toc li.s { padding-left: 16pt; font-size: 9.8pt; } .toc a { color: var(--ink); text-decoration: none; }
.lesson { break-before: page; }
.lesson .meta { color: var(--ink2); font-size: 9.2pt; }
.concept { break-inside: avoid; margin: 5pt 0 7pt; }
.concept h4 .pg, .pg { color: var(--ink2); font-weight: 400; font-size: 8.5pt; font-family: 'Atkinson Hyperlegible'; }
.concept ul { margin: 2pt 0; }
.q { break-inside: avoid; margin: 6pt 0 8pt; }
.q .stem { font-weight: 700; }
.q ol { list-style: lower-alpha; margin: 2pt 0; }
.ans td:first-child { white-space: nowrap; font-weight: 700; }
.gloss { columns: 2; column-gap: 18pt; font-size: 9.4pt; } .gloss p { break-inside: avoid; margin: 0 0 4pt; }
.small { font-size: 8.8pt; color: var(--ink2); }
.pill { display:inline-block; padding:0 6pt; border-radius:8pt; background:var(--accs); color:var(--acc); font-weight:700; font-size:8.5pt; }
'''

H = []
toc = []  # (level, id, title)


def sec(level, sid, title):
    toc.append((level, sid, title))


# Cover
H.append(f'''<section class="cover"><div>
<div class="band"><div class="disp" style="font-size:13pt;letter-spacing:.12em;opacity:.85">CALIFORNIA CDL · WRITTEN TESTS</div>
<h1>General Knowledge + Combination Vehicles</h1>
<div class="sub">The study guide — most-asked and most-confusing topics first, hard topics explained with worked examples.</div></div>
<div style="margin-top:18pt;display:grid;grid-template-columns:1fr 1fr;gap:12pt">
<div class="box ok"><div class="bt">General Knowledge test</div><b>50 questions</b> · pass with <b>40</b> correct (80%)</div>
<div class="box ok"><div class="bt">Combination Vehicles test</div><b>20 questions</b> · pass with <b>16</b> correct (80%)</div></div>
<p class="lead" style="margin-top:12pt">3 answer choices per question · 3 tries per application · no time limit. Answer the <b>California handbook way</b> (DL 650), even if another book or website says something else.</p>
</div>
<div class="small">Built from your 18 GK/CV lessons and the California Commercial Driver Handbook DL 650 (REV. 12/2019); page numbers like "p. 2-15" point to that handbook. Every fact here was checked against it. Not affiliated with the California DMV. Rules change — confirm current rules with DMV before test day.<br>Made with the CDL Workshop app (same facts as the app's lessons).</div>
</section>''')

TOC_AT = len(H)
H.append('')  # placeholder for TOC

# How to use
H.append('<section class="part" id="how"><span class="ptag">START HERE</span><h1>How to use this guide</h1>')
sec(1, 'how', 'How to use this guide')
H.append('''<ol>
<li><b>Part 1 — the 50 most-missed facts.</b> Learn these first. They come from your course's most-missed list (built from large California CDL question banks, each answer checked against the handbook). Tick each one when you can say it without looking.</li>
<li><b>Part 2 — easy to confuse.</b> Look-alike numbers and "opposite" answers side by side. Most wrong answers on the test are one of these swaps.</li>
<li><b>Part 3 — hard topics, explained.</b> Plain-language walk-throughs with diagrams and worked examples.</li>
<li><b>Part 4 — everything, lesson by lesson.</b> Key points, every number to memorize, and every exam trap for all 18 lessons. Lessons are ordered by test weight (High first), then by how many most-missed facts they hold.</li>
<li><b>Part 5 — practice tests.</b> A 50-question General Knowledge test and a 20-question Combination test, with an answer key that explains each answer. Aim for 90%+ before test day.</li>
</ol>
<div class="box hk"><div class="bt">How to read the boxes</div>
<p><span class="pill" style="background:var(--blues);color:var(--blue)">Worked example</span> step-by-step numbers · <span class="pill" style="background:var(--reds);color:var(--red)">Exam trap</span> the wrong answer the test hopes you pick · <span class="pill" style="background:var(--amb);color:var(--ambi)">Memory hook</span> a short way to remember · <span class="ca">CA</span> a California rule that beats the federal answer.</p></div>
<h3>A 7-day plan using this guide</h3>''')
H.append(tbl(['Day', 'Do'], [
    ['1', 'Part 1 (all 50). Part 3: license classes, stopping distance, following distance.'],
    ['2', 'Part 2 (all). Part 3: triangles, railroad, mountain driving.'],
    ['3', 'Part 3: emergencies, ABS, penalties, California rules, cargo, crashes/fires. Part 4: first 5 GK lessons.'],
    ['4', 'Part 4: remaining GK lessons. Re-read every red Exam trap box.'],
    ['5', 'Part 3 combination topics (rollover, air brakes, coupling, inspection). Part 4: CV lessons.'],
    ['6', 'Part 5: both practice tests without looking. Re-study every miss.'],
    ['7', 'Part 1 again + the "test-day checklist". Retake Part 5; score 90%+.'],
]))
H.append('</section>')

# Part 1
H.append('<section class="part" id="p1"><span class="ptag">PART 1</span><h1>The 50 most-missed facts</h1>')
sec(1, 'p1', 'Part 1 — The 50 most-missed facts')
H.append('<p class="lead">If you only have one hour, learn this page set. General Knowledge first (30), then Combination (20).</p>')
for t, name in (('GK', 'General Knowledge — 30 facts'), ('CV', 'Combination Vehicles — 20 facts')):
    sid = 'p1-' + t
    H.append(f'<h2 id="{sid}">{name}</h2><ol class="mm">')
    sec(2, sid, name)
    for m in [m for m in mm if m['test'] == t]:
        H.append(f'<li>{clean(m["text"])} <span class="lt">[{m["lesson"]}]</span></li>')
    H.append('</ol>')
H.append('</section>')

# Part 2
H.append('<section class="part" id="p2"><span class="ptag">PART 2</span><h1>Easy to confuse</h1>')
sec(1, 'p2', 'Part 2 — Easy to confuse')
H.append('<p class="lead">Test writers put a real handbook number from a <i>different</i> rule next to the right one. Learn these side by side.</p>')
H.append('<h2 id="p2-way">The handbook way vs. what other sources say</h2>')
sec(2, 'p2-way', 'The handbook way vs. what other sources say')
H.append(tbl(['Topic', 'Answer on the CA test', 'What others say (wrong here)'],
             [[clean(r['topic']), clean(r['answer']), clean(r['elsewhere'])] for r in C['handbookWay']]))
H.append('<h2 id="p2-num">Look-alike numbers</h2>')
sec(2, 'p2-num', 'Look-alike numbers')
for g, rows in CONFUSING_NUMBERS:
    H.append(f'<h3>{g}</h3>')
    H.append(tbl(['Rule', 'Right answer', 'Watch out'], rows))
H.append('<h2 id="p2-pairs">Same situation, opposite answers</h2>')
sec(2, 'p2-pairs', 'Same situation, opposite answers')
H.append(tbl(['Topic', 'Handbook answer', 'Tempting wrong answer'], [list(p) for p in PAIRS]))
H.append('</section>')

# Part 3
H.append('<section class="part" id="p3"><span class="ptag">PART 3</span><h1>Hard topics, explained</h1>')
sec(1, 'p3', 'Part 3 — Hard topics, explained')
H.append('<p class="lead">General Knowledge topics first, then Combination. Read the text, then check the worked example.</p>')
for i, (hid, title, body) in enumerate(HARD, 1):
    H.append(f'<div class="hard" id="{hid}"><h2><span class="num">{i}</span>{title}</h2>{body}</div>')
    sec(2, hid, f'{i}. {title}')
H.append('</section>')

# Part 4
H.append('<section class="part" id="p4"><span class="ptag">PART 4</span><h1>Everything, lesson by lesson</h1>')
sec(1, 'p4', 'Part 4 — Everything, lesson by lesson')
H.append('<p class="lead">For each lesson: key points by topic, every number to memorize, and every exam trap. General Knowledge lessons come first, ordered by test weight and how often they are missed; then Combination.</p>')
H.append(tbl(['Order', 'Lesson', 'Test weight', 'Most-missed facts'],
             [[str(n), f'{lid} {esc(LBY[lid]["title"])}', LBY[lid]['weight'], str(mm_count[lid])]
              for n, lid in enumerate(GK_ORDER + CV_ORDER, 1)]))
H.append('</section>')
for lid in GK_ORDER + CV_ORDER:
    L = LBY[lid]
    sid = 'L-' + lid
    H.append(f'<section class="lesson" id="{sid}"><h2>{lid} · {esc(L["title"])}</h2>')
    sec(2, sid, f'{lid} {L["title"]}')
    H.append(f'<p class="meta">{"General Knowledge" if L["test"] == "GK" else "Combination Vehicles"} · test weight <b>{L["weight"]}</b> · handbook {esc(L["handbook"])}</p>')
    if L.get('objectives'):
        H.append('<h4>You will be able to answer</h4><ul>' + ''.join(f'<li>{clean(o)}</li>' for o in L['objectives']) + '</ul>')
    H.append('<h3>Key points</h3>')
    for cc in [x for x in C['concepts'].values() if x['lesson'] == lid]:
        if not cc.get('core'):
            continue
        H.append(f'<div class="concept"><h4>{"<span class=ca>CA</span> " if cc.get("ca") else ""}{esc(cc["title"])} <span class="pg">{pages(cc["pages"])}</span></h4><ul>'
                 + ''.join(f'<li>{clean(b)}</li>' for b in cc['core']) + '</ul></div>')
    nums = [n for n in C['numbers'].values() if n['lesson'] == lid]
    if nums:
        H.append('<h3>Numbers and terms to memorize</h3>')
        H.append(tbl(['Item', 'Value', 'Page'], [[("<span class=ca>CA</span> " if n.get('ca') else '') + esc(n['item']), clean(n['valueHtml']), ', '.join(n['pages'])] for n in nums]))
    trs = [t for t in C['traps'].values() if t['lesson'] == lid]
    if trs:
        H.append('<h3>Exam traps</h3>')
        H.append(tbl(['Tempting wrong idea', 'Handbook answer'], [[esc(t['trap']), clean(t['correctHtml'])] for t in trs]))
    H.append('</section>')

# Part 5
PK = pick_items()
H.append('<section class="part" id="p5"><span class="ptag">PART 5</span><h1>Practice tests</h1>')
sec(1, 'p5', 'Part 5 — Practice tests')
H.append('<p class="lead">Cover the answers, circle a, b or c, then check the key at the end. Same format as the real tests: 3 choices, one correct.</p>')
qn = 0
for t, name, need in (('GK', 'General Knowledge practice test (50 questions)', 'Pass: 40 of 50'), ('CV', 'Combination practice test (20 questions)', 'Pass: 16 of 20')):
    sid = 'p5-' + t
    H.append(f'<h2 id="{sid}">{name}</h2><p><b>{need}</b></p>')
    sec(2, sid, name)
    for k, it in enumerate(PK[t], 1):
        H.append(f'<div class="q"><div class="stem">{k}. {clean(it["stem"])}</div><ol>' + ''.join(f'<li>{clean(o)}</li>' for o in it['options']) + '</ol></div>')
H.append('<h2 id="p5-key" style="break-before:page">Answer key with explanations</h2>')
sec(2, 'p5-key', 'Answer key with explanations')
for t, name in (('GK', 'General Knowledge'), ('CV', 'Combination')):
    H.append(f'<h3>{name}</h3>')
    rows = []
    for k, it in enumerate(PK[t], 1):
        L = 'abc'[it['key']]
        rows.append([f'{k}. {L}', f'{clean(it["explanation"])} <span class="pg">{pages(it["pages"])} · {it["lesson"]}</span>'])
    H.append(tbl(['Q', 'Why'], rows, 'ans'))
H.append('</section>')

# Glossary + checklist
H.append('<section class="part" id="gl"><span class="ptag">REFERENCE</span><h1>Plain-English glossary</h1>')
sec(1, 'gl', 'Plain-English glossary')
H.append('<div class="gloss">' + ''.join(f'<p><b>{esc(g["term"])}</b> — {clean(g["defHtml"])}</p>'
         for g in sorted([g for g in C['glossary'] if g['lesson'][:2] in ('GK', 'CV')], key=lambda g: g['term'].lower())) + '</div>')
H.append('<h2 id="day">Test-day checklist</h2>')
sec(1, 'day', 'Test-day checklist')
H.append('''<ul class="check">
<li>Bring your DL 44C application, ID (certified copy), residency document, Social Security proof, medical forms (MER + MEC) and the fee.</li>
<li>No notes, phone, or watch in the test — any aid or helper means the test is failed.</li>
<li>Allow 2–3 hours if you take several tests. You get 3 tries per application for knowledge tests.</li>
<li>Read every question <b>twice</b>. Watch for <b>NOT</b>, <b>EXCEPT</b>, "most", "first", "always".</li>
<li>Two answers look right? Pick the <b>handbook</b> one — the California rule, the safer action, the exact number (26,<b>001</b>, not 26,000).</li>
<li>The screen shows at once when an answer is wrong. Test-takers report a skip option that brings skipped questions back at the end (not official).</li><li>No official time limit, but DMV won't start a knowledge test within about 30 minutes of closing.</li>
<li>Last look before you go in: Part 1 of this guide.</li>
</ul>''')
H.append('</section>')

# TOC
tl = []
for lvl, sid, title in toc:
    pg = PAGEMAP.get(sid, '')
    tl.append(f'<li class="{"s" if lvl == 2 else ""}"><a class="t" href="#{sid}">{esc(title)}</a><span>{pg}</span></li>')
H[TOC_AT] = '<section class="part" id="toc"><h1>Contents</h1><ul class="toc">' + ''.join(tl) + '</ul></section>'

doc = f'''<!doctype html><html lang="en"><head><meta charset="utf-8"><title>CA CDL Study Guide — General Knowledge + Combination</title>
<style>{font_face()}\n{CSS}</style></head><body>{''.join(H)}</body></html>'''
OUT.write_text(doc)
(Path(__file__).resolve().parent / 'toc.json').write_text(json.dumps([[l, s, t] for l, s, t in toc]))
print('wrote', OUT, len(doc) // 1024, 'KB;', len(PK['GK']), 'GK +', len(PK['CV']), 'CV practice questions')
