#!/usr/bin/env python3
"""Build the CA CDL skills-test ("behind-the-wheel") guide as HTML; see build.sh for the PDF.

Facts come from the verified lessons source/packs/SK-01..03 (handbook DL 650 sections 11-13),
GK-01 (restrictions, CLP wait), GK-04 (inspection), GK-05/CV-01 (backing). Every number carries
a handbook page. Tips that are general driving-school practice, not handbook text, are marked.
"""
import html, json, sys
from pathlib import Path
from urllib.parse import quote_plus

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import svgs  # noqa: E402

FONTS = HERE.parent / 'app/node_modules/@fontsource'
PAGEMAP = json.loads(sys.argv[1]) if len(sys.argv) > 1 else {}
H, toc = [], []


def sec(level, sid, title):
    toc.append((level, sid, title))


def box(kind, title, body):
    return f'<div class="box {kind}"><div class="bt">{title}</div>{body}</div>'


def trap(b): return box('tr', 'Common fail', b)
def tip(b): return box('hk', 'Practice tip (driving-school advice, not handbook text)', b)
def rule(b): return box('ok', 'Rule', b)


def tbl(head, rows, cls=''):
    th = ''.join(f'<th>{h}</th>' for h in head)
    tr = ''.join('<tr>' + ''.join(f'<td>{c}</td>' for c in r) + '</tr>' for r in rows)
    return f'<table class="{cls}"><thead><tr>{th}</tr></thead><tbody>{tr}</tbody></table>'


def fig(name, caption='', legend_title='What each number is'):
    s, leg = svgs.draw(name)
    out = f'<figure class="fig">{s}'
    if leg:
        out += f'<div class="legh">{legend_title}</div><ol class="leg">' + ''.join(f'<li>{x}</li>' for x in leg) + '</ol>'
    if caption:
        out += f'<figcaption>{caption}</figcaption>'
    return out + '</figure>'


def say(rows):
    """rows: (point to, say/check)."""
    return tbl(['Point to / touch', 'Say out loud'], rows, 'say')


def yt(q):
    return f'<a href="https://www.youtube.com/results?search_query={quote_plus(q)}">YouTube search: “{html.escape(q)}”</a>'


def videos(*qs):
    return '<p class="vid">Watch: ' + ' · '.join(yt(q) for q in qs) + ' <span class="small">(search links; videos are not checked — the handbook wins if they differ)</span></p>'


# ================================================================ cover
H.append('''<section class="cover"><div>
<div class="band"><div class="disp" style="font-size:13pt;letter-spacing:.12em;opacity:.85">CALIFORNIA CDL · SKILLS TESTS</div>
<h1>Behind the Wheel</h1>
<div class="sub">The vehicle inspection, basic control skills and road test — what each part is, what to say, how to drive it, and what fails you.</div></div>
<div style="margin-top:16pt">''' + svgs.flow() + '''</div>
<p class="lead" style="margin-top:8pt">Written for a <b>Class A</b> tractor-semitrailer with air brakes (your General Knowledge + Combination path). Class B/C and bus notes are included where they differ.</p>
</div>
<div class="small">Built from the California Commercial Driver Handbook DL 650 (REV. 12/2019) Sections 11–13, plus Sections 1, 2, 5 and 6; page numbers like “p. 11-4” point to it. All drawings are original, made for this guide; they are simplified, so learn the real parts on your own truck. Boxes marked “Practice tip” are common driving-school advice, not handbook rules. Not affiliated with the California DMV. Rules and test procedures change — confirm with DMV and your training provider before test day.</div>
</section>''')
TOC_AT = len(H)
H.append('')

# ================================================================ part 1 overview
H.append('<section class="part" id="p1"><span class="ptag">PART 1</span><h1>How the skills test day works</h1>')
sec(1, 'p1', 'Part 1 — How the skills test day works')
H.append('''<p class="lead">Three tests, always in this order. You bring the vehicle. Everything is in English, with no interpreter.</p>''')
H.append(tbl(['', '1. Vehicle inspection', '2. Basic control skills', '3. Road test'], [
    ['About', '~40 min', '~30 min', '~45–60 min'],
    ['What you do', 'Walk around the vehicle: point to or touch each part, name it, and say what you check and why. Start the engine, do the in-cab and air-brake checks.', 'Backing exercises inside cones and lines (a set chosen from 6).', 'Drive a set route in traffic following the examiner’s directions.'],
    ['Scored on', 'Items named and explained; brake checks done right', 'Encroachments, pull-ups, looks, final position', 'Errors (max 30) and critical driving errors (0 allowed)'],
    ['Page', '11-1 – 11-11', '12-1 – 12-4', '13-1 – 13-4'],
]))
H.append('<h2 id="p1-rules">Rules for all three tests</h2>')
sec(2, 'p1-rules', 'Rules for all three tests')
H.append('''<ul>
<li><b>Pass the vehicle inspection first.</b> Fail it, and the other two tests are put off to another day (pp. 11-1, 11-11).</li>
<li><b>3 tries in all</b> to pass the three skills tests on one application (p. 11-1). A failed basic control or road test means a retest fee when you return (p. 1-4).</li>
<li><b>English only, no interpreters.</b> Not understanding or speaking another language: verbal warnings the first <b>2</b> times; the <b>3rd</b> time that day is an <b>automatic failure</b> (pp. 11-1, 12-1, 13-1).</li>
<li><b>No aids.</b> The only aid allowed on the inspection is the handbook’s Section 11 inspection guide, and you may not write on it (p. 11-1). No phones, no helpers, no marks on the vehicle or curbs (p. 1-10).</li>
<li><b>No labels on parts.</b> A vehicle with parts marked or labeled cannot be used (p. 11-1).</li>
<li><b>Brake lights, 4-way flashers, turn signals and horn must work</b> — if not, the tests are postponed (p. 11-1).</li>
<li><b>No recording.</b> Dash cams must be off or covered on the road test (p. 1-11).</li>
<li><b>Backup cameras and self-parking</b> cannot be the only thing you use (pp. 12-1, 13-1).</li>
</ul>''')
H.append('<h2 id="p1-before">Before you book: permit, training, vehicle</h2>')
sec(2, 'p1-before', 'Before you book: permit, training, vehicle')
H.append(tbl(['Need', 'Detail'], [
    ['Commercial learner’s permit (CLP)', 'Held at least <b>14 days</b> after it was first issued (p. 1-2). Parts you pass count only within the CLP’s first 180 days; after a renewal you retake all parts (p. 1-4).'],
    ['Entry-level driver training (ELDT)', 'Federal rule for a first Class A or B CDL: finish theory and behind-the-wheel training with a provider listed on FMCSA’s Training Provider Registry before the skills test. <i>Not in the 2019 handbook — confirm with DMV and your school.</i>'],
    ['Written tests', 'Passed before any skills test (p. 1-11): General Knowledge, Combination, and Air Brakes for an air-brake truck.'],
    ['The vehicle', 'The type you want to be licensed for (p. 1-11), in safe working order, with no labels on parts. You test in it for all three tests.'],
    ['Riding with you', 'When you practice on a CLP, a CDL holder with the right class and endorsements must ride with you (p. 1-2).'],
]))
H.append('<h3>Your test vehicle sets your restrictions</h3>')
H.append(tbl(['If you test in…', 'You get restriction', 'Meaning'], [
    ['a vehicle with an automatic transmission', '<b>E</b>', 'no manual-transmission CMV'],
    ['a vehicle without air brakes (or you skipped the air brake knowledge test)', '<b>L</b>', 'no air-brake CMV'],
    ['a vehicle with air-over-hydraulic brakes', '<b>Z</b>', 'no full air-brake CMV'],
    ['a combination joined by a pintle hook or other non-fifth-wheel hitch', '<b>O</b>', 'no tractor-trailer'],
]) + '<p class="small">From p. 1-6 (restriction codes). Testing in a manual-transmission tractor-semitrailer with full air brakes avoids all four.</p>')
H.append('<h2 id="p1-auto">Automatic fails — know these cold</h2>')
sec(2, 'p1-auto', 'Automatic fails — know these cold')
H.append(tbl(['Test', 'Automatic failure'], [
    ['All three', '3rd language warning on the same day'],
    ['Inspection', 'Air brake checks not done correctly (each one done AND the numbers said) — fails the whole inspection (p. 11-4)'],
    ['Inspection', 'Hydraulic brakes: not doing BOTH parts of the check (p. 11-4)'],
    ['Basic control', 'Not securing the vehicle (Neutral + parking brake) or not getting out safely for a look — <i>may</i> be an automatic failure (p. 12-1)'],
    ['Basic control', 'Refusing an exercise or not finishing it as told — <i>may</i> be an automatic failure (p. 12-1)'],
    ['Road test', 'Any critical driving error; an accident; a moving violation; more than 30 errors (p. 13-1)'],
]))
H.append('</section>')

# ================================================================ part 2 inspection
H.append('<section class="part" id="p2"><span class="ptag">PART 2</span><h1>Test 1: Vehicle inspection</h1>')
sec(1, 'p2', 'Part 2 — Test 1: Vehicle inspection')
H.append('''<p class="lead">For each item: <b>point to or touch it, name it, and say what you are checking</b>. A check you do but don’t say may not earn credit.</p>''')
H.append(tbl(['', 'Class A', 'Class B or C'], [
    ['Test versions', '1 of <b>4</b>, all equal; you learn which just before you start', '1 of <b>3</b>'],
    ['Always included', '<b>Engine start</b>, <b>in-cab inspection</b>, <b>coupling system</b>', 'Engine start, in-cab, plus special features (e.g. bus)'],
    ['How much', 'The whole vehicle or only part — the examiner tells you', 'Same'],
]) + '<p class="small">p. 11-11. Because you don’t know which part you’ll get, learn all of it.</p>')
H.append('<h2 id="p2-anat">Know the parts: the rig from the side</h2>')
sec(2, 'p2-anat', 'Know the parts: the rig from the side')
H.append(fig('anatomy', 'Driver (left) side of a tractor-semitrailer. Lights and reflectors: <b>amber</b> at the front and sides, <b>red</b> at the rear (p. 11-3).'))
H.append('<h2 id="p2-route">Your walk-around route</h2>')
sec(2, 'p2-route', 'Your walk-around route')
H.append('<div class="two">' + fig('route', '', 'Order') + '''<div>
<p>Use the <b>same order every time</b>. A fixed order helps you remember every step under test pressure (p. 11-1).</p>
<p>This order follows the areas of the handbook’s memory aid page (the page after 11-11). It’s one sensible route, not the only allowed one.</p>
<p><b>As you walk up</b> (engine off), say:</p>
<ul><li>General condition: no damage, <b>not leaning</b> to one side.</li>
<li>Under the vehicle: no fresh <b>oil, coolant, grease or fuel</b> leaks.</li>
<li>Around it: no hazards — people, vehicles, objects, <b>low wires or limbs</b>.</li>
<li><b>Parking brakes set and/or wheels chocked</b>.</li></ul>
<p class="small">p. 11-1</p></div></div>''')

# engine
H.append('<h2 id="p2-eng">1 · Engine compartment</h2>')
sec(2, 'p2-eng', '1 · Engine compartment')
H.append(fig('engine', 'Layouts differ by engine. Some parts (often the air compressor) are gear-driven, not belt-driven: tell the examiner which, and check that it works, is mounted securely and not leaking (p. 11-2).'))
H.append(say([
    ['Ground, engine, transmission', 'No puddles on the ground, no drips under the engine or transmission. Hoses in good condition, not leaking.'],
    ['Water pump · alternator · air compressor', 'Name each. Mounted securely, not leaking. Alternator wires fastened.'],
    ['Oil dipstick', 'Engine off. This is the dipstick; the level is in the safe range, above the refill mark.'],
    ['Coolant sight glass', 'Level shows in the sight glass. (No sight glass: describe what you’d look for with the radiator cap off.)'],
    ['Power steering fluid', 'Dipstick or sight glass: above the refill mark.'],
    ['Washer fluid', 'Level OK, cap secure.'],
    ['Each belt', 'Snug: <b>½ to ¾ inch</b> of play at the center of the belt; no cracks, frays, loose fibers or wear.'],
    ['Brake master cylinder (hydraulic brakes)', 'Securely attached, not leaking; fluid between the add and full marks.'],
]) + '<p class="small">p. 11-2. Example: you press the middle of the alternator belt and it moves about ⅝ in — inside ½–¾, so it is snug enough. Still check it for cracks and frays.</p>')

# cab
H.append('<h2 id="p2-cab">2 · Engine start and in-cab checks</h2>')
sec(2, 'p2-cab', '2 · Engine start and in-cab checks')
H.append('<p><b>Safe start:</b> shift lever in <b>Neutral</b> (Park for an automatic) → <b>press the clutch</b> → start → keep the clutch in until the engine reaches idle speed → let the clutch out slowly (p. 11-2).</p>')
H.append(fig('dash', 'Gauge layouts vary by truck; know where yours are.'))
H.append(tbl(['Item', 'What should happen / what you say'], [
    ['Oil pressure', 'Rises to normal (or the warning light goes off); oil temperature, if fitted, climbs slowly to normal. (Handbook example gauge: 5–20 psi idling, 35–75 psi operating, p. 2-5.)'],
    ['Coolant temperature', 'Starts to climb to normal (or the light goes off).'],
    ['Air pressure', 'Builds to governor cut-out, about <b>120–140 psi</b> or the maker’s number.'],
    ['Ammeter / voltmeter', 'Shows charging (or the warning light is off).'],
    ['Mirrors, windshield', 'Clean, adjusted; no illegal stickers, nothing blocking the view, no glass damage.'],
    ['Wipers / washers', 'Arms and blades secure, undamaged, move smoothly; washers work.'],
    ['Horn(s)', 'Air and/or electric horn works.'],
    ['Heater / defroster', 'Both work.'],
    ['Safety belt', 'Securely mounted, adjusts, latches; not ripped or frayed.'],
    ['Lights (dash)', 'Left and right turn signal, 4-way flasher, high-beam and ABS indicators come on.'],
]) + '<p class="small">pp. 11-2 – 11-3</p>')
H.append('<h3>Emergency equipment</h3>' + tbl(['Required', 'Optional (mention if carried)'], [[
    'Spare electrical fuses (say so if the vehicle has none)<br><b>3 red reflective triangles</b>, or 6 fuses, or 3 liquid-burning flares<br>Fire extinguisher: <b>charged</b> and <b>securely mounted</b>',
    'Tire chains (where winter needs them)<br>Tire-changing equipment<br>List of emergency phone numbers<br>Accident reporting kit']]) + '<p class="small">p. 11-3</p>')

# air brake
H.append('<h2 id="p2-air">3 · Air brake checks (where people fail)</h2>')
sec(2, 'p2-air', '3 · Air brake checks (where people fail)')
H.append(rule('Do <b>each</b> check <b>and say the numbers</b>. The order is up to you. Doing any of them wrong = <b>automatic failure of the whole inspection</b> (p. 11-4).'))
H.append(fig('airchart', 'Pressure on your gauge during the checks (drawn, not to scale).', 'Steps'))
H.append(tbl(['Check', 'How', 'Pass / what to say'], [
    ['<b>Applied leakage</b>', 'Air at cut-out (say when it cut out). Engine off, chock wheels if needed, release the parking brake (and tractor protection valve). Press the foot brake fully and hold <b>1 minute</b> after the gauge settles.', 'Loss no more than <b>3 psi</b> single vehicle · <b>4 psi</b> combination of 2 · <b>6 psi</b> combination of 3+. Say the loss and your limit.'],
    ['<b>Low-air warning</b>', 'Key on. Fan off the air by pumping the foot brake.', 'Buzzer/light/flag comes on <b>before 55 psi</b> (or the maker’s level). Say where.'],
    ['<b>Spring brakes pop</b>', 'Parking brake (and tractor protection valve) released; keep fanning the air down.', 'Parking brake knob (and trailer air supply knob on a tractor-trailer) <b>pop out</b>, normally <b>20–45 psi</b>. Say where.'],
    ['<b>Parking brake</b>', 'Seat belt on. Air at cut-out, parking brake on, trailer brakes released: gently try to pull forward. Then set only the <b>trailer</b> parking brake and gently pull against it.', 'Vehicle does not move.'],
    ['<b>Service brake</b>', 'Roll ahead at about <b>5 mph</b>, press the brake to stop.', 'Stops; no pull to either side.'],
]) + '<p class="small">pp. 11-4, 5-8 – 5-9. A large bus often warns at 80–85 psi: say the 55–75 psi range and that your bus is built to warn higher (p. 5-9).</p>')
H.append(box('ex', 'Say it like this (example)', '<p>“Air is at 125 psi, the governor cut out at 125. Engine off, parking brakes released, foot brake applied and held… after one minute I lost 2 psi. The limit for a two-vehicle combination is 4 psi, so it passes.”</p><p>“Pumping the brake… the low-air warning came on at 62 psi, above 55 — passes. Still pumping… the trailer air supply knob popped at 40 psi and the parking brake knob at 30 — both within 20 to 45.”</p><p class="small">Numbers here are an example; say the real numbers your gauge shows.</p>'))
H.append(trap('Rushing the leak test (not waiting for the gauge to settle, or not holding a full minute) · forgetting to <b>say</b> the numbers · checking the parking brake without the seat belt on · pulling hard against the brakes instead of gently.'))
H.append('<h3>Hydraulic brakes instead (if your truck has them)</h3><p>Pump the pedal <b>3 times</b>, then hold it down <b>5 seconds</b>: it must not sink. With a reserve (backup) system: key off, press the pedal, listen for the reserve pump motor; warning buzzer/light off. <b>Skipping either part = automatic failure</b>. Hydro-Boost: engine off, pump the pressure off, hold the pedal lightly (15–25 lb), start the engine — the pedal gives a little then holds (p. 11-4).</p>')

# steering, suspension, brakes
H.append('<h2 id="p2-steer">4 · Steering</h2>')
sec(2, 'p2-steer', '4 · Steering')
H.append(fig('steering'))
H.append(say([
    ['Steering gear box and hoses', 'Mounted securely, not leaking, no missing nuts, bolts or cotter keys. No power steering fluid leaks or hose damage.'],
    ['Pitman arm, drag link, steering arm, tie rod', 'Not worn or cracked; joints and sockets not worn or loose; no missing nuts, bolts or cotter keys.'],
]) + '<p class="small">p. 11-5. A cotter key is a small split pin that stops a nut from backing off. In the cab, steering play of more than 10° (about 2 in on a 20-in wheel) makes the truck hard to steer (p. 2-2).</p>')
H.append('<h2 id="p2-susp">5 · Suspension</h2>')
sec(2, 'p2-susp', '5 · Suspension')
H.append(fig('suspension', 'Out of service when <b>¼ or more</b> of the leaves in a spring are missing or broken. Example: 12 leaves → 3 or more = out of service (p. 11-5).'))
H.append(say([
    ['Springs (leaf, coil, torsion bars, air ride)', 'No missing, shifted, cracked or broken leaves; air ride not damaged or leaking.'],
    ['Spring mounts: hangers, bushings, U-bolts', 'No cracked or broken hangers, no missing or damaged bushings, no broken, loose or missing bolts or U-bolts — at the frame and the axle.'],
    ['Shock absorbers', 'Secure, not leaking.'],
]) + '<p class="small">p. 11-5. Be ready to do this on every axle — tractor and trailer.</p>')
H.append('<h2 id="p2-brk">6 · Brakes at each wheel</h2>')
sec(2, 'p2-brk', '6 · Brakes at each wheel')
H.append(fig('brakes', 'Manual slack adjuster, brakes <b>released</b>: pull it by hand — the pushrod should not move more than <b>1 inch</b> (p. 11-5).'))
H.append(say([
    ['Slack adjuster and pushrod', 'Mounted securely; no broken, loose or missing parts; pushrod moves no more than 1 inch when pulled by hand (manual adjusters).'],
    ['Brake chamber', 'Not leaking, cracked or dented; mounted securely; no loose or missing clamps.'],
    ['Brake hoses / lines', 'Not cracked, worn or frayed; fittings tight, not leaking.'],
    ['Drum', 'No cracks, dents or holes; no loose or missing bolts; no debris, oil or grease.'],
    ['Linings (where visible)', 'Not dangerously thin.'],
]) + '<p class="small">p. 11-5</p>')

# wheels
H.append('<h2 id="p2-wheel">7 · Wheels and tires</h2>')
sec(2, 'p2-wheel', '7 · Wheels and tires')
H.append(fig('wheels'))
H.append(say([
    ['Rim', 'Not damaged or bent, no welding repairs. Rust trails can mean the rim is loose.'],
    ['Tires (every tire)', 'Tread at least <b>4/32 in</b> on steering-axle tires, <b>2/32 in</b> on all others; even wear; no cuts or damage to tread or sidewalls; valve caps and stems there and undamaged. <b>Check pressure with a tire gauge</b>.'],
    ['Radial / bias-ply', 'Not mixed on the vehicle. (Bus front tires may not be recapped, retreaded or regrooved.)'],
    ['Hub oil seal / axle seal', 'Not leaking; level OK in the sight glass, if fitted.'],
    ['Lug nuts', 'All there, no cracks or bending, no rust trails or shiny threads (signs of looseness); bolt holes not cracked.'],
    ['Spacers (Budd spacing)', 'Not bent, damaged or rusted through; duals evenly apart, nothing stuck between them.'],
]) + '<p class="small">p. 11-6</p>')
H.append(trap('Kicking a tire or hitting it with a mallet to check inflation — <b>no credit</b>. Use a tire gauge (p. 11-6).'))

# side, under
H.append('<h2 id="p2-side">8 · Side of the vehicle, under, and rear</h2>')
sec(2, 'p2-side', '8 · Side of the vehicle, under, and rear')
H.append(say([
    ['Door, hinges, seals; mirrors and brackets; windows', 'Door and hinges work, seals intact; mirrors secure, not too dirty; windows clean and working.'],
    ['Fuel tank and cap', 'Tank secure, cap tight, no leaks.'],
    ['Drive shaft and U-joints', 'Not bent or cracked; U-joints secure.'],
    ['Exhaust', 'No rust or carbon soot (leak signs); no cracks, holes or bad dents; tightly connected and mounted.'],
    ['DEF tank (after-treatment)', 'More than <b>⅛ full</b>; DEF dash indicator works.'],
    ['Frame', 'No cracks, broken welds or holes in rails, cross members, box or floor.'],
    ['Lights and reflectors', 'Clean, working, right color: clearance lights and reflectors <b>red at the rear, amber elsewhere</b>. Brake lights, turn signals and 4-way flashers checked <b>separately</b>.'],
    ['Rear: splash guards, doors, ties, lift', 'Splash guards secure; cargo doors open, close and latch from outside; ties, straps, chains and binders secure; a cargo lift has no leaks or damaged parts (explain how you’d test it) and is fully retracted and latched.'],
]) + '<p class="small">pp. 11-3, 11-6 – 11-7</p>')

# coupling
H.append('<h2 id="p2-coup">9 · Coupling system (on every Class A test)</h2>')
sec(2, 'p2-coup', '9 · Coupling system (on every Class A test)')
H.append(fig('coupling', 'Look into the fifth-wheel gap from behind: jaws closed around the kingpin <b>shank</b>, trailer lying flat on the skid plate with <b>no gap</b> (pp. 6-16, 11-8).'))
H.append(say([
    ['Air lines and electrical cord (tractor to trailer)', 'Listen for air leaks. Lines not cut, chafed, spliced or worn (no steel braid showing), not tangled, pinched or dragging. Electrical plug firmly seated and locked at both ends.'],
    ['Glad hands (trailer front)', 'Locked, undamaged, not leaking; seals in good shape.'],
    ['Catwalk and steps', 'Solid, clear of objects, bolted to the frame.'],
    ['Fifth-wheel mounting', 'No loose or missing brackets, clamps, bolts or nuts; fifth wheel and slide solidly attached.'],
    ['Skid plate and platform', 'Greased, securely mounted, bolts and pins in place; platform not cracked or broken.'],
    ['Release arm / hitch release lever', 'In place and secure; engaged, with the safety latch in place.'],
    ['Locking jaws', 'Fully closed around the kingpin.'],
    ['Kingpin, apron, gap', 'Kingpin not bent or damaged; apron not bent, cracked or broken; trailer lying flat on the skid plate — no gap; kingpin lock checked.'],
    ['Sliding fifth wheel', 'Locking pins there and fully engaged, no air leaks if air-powered; positioned so the tractor frame clears the landing gear in turns.'],
]) + '<p class="small">pp. 11-7 – 11-8. Pintle hook or ball hitch instead: no missing or broken parts, no broken welds; safety chains secure, no kinks, not too much slack (p. 11-8).</p>')

# trailer
H.append('<h2 id="p2-trl">10 · Trailer</h2>')
sec(2, 'p2-trl', '10 · Trailer')
H.append(say([
    ['Header board (if any), front wall', 'Secure, undamaged, strong enough to hold cargo; tarp carrier fastened; enclosed trailer front: no cracks, bulges, holes or missing rivets.'],
    ['Landing gear', '<b>Fully raised</b>, no missing parts, crank handle secure; frame and pads undamaged; no leaks if powered.'],
    ['Tandem release arm and locking pins', 'Locked in place and secured.'],
    ['Doors, ties, lift; frame', 'Same as for the truck.'],
    ['Trailer wheels, suspension, brakes, splash guards', 'Same checks as the tractor axles.'],
]) + '<p class="small">p. 11-10</p>')
H.append(box('ex', 'Bus or school bus instead?', '<p>Also: passenger entry door and lift, <b>demonstrate at least 1 emergency exit</b> and point out the rest, seats, and the bus sitting level with no air leaks. School buses add the emergency kit and body-fluid cleanup kit, warning-light indicators, student mirrors and stop arm (pp. 11-9 – 11-11).</p>'))
H.append(videos('California CDL class A pre-trip inspection full', 'CDL air brake check leak test low air warning', 'CDL in-cab inspection'))
H.append('</section>')

# ================================================================ part 3 basic control
H.append('<section class="part" id="p3"><span class="ptag">PART 3</span><h1>Test 2: Basic control skills</h1>')
sec(1, 'p3', 'Part 3 — Test 2: Basic control skills')
H.append('<p class="lead">Backing exercises in a course marked with <b>lines and cones</b>. You’ll get <b>some</b> of these 6: straight line backing, offset back right, offset back left, parallel park driver side, parallel park conventional, alley dock (p. 12-1).</p>')
H.append('<h2 id="p3-score">How it’s scored</h2>')
sec(2, 'p3-score', 'How it’s scored')
H.append(tbl(['Scored', 'Counts as', 'Rule'], [
    ['<b>Encroachment</b>', 'Any part of the vehicle touches or crosses a line or cone', '<b>Each one = 1 error</b> — the mirror brushing a cone counts'],
    ['<b>Pull-up</b>', 'Stopping <b>and</b> driving forward to fix position', 'First pull-ups aren’t penalized; too many are errors. Just stopping (no change of direction) is <b>not</b> a pull-up'],
    ['<b>Look</b>', 'Opening the door, leaving the seat, or walking to the back of a bus', 'Max <b>2</b> per exercise; straight line backing only <b>1</b>'],
    ['<b>Final position</b>', 'Where the examiner told you to end', 'Not in position = penalized, could fail'],
]) + '<p class="small">p. 12-1</p>')
H.append('<h3>Getting out for a look — do it safely every time</h3>' + fig('three_points') + '<p>Not securing the vehicle or not getting out safely <b>may be an automatic failure</b> (p. 12-1). On a bus, keep a firm hold on the handrail.</p>')
H.append('<h2 id="p3-steer">Steering when backing a trailer</h2>')
sec(2, 'p3-steer', 'Steering when backing a trailer')
H.append(fig('backing_wheel'))
H.append('''<ul><li>Handbook rule: turn the steering wheel the <b>opposite</b> way from where you want the trailer to go; once it starts to turn, turn back to <b>follow</b> it (pp. 2-10, 6-4).</li>
<li><b>Correct drift right away</b> by turning toward the drift. Back <b>slowly</b>, in the <b>lowest reverse gear</b>. Check both mirrors often (p. 2-10).</li>
<li>Back toward the <b>driver’s side</b> whenever you can — you see that side through your own window (p. 2-10).</li></ul>''')
H.append(tip('<p>Think of the hand at the bottom of the wheel: move it the way you want the trailer’s rear to go. Make small inputs and wait for the trailer to respond. When in doubt, stop, get out and look (it costs a look, not a crash), or pull up and start over.</p>'))

EX = [
    ('straight', 'Straight line backing', '''<ol><li>Line the rig up straight in the lane before you start.</li><li>Back slowly in the lowest reverse gear.</li><li>Watch both mirrors; the gap between the trailer and each cone row should stay the same.</li><li>If the trailer drifts toward one side, turn toward the drift a little, then straighten.</li></ol>
<p><b>Only 1 look</b> allowed on this exercise (p. 12-1).</p>'''),
    ('offset_right', 'Offset back / right', '''<ol><li>Drive straight forward out of your lane toward the <b>outer boundary</b>.</li><li>Back into the lane to your <b>right rear</b>.</li><li>Keep backing until the <b>front of the vehicle is past the first set of cones</b>, touching no line or cone.</li></ol>
<p>This is the harder side: you back toward your blind (right) side, so use the right mirror constantly and take a look if unsure (pp. 2-10, 12-2).</p>'''),
    ('offset_left', 'Offset back / left', '''<ol><li>Drive straight forward toward the outer boundary.</li><li>Back into the lane to your <b>left rear</b> — the sight side, so you can watch the trailer through your window.</li><li>Finish with the front of the vehicle past the first set of cones.</li></ol><p class="small">p. 12-2</p>'''),
    ('pp_driver', 'Parallel park — driver side', '''<ol><li>The space is on your <b>left</b>.</li><li>Drive <b>past</b> the space, parallel to it.</li><li>Back in without crossing the front, side or rear boundaries.</li><li>Finish with the <b>entire vehicle completely inside</b> the space.</li></ol><p class="small">p. 12-2</p>'''),
    ('pp_conv', 'Parallel park — conventional', '''<ol><li>The space is on your <b>right</b> (the blind side).</li><li>Drive past the space, parallel to it, then back in.</li><li>Finish with the entire vehicle inside, no line or cone touched.</li></ol><p class="small">p. 12-2. Trap: “conventional” is the <b>right</b> side; “driver side” is the left.</p>'''),
    ('alley', 'Alley dock', '''<ol><li>Drive <b>past</b> the alley and set the rig <b>parallel to the outer boundary</b>.</li><li><b>Sight-side</b> back into the alley (it’s at a right angle, like a loading dock).</li><li>Finish with the rear <b>within 3 feet</b> of the back of the alley, <b>straight</b>, no line or cone touched.</li></ol>
<p>Example: stopping 5 ft from the back line is not finished (more than 3 ft). Stopping 2 ft away but angled is not finished either — it must be straight (p. 12-2).</p>'''),
]
for key, title, body in EX:
    sid = 'p3-' + key
    H.append(f'<div class="ex" id="{sid}"><h2>{title}</h2>' + fig(key) + body + '</div>')
    sec(2, sid, title)
H.append(trap('Rolling back while you think instead of stopping · too many pull-ups · a 3rd look · getting out without Neutral + parking brake · leaving the trailer angled or part-way out of the final box · relying on a backup camera.'))
H.append(videos('CDL alley dock 90 degree backing', 'CDL offset backing right', 'CDL parallel parking conventional', 'CDL straight line backing tips'))
H.append('</section>')

# ================================================================ part 4 road test
H.append('<section class="part" id="p4"><span class="ptag">PART 4</span><h1>Test 3: Road test</h1>')
sec(1, 'p4', 'Part 4 — Test 3: Road test')
H.append(rule('Pass = <b>no more than 30 errors</b> AND <b>no critical driving error</b>. Wear your seat belt, obey every sign, signal and law, and finish with <b>no accident and no moving violation</b> (p. 13-1).'))
H.append('<p>The examiner gives directions early enough to follow them and will never ask you to drive unsafely. Anything the route lacks (e.g. a railroad crossing) you may be asked to <b>simulate</b> — tell and show what you would do (pp. 13-1, 13-3).</p>')
H.append('<h2 id="p4-turn">Turns</h2>')
sec(2, 'p4-turn', 'Turns')
H.append(fig('turns', 'On a multi-lane road: a left turn ends in the lane <b>directly right of the center line</b>; a right turn ends in the <b>right-most (curb) lane</b> (p. 13-4).'))
H.append(tbl(['Stage', 'Do'], [
    ['When told to turn', 'Check traffic in all directions, signal, get into the correct lane safely.'],
    ['Approaching', 'Signal. Slow smoothly; change gears as needed to keep power. <b>No unsafe coasting</b> (out of gear for more than your vehicle’s length).'],
    ['If you must stop', 'Stop smoothly, fully, <b>behind</b> the stop line, crosswalk or sign. Behind another vehicle, leave a <b>safe gap</b>. Don’t roll. <b>Front wheels straight ahead</b> while waiting.'],
    ['During the turn', 'Check all directions again, both hands on the wheel, watch the mirror so the rear doesn’t hit anything on the inside. Don’t swing into oncoming traffic. Finish in the correct lane. <b>Don’t shift in the turn.</b>'],
    ['After', 'Signal off; get up to traffic speed; move to the right-most lane when safe; check mirrors.'],
]) + '<p class="small">pp. 13-1 – 13-2, 13-4</p>')
H.append(fig('safegap'))
H.append('<h2 id="p4-int">Intersections, city streets, lane changes, expressway</h2>')
sec(2, 'p4-int', 'Intersections, city streets, lane changes, expressway')
H.append('''<ul><li><b>Intersections:</b> check all directions; slow gently, brake smoothly; stop fully behind the line with a safe gap, no rolling. <b>Don’t enter if you can’t clear it.</b> Yield to pedestrians and traffic; don’t change lanes in the intersection; hands on the wheel; check mirrors after (p. 13-2).</li>
<li><b>City (urban business):</b> regular traffic checks, safe following distance, centered in the right-most lane, keep up with traffic but never over the limit (p. 13-2).</li></ul>''')
H.append(fig('lanechange', 'You’ll be told to change lanes to the left, then back to the right (p. 13-2).'))
H.append(tbl(['Expressway', 'Examiner wants'], [
    ['Entering', 'Check traffic, signal, merge smoothly into the proper lane.'],
    ['On it', 'Right lane position, spacing and speed; keep checking traffic in all directions.'],
    ['Exiting', 'Traffic checks, signal, slow smoothly <b>in the exit lane</b>; on the ramp keep slowing inside the lane markings with good spacing.'],
]) + '<p class="small">p. 13-2</p>')
H.append('<h2 id="p4-stop">Stop/start (roadside stop)</h2>')
sec(2, 'p4-stop', 'Stop/start (roadside stop)')
H.append('<p>Pull over as if to get out and check the truck (p. 13-2).</p><div class="two"><div><h4>Stopping</h4><ol class="steps"><li>Check traffic all around.</li><li><b>Right</b> signal; move to the right-most lane or shoulder.</li><li>Slow smoothly, brake evenly, shift as needed; full stop, <b>no coasting</b>.</li><li>Parallel to the curb, out of traffic, not blocking driveways, hydrants, intersections or signs.</li><li><b>Cancel</b> the signal → <b>4-way flashers on</b> → <b>parking brake</b> → Neutral or Park → feet off brake and clutch.</li></ol></div><div><h4>Starting again</h4><ol class="steps"><li>Check mirrors in all directions.</li><li><b>4-ways off</b> → <b>left</b> signal on.</li><li>When clear: release the parking brake, pull <b>straight ahead</b>. <b>Don’t turn the wheel before the vehicle moves.</b></li><li>Check traffic, especially to the <b>left</b>; steer and speed up smoothly into the lane.</li><li>Cancel the left signal.</li></ol></div></div><p class="small">pp. 13-2 – 13-3</p>')
H.append('<h2 id="p4-rr">Curves, railroad crossings, bridges and signs</h2>')
sec(2, 'p4-rr', 'Curves, railroad crossings, bridges and signs')
H.append('<ul><li><b>Curves:</b> slow down <b>before</b> the curve so you don’t brake or shift in it; stay in your lane; keep checking traffic (p. 13-3).</li><li><b>Every railroad crossing:</b> slow, brake smoothly, shift as needed; look and listen; right-most lane on a multi-lane road. While any part is in the crossing: <b>no stopping, no gear changes, no passing, no lane changes</b> (p. 13-3).</li></ul>')
H.append(fig('rr', 'Extra steps for a bus, school bus or placarded vehicle at every non-exempt crossing (p. 13-3). Example: 60 ft away is too far; 10 ft is too close.'))
H.append('<ul><li><b>Overpass:</b> you may be asked the posted <b>clearance (height)</b>. <b>Bridge:</b> the posted <b>weight limit</b>. No bridge on the route? Be ready to name and explain any traffic sign (p. 13-4).</li><li><b>School bus endorsement:</b> you demonstrate loading and unloading students (p. 13-4).</li></ul>')
H.append(tip('Read every clearance and weight sign aloud to yourself as you pass it during practice. It makes the bridge/overpass question easy.'))
H.append('<h2 id="p4-gen">Scored the whole drive</h2>')
sec(2, 'p4-gen', 'Scored the whole drive')
H.append(tbl(['', 'Do', 'Don’t'], [
    ['Clutch', 'Always use it to shift; double-clutch on an unsynchronized transmission', 'Rev or lug the engine; ride the clutch; coast with it in; “pop” it'],
    ['Gears', 'Pick a gear that doesn’t rev or lug', 'Grind or clash; shift in turns or intersections'],
    ['Brakes', 'Smooth, steady pressure', 'Ride or pump; brake harshly'],
    ['Lanes', 'Stop behind lines, crosswalks, signs; right-most lane unless blocked', 'Drive over curbs, sidewalks or lane markings'],
    ['Steering', 'Both hands on the wheel except while shifting', 'Over-steer or under-steer'],
    ['Traffic checks', 'Mirrors regularly; before, during and after intersections; scan busy and pedestrian areas', '—'],
    ['Signals', 'Use properly, when required, at the right time', 'Forget to cancel after a turn or lane change'],
]) + '<p class="small">p. 13-4</p>')
H.append(videos('CDL road test California what examiners look for', 'CDL road test turns tractor trailer right turn', 'CDL double clutching tutorial'))
H.append('</section>')

# ================================================================ part 5 printables
H.append('<section class="part" id="p5"><span class="ptag">PART 5</span><h1>Cards to practice with</h1>')
sec(1, 'p5', 'Part 5 — Cards to practice with')
H.append('<h2 id="p5-air">Air brake check card</h2>')
sec(2, 'p5-air', 'Air brake check card')
H.append(tbl(['#', 'Do', 'Say', 'Pass'], [
    ['1', 'Build air', '“Governor cut out at ___ psi”', '120–140 (or maker’s)'],
    ['2', 'Engine off, parking brake(s) released, foot brake held 1 min', '“Lost ___ psi in one minute; limit ___”', '≤3 single · ≤4 two · ≤6 three+'],
    ['3', 'Key on, fan the brake', '“Warning came on at ___ psi”', 'before 55'],
    ['4', 'Keep fanning', '“Knob(s) popped at ___ psi”', '20–45'],
    ['5', 'Seat belt on, rebuild air, parking brake on (trailer released), gentle tug; then trailer brake only, gentle tug', '“Parking brakes hold”', 'no movement'],
    ['6', 'Roll ~5 mph, brake', '“Stops straight, no pull”', 'no pull'],
]))
H.append('<h2 id="p5-insp">One-page inspection checklist</h2>')
sec(2, 'p5-insp', 'One-page inspection checklist')
H.append('<ul class="check cols">' + ''.join(f'<li>{x}</li>' for x in [
    'Approach: leaks, lean, hazards, brakes set / chocked', 'Leaks, hoses', 'Water pump, alternator, compressor (belt or gear)', 'Oil dipstick', 'Coolant', 'Power steering fluid', 'Washer fluid', 'Belts ½–¾ in', 'Master cylinder (hydraulic)',
    'Safe start: Neutral, clutch in', 'Oil, coolant, air, volt gauges', 'Mirrors, windshield', 'Wipers, washers', 'Horn(s)', 'Heater, defroster', 'Seat belt', 'Fuses; 3 triangles; extinguisher', 'Dash indicators', 'Air checks 1–6 (card above)',
    'Lights and reflectors: amber front/sides, red rear', 'Steering box, hoses, linkage', 'Springs (¼ rule), mounts, shocks', 'Slack adjuster ≤1 in, chamber, hoses, drum, linings', 'Rims, tires 4/32 & 2/32, gauge', 'Hub seal, lug nuts, spacers',
    'Door, mirrors, fuel tank, cap', 'Drive shaft, U-joints, exhaust, DEF >⅛', 'Frame', 'Air/electric lines, glad hands', 'Catwalk, steps', 'Fifth-wheel mounts, skid plate', 'Release arm + latch; jaws on shank; no gap', 'Slider pins', 'Header board, landing gear up, tandem pins', 'Trailer axles', 'Rear: splash guards, doors, lights'
]) + '</ul>')
H.append('<h2 id="p5-log">Backing practice log</h2>')
sec(2, 'p5-log', 'Backing practice log')
H.append('<p>Score yourself like the examiner does. Aim for 0 encroachments, few pull-ups, and at most 2 looks (1 on straight line backing), several times in a row.</p>')
H.append(tbl(['Date', 'Exercise', 'Encroachments', 'Pull-ups', 'Looks', 'Final position OK?'], [['&nbsp;'] * 6 for _ in range(14)], 'log'))
H.append('<h2 id="p5-day">Day-before checklist</h2>')
sec(2, 'p5-day', 'Day-before checklist')
H.append('<ul class="check">' + ''.join(f'<li>{x}</li>' for x in [
    'CLP held 14+ days; written tests passed; training completed (confirm ELDT with your school)',
    'Test vehicle: brake lights, 4-way flashers, turn signals and horn all work; no labels on parts; dash cam off or covered',
    'Emergency equipment on board: fuses, 3 triangles, charged and mounted extinguisher',
    'Tire gauge in the cab (no credit for kicking tires)',
    'Clean handbook Section 11 inspection guide — no notes written on it',
    'Say the whole inspection out loud once, in order, without looking',
    'Run through the air brake card once with the real gauges',
    'Sleep. The road test is 45–60 minutes of full attention.',
]) + '</ul>')
H.append('</section>')

# ================================================================ css + assemble
CSS = (HERE.parent / 'study-guide/print.css').read_text() if (HERE.parent / 'study-guide/print.css').exists() else ''
CSS += '''
.fig { margin: 8pt 0 10pt; break-inside: avoid; }
.fig svg { margin: 0 0 4pt; }
.legh { font-family: 'Barlow Condensed'; font-weight: 700; font-size: 10.5pt; color: var(--acc); text-transform: uppercase; letter-spacing: .05em; }
.leg { columns: 2; column-gap: 16pt; font-size: 9.2pt; margin: 2pt 0 4pt; padding-left: 20pt; }
.leg li { break-inside: avoid; margin: 1pt 0 3pt; }
.leg li::marker { font-weight: 700; color: #563d00; }
figcaption { font-size: 9.2pt; color: var(--ink2); }
table.say td:first-child { width: 32%; font-weight: 700; }
.vid { font-size: 9pt; } .vid a { color: var(--blue); }
.cols { columns: 2; column-gap: 18pt; font-size: 9.4pt; } .cols li { break-inside: avoid; }
table.log td { height: 22pt; }
.ex { break-inside: avoid; }
.two .fig svg { max-height: 6.2in; }
'''


def font_face():
    rows = [('Atkinson Hyperlegible', 'atkinson-hyperlegible/files/atkinson-hyperlegible-latin-400-normal.woff2', 400, 'normal'),
            ('Atkinson Hyperlegible', 'atkinson-hyperlegible/files/atkinson-hyperlegible-latin-700-normal.woff2', 700, 'normal'),
            ('Atkinson Hyperlegible', 'atkinson-hyperlegible/files/atkinson-hyperlegible-latin-400-italic.woff2', 400, 'italic'),
            ('Barlow Condensed', 'barlow-condensed/files/barlow-condensed-latin-600-normal.woff2', 600, 'normal'),
            ('Barlow Condensed', 'barlow-condensed/files/barlow-condensed-latin-700-normal.woff2', 700, 'normal')]
    return '\n'.join(f"@font-face{{font-family:'{f}';src:url('{(FONTS / p).as_uri()}') format('woff2');font-weight:{w};font-style:{s}}}" for f, p, w, s in rows)


tl = ''.join(f'<li class="{"s" if l == 2 else ""}"><a class="t" href="#{sid}">{html.escape(tt)}</a><span>{PAGEMAP.get(sid, "")}</span></li>' for l, sid, tt in toc)
H[TOC_AT] = '<section class="part" id="toc"><h1>Contents</h1><ul class="toc">' + tl + '</ul></section>'
doc = f'<!doctype html><html lang="en"><head><meta charset="utf-8"><title>CA CDL Skills Test Guide — Behind the Wheel</title><style>{font_face()}\n{CSS}</style></head><body>{"".join(H)}</body></html>'
(HERE / 'guide.html').write_text(doc)
(HERE / 'toc.json').write_text(json.dumps(toc))
print('wrote guide.html', len(doc) // 1024, 'KB')
