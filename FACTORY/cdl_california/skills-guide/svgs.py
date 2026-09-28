"""Original vector illustrations for the skills-test guide.

Drawn from scratch in code (not traced from the handbook). The DL 650 figures were used
only as a reference to check part names and the direction of each backing exercise.
All drawings use a 660-unit-wide viewBox so 12-unit text prints at about 9 pt.
"""
from html import escape

INK = '#14201a'; INK2 = '#4b5a51'; LINE = '#d3dccf'; ACC = '#0b5d3b'; ACCS = '#dcebe1'
RED = '#b3261e'; REDS = '#f9dedc'; BLUE = '#1f5fa8'; BLUES = '#dde9f7'; AMB = '#e7a600'; AMBS = '#fdf0c7'
STEEL = '#8a969c'; STEEL2 = '#c3ccd0'; TIRE = '#2b2f31'; PAPER = '#f3f5f0'
FONT = "font-family=\"Atkinson Hyperlegible, DejaVu Sans, sans-serif\""


def svg(w, h, label, body, fs=12, x0=0):
    return (f'<svg viewBox="{x0} 0 {w} {h}" role="img" aria-label="{escape(label)}" xmlns="http://www.w3.org/2000/svg">'
            f'<g {FONT} font-size="{fs}" fill="{INK}">{body}</g></svg>')


def t(x, y, s, anchor='start', size=None, weight=None, fill=None, italic=False):
    a = f' text-anchor="{anchor}"' if anchor != 'start' else ''
    a += f' font-size="{size}"' if size else ''
    a += f' font-weight="{weight}"' if weight else ''
    a += f' fill="{fill}"' if fill else ''
    a += ' font-style="italic"' if italic else ''
    return f'<text x="{x}" y="{y}"{a}>{s}</text>'


LEG = []  # legend of the diagram being drawn: list of label strings


def call(px, py, tx=None, ty=None, s='', anchor=None, n=None, color=ACC, bold=True):
    """Numbered marker on a part; the label goes into LEG (printed as a key next to the figure)."""
    LEG.append(s)
    k = len(LEG)
    lead = ''
    if tx is not None:  # badge placed away from the part, joined by a leader line
        lead = (f'<line x1="{px}" y1="{py}" x2="{tx}" y2="{ty}" stroke="{INK}" stroke-width="1.3"/>'
                f'<circle cx="{px}" cy="{py}" r="3" fill="{INK}"/>')
        px, py = tx, ty
    return (lead + f'<circle cx="{px}" cy="{py}" r="10" fill="{AMB}" stroke="#fff" stroke-width="2"/>'
            + t(px, py + 4, str(k), 'middle', size=11, weight=700))


def wheel(cx, cy, r=26, hub=True):
    s = f'<circle cx="{cx}" cy="{cy}" r="{r}" fill="{TIRE}"/><circle cx="{cx}" cy="{cy}" r="{r*0.55:.1f}" fill="{STEEL2}" stroke="{STEEL}"/>'
    if hub:
        s += f'<circle cx="{cx}" cy="{cy}" r="{r*0.2:.1f}" fill="{STEEL}"/>'
    return s


def cone(x, y, s=9):
    return f'<rect x="{x - s/2}" y="{y - s/2}" width="{s}" height="{s}" rx="2" fill="{AMB}" stroke="{AMBS}" stroke-width=".8"/>'


# ------------------------------------------------------------------ 1. side view anatomy
def anatomy():
    b = []
    # trailer box
    b.append(f'<rect x="250" y="70" width="700" height="165" rx="3" fill="#eef1ec" stroke="{INK2}" stroke-width="1.5"/>')
    b.append(f'<rect x="250" y="235" width="700" height="10" fill="{STEEL}"/>')  # trailer frame rail
    # trailer lights: amber front-top, red rear-top, amber mid side marker, red rear lamp
    b.append(f'<rect x="254" y="74" width="8" height="6" fill="{AMB}"/><rect x="938" y="74" width="8" height="6" fill="{RED}"/>')
    b.append(f'<rect x="596" y="224" width="8" height="6" fill="{AMB}"/><rect x="940" y="222" width="8" height="8" fill="{RED}"/>')
    b.append(f'<rect x="300" y="208" width="640" height="4" fill="{RED}" opacity=".55"/>')  # reflective tape
    # landing gear
    b.append(f'<rect x="380" y="245" width="8" height="46" fill="{STEEL}"/><rect x="372" y="289" width="24" height="6" fill="{INK2}"/>')
    b.append(f'<line x1="392" y1="252" x2="408" y2="252" stroke="{INK2}" stroke-width="3"/>')  # crank handle
    # trailer tandem + slider rail
    b.append(f'<rect x="790" y="245" width="150" height="10" fill="{STEEL2}" stroke="{STEEL}"/>')
    b.append(f'<circle cx="812" cy="250" r="3" fill="{INK}"/><circle cx="832" cy="250" r="3" fill="{INK}"/>')
    b.append(wheel(830, 282) + wheel(895, 282))
    b.append(f'<rect x="920" y="262" width="8" height="40" fill="{INK}"/>')  # mud flap
    # tractor frame
    b.append(f'<rect x="30" y="236" width="360" height="12" fill="{STEEL}"/>')
    # hood + cab
    b.append(f'<path d="M22 150 L30 132 L110 124 L110 236 L22 236 Z" fill="{ACC}" stroke="{INK}" stroke-width="1.5"/>')
    b.append(f'<path d="M110 40 L215 40 L222 236 L110 236 Z" fill="{ACC}" stroke="{INK}" stroke-width="1.5"/>')
    b.append(f'<path d="M118 52 L200 52 L204 118 L118 118 Z" fill="#cfe3ec" stroke="{INK}"/>')  # door window
    b.append(f'<rect x="206" y="60" width="6" height="30" fill="{INK}"/><rect x="100" y="70" width="12" height="36" fill="{INK}"/>')  # mirror + arm
    b.append(f'<rect x="14" y="222" width="18" height="20" fill="{STEEL}"/>')  # bumper
    b.append(f'<rect x="24" y="160" width="10" height="8" fill="#fff6c8" stroke="{INK}"/>')  # headlight
    # steps / battery box, fuel tank, DEF
    b.append(f'<rect x="126" y="244" width="28" height="30" fill="{STEEL2}" stroke="{STEEL}"/>')
    b.append(f'<rect x="156" y="248" width="66" height="30" rx="14" fill="#dfe5e8" stroke="{STEEL}"/>')
    b.append(f'<rect x="224" y="252" width="22" height="24" rx="4" fill="{BLUES}" stroke="{BLUE}"/>')
    # exhaust stack + catwalk
    b.append(f'<rect x="226" y="30" width="9" height="206" fill="{STEEL}"/>')
    b.append(f'<rect x="236" y="226" width="54" height="6" fill="{INK2}"/>')
    # air lines + electrical coiled from cab to trailer front
    b.append(f'<path d="M222 150 C 240 150, 236 170, 252 172" fill="none" stroke="{RED}" stroke-width="3"/>')
    b.append(f'<path d="M222 160 C 240 160, 236 184, 252 186" fill="none" stroke="{BLUE}" stroke-width="3"/>')
    b.append(f'<path d="M222 140 C 240 140, 236 158, 252 160" fill="none" stroke="{INK}" stroke-width="2.5"/>')
    b.append(f'<rect x="246" y="156" width="8" height="34" fill="{STEEL}"/>')  # glad-hand panel
    # fifth wheel + kingpin
    b.append(f'<rect x="276" y="228" width="72" height="8" fill="{INK2}"/><rect x="306" y="236" width="12" height="8" fill="{INK}"/>')
    # wheels: steer, drive tandem
    b.append(wheel(72, 262) + wheel(296, 262) + wheel(356, 262))
    b.append(f'<rect x="384" y="244" width="8" height="38" fill="{INK}"/>')  # tractor mud flap
    # labels (above)
    L = [
        (28, 164, 20, 22, 'Headlights, turn signals', 'start'),
        (206, 74, 160, 22, 'Mirrors', 'start'),
        (230, 60, 262, 22, 'Exhaust', 'start'),
        (258, 77, 330, 40, 'Clearance light: amber at front', 'start'),
        (942, 77, 940, 40, 'Red at rear', 'end'),
        (250, 172, 256, 130, 'Glad hands + electrical plug', 'start'),
        (600, 226, 560, 190, 'Side marker (amber)', 'end'),
        (700, 210, 760, 190, 'Reflective tape', 'start'),
    ]
    for px, py, tx, ty, s, a in L:
        b.append(call(px, py, s=s))
    # labels (below)
    B = [
        (72, 262, 30, 330, 'Steering axle: tires 4/32 in', 'start'),
        (140, 256, 118, 352, 'Steps, battery box', 'start'),
        (188, 262, 200, 312, 'Fuel tank + cap', 'start'),
        (234, 262, 250, 372, 'DEF tank (if equipped)', 'start'),
        (312, 232, 380, 330, 'Fifth wheel + kingpin', 'start'),
        (326, 272, 330, 352, 'Drive axles: tires 2/32 in', 'start'),
        (384, 290, 520, 312, 'Landing gear, crank handle', 'start'),
        (820, 250, 700, 330, 'Tandem slider + locking pins', 'end'),
        (924, 290, 940, 352, 'Splash guards', 'end'),
        (862, 282, 870, 372, 'Trailer axles', 'end'),
    ]
    for px, py, tx, ty, s, a in B:
        b.append(call(px, py, s=s))
    return svg(960, 310, 'Side view of a tractor-semitrailer with the parts you name on the inspection test', ''.join(b))


# ------------------------------------------------------------------ 2. walkaround route (top view)
def route():
    b = []
    # tractor (top view) facing up; trailer below
    b.append(f'<rect x="290" y="30" width="80" height="60" rx="8" fill="{ACC}"/>')  # hood
    b.append(f'<rect x="280" y="90" width="100" height="70" rx="6" fill="{ACC}" stroke="{INK}"/>')  # cab
    b.append(f'<rect x="300" y="160" width="60" height="50" fill="{STEEL}"/>')  # frame
    b.append(f'<rect x="270" y="190" width="120" height="420" rx="4" fill="#eef1ec" stroke="{INK2}" stroke-width="1.5"/>')  # trailer
    for (x, y) in [(270, 50), (390, 50), (270, 185), (390, 185), (270, 215), (390, 215), (262, 530), (398, 530), (262, 570), (398, 570)]:
        b.append(f'<rect x="{x - 8}" y="{y - 14}" width="16" height="28" rx="3" fill="{TIRE}"/>')
    steps = [
        (330, 20, 'Front of vehicle, lights, engine compartment, steering', 'middle'),
        (300, 50, 'Steering axle: suspension, brakes, tires', 'end'),
        (330, 128, 'Engine start + in-cab checks (air brake checks)', 'middle'),
        (285, 125, 'Driver door, fuel area', 'end'),
        (330, 185, 'Under vehicle: drive shaft, exhaust, frame', 'end'),
        (300, 228, 'Drive axles: suspension, brakes, tires', 'end'),
        (330, 214, 'Coupling devices (tractor + trailer)', 'start'),
        (390, 330, 'Side of tractor + trailer, lights, reflectors', 'start'),
        (270, 330, 'Trailer front, side, frame, landing gear, tandem release', 'end'),
        (262, 552, 'Trailer axles: suspension, brakes, tires', 'end'),
        (330, 612, 'Rear of trailer, lights, reflectors', 'middle'),
    ]
    b.append(t(262, 420, 'driver (left) side', 'end', fill=INK2, italic=True) + t(398, 420, 'right side', 'start', fill=INK2, italic=True))
    for i, (x, y, s, a) in enumerate(steps, 1):
        LEG.append(s)
        b.append(f'<circle cx="{x}" cy="{y}" r="11" fill="{AMB}"/>' + t(x, y + 4, str(i), 'middle', weight=700))
        continue
        if a == 'middle':
            b.append(f'<circle cx="{x}" cy="{y}" r="11" fill="{AMB}"/>' + t(x, y + 4, str(i), 'middle', weight=700))
            if i == 3:
                b.append(t(x, y + 26, 'Engine start +', 'middle', weight=700, fill='#fff', size=11))
                b.append(t(x, y + 39, 'in-cab checks', 'middle', weight=700, fill='#fff', size=11))
                b.append(t(400, y + 4, '← includes the air brake checks', 'start', weight=700))
            else:
                b.append(t(x, y + 28 if i == 11 else y - 16, s, 'middle', weight=700))
        else:
            b.append(f'<circle cx="{x}" cy="{y}" r="11" fill="{AMB}"/>' + t(x, y + 4, str(i), 'middle', weight=700))
            b.append(t(cx + (4 if a == 'start' else -4), y + 4, s, a, weight=700))
    return svg(420, 640, 'Suggested inspection route for a Class A combination, numbered 1 to 11', ''.join(b), x0=120)


# ------------------------------------------------------------------ 3. engine compartment
def engine():
    b = []
    b.append(f'<rect x="170" y="70" width="320" height="210" rx="14" fill="#e3e7e4" stroke="{INK2}" stroke-width="1.5"/>')
    b.append(t(330, 262, 'engine (front view)', 'middle', fill=INK2, italic=True))
    P = {'crank': (330, 230, 30), 'water': (330, 140, 22), 'alt': (430, 110, 20), 'ps': (230, 110, 18), 'comp': (220, 200, 22)}
    belt = f'M{330-30} 230 L{230-18} 110 A18 18 0 0 1 {230+14} 98 L{330-20} 128 A22 22 0 0 1 {330+20} 128 L{430-16} 98 A20 20 0 0 1 {430+20} 112 L{330+30} 232 A30 30 0 0 1 {330-30} 230'
    b.append(f'<path d="{belt}" fill="none" stroke="{INK}" stroke-width="5" stroke-linejoin="round"/>')
    for k, (x, y, r) in P.items():
        b.append(f'<circle cx="{x}" cy="{y}" r="{r}" fill="{STEEL2}" stroke="{INK}" stroke-width="1.5"/><circle cx="{x}" cy="{y}" r="{r/3:.0f}" fill="{STEEL}"/>')
    # dipstick, reservoirs
    b.append(f'<rect x="392" y="170" width="6" height="70" fill="{STEEL}"/><circle cx="395" cy="166" r="7" fill="{AMB}"/>')
    b.append(f'<rect x="520" y="80" width="70" height="90" rx="8" fill="#f7fbf8" stroke="{INK2}"/><rect x="546" y="110" width="18" height="44" fill="{BLUES}" stroke="{BLUE}"/><line x1="546" y1="126" x2="564" y2="126" stroke="{BLUE}" stroke-width="2"/>')
    b.append(f'<rect x="70" y="84" width="56" height="60" rx="8" fill="#f7fbf8" stroke="{INK2}"/>')
    b.append(f'<rect x="520" y="200" width="70" height="60" rx="8" fill="{BLUES}" stroke="{INK2}"/>')
    b.append(f'<rect x="70" y="200" width="60" height="40" rx="4" fill="#f7fbf8" stroke="{INK2}"/><rect x="86" y="190" width="28" height="10" fill="{STEEL}"/>')
    # belt play callout
    b.append(f'<path d="M270 172 l-10 -6 m10 6 l-6 10" stroke="{RED}" stroke-width="2" fill="none"/>')
    L = [
        (330, 140, 330, 40, 'Water pump', 'middle'),
        (430, 110, 470, 40, 'Alternator (wires fastened)', 'start'),
        (230, 110, 200, 40, 'Power steering pump', 'end'),
        (220, 200, 150, 300, 'Air compressor', 'end'),
        (395, 166, 430, 300, 'Oil dipstick', 'start'),
        (555, 126, 610, 60, 'Coolant sight glass', 'end'),
        (98, 114, 60, 60, 'Power steering fluid', 'start'),
        (555, 230, 610, 300, 'Washer fluid, cap on', 'end'),
        (100, 220, 20, 300, 'Brake master cylinder (hydraulic)', 'start'),
        (265, 170, 250, 326, 'Belt: ½–¾ in play at the center', 'middle'),
    ]
    for px, py, tx, ty, s, a in L:
        b.append(call(px, py, s=s))
    return svg(660, 290, 'Engine compartment: belt-driven parts, fluid checks and belt play', ''.join(b))


# ------------------------------------------------------------------ 4. steering
def steering():
    b = []
    b.append(f'<rect x="60" y="40" width="540" height="14" fill="{STEEL}"/><rect x="60" y="150" width="540" height="14" fill="{STEEL}" opacity=".35"/>')
    b.append(t(60, 34, 'frame rail', 'start', fill=INK2, size=11))
    b.append(f'<rect x="440" y="26" width="46" height="40" rx="4" fill="{INK2}"/>')  # gear box
    b.append(f'<line x1="463" y1="66" x2="470" y2="118" stroke="{INK}" stroke-width="7" stroke-linecap="round"/>')  # pitman arm
    b.append(f'<line x1="470" y1="118" x2="560" y2="200" stroke="{INK}" stroke-width="6" stroke-linecap="round"/>')  # drag link
    # knuckles + wheels (top view)
    for x in (110, 560):
        b.append(f'<rect x="{x - 22}" y="200" width="44" height="100" rx="8" fill="{TIRE}"/>')
        b.append(f'<circle cx="{x + (26 if x == 110 else -26)}" cy="250" r="9" fill="{STEEL}"/>')
    b.append(f'<line x1="136" y1="250" x2="534" y2="250" stroke="{STEEL}" stroke-width="10"/>')  # axle beam
    b.append(f'<line x1="534" y1="250" x2="560" y2="200" stroke="{INK}" stroke-width="5"/>')  # steering arm
    b.append(f'<line x1="136" y1="262" x2="150" y2="300" stroke="{INK}" stroke-width="5"/><line x1="534" y1="262" x2="520" y2="300" stroke="{INK}" stroke-width="5"/>')
    b.append(f'<line x1="150" y1="300" x2="520" y2="300" stroke="{INK}" stroke-width="5"/>')  # tie rod
    b.append(f'<line x1="463" y1="30" x2="420" y2="10" stroke="{INK}" stroke-width="4"/>')  # shaft
    L = [
        (463, 40, 360, 30, 'Steering gear box: mounted, no leaks, no missing nuts/bolts', 'end'),
        (468, 100, 380, 92, 'Pitman arm', 'end'),
        (515, 160, 610, 150, 'Drag link', 'start'),
        (548, 222, 612, 216, 'Steering arm', 'start'),
        (534, 250, 612, 268, 'Knuckle / spindle', 'start'),
        (330, 300, 330, 336, 'Tie rod: not worn or cracked; joints tight; cotter keys in place', 'middle'),
    ]
    for px, py, tx, ty, s, a in L:
        b.append(call(px, py, s=s))
    b.append(t(110, 330, 'front axle, seen from above', 'middle', fill=INK2, italic=True, size=11))
    return svg(660, 360, 'Steering linkage seen from above: gear box, pitman arm, drag link, steering arm, knuckle, tie rod', ''.join(b))


# ------------------------------------------------------------------ 5. suspension
def suspension():
    b = []
    b.append(f'<rect x="40" y="40" width="580" height="18" fill="{STEEL}"/>')
    b.append(t(40, 34, 'frame', 'start', fill=INK2, size=11))
    b.append(f'<path d="M110 58 L130 58 L126 96 L114 96 Z" fill="{INK2}"/>')  # front hanger
    b.append(f'<path d="M520 58 L540 58 L536 80 L524 80 Z" fill="{INK2}"/><line x1="530" y1="80" x2="522" y2="104" stroke="{INK}" stroke-width="5"/>')  # rear hanger+shackle
    for i in range(7):
        w = 400 - i * 46
        x0 = 320 - w / 2
        y = 98 + i * 7
        b.append(f'<path d="M{x0} {y} Q 320 {y + 26} {x0 + w} {y}" fill="none" stroke="{INK if i != 4 else RED}" stroke-width="5"/>')
    b.append(f'<line x1="120" y1="96" x2="124" y2="100" stroke="{INK}" stroke-width="4"/>')
    b.append(f'<rect x="280" y="146" width="80" height="40" rx="6" fill="{STEEL2}" stroke="{INK}"/>')  # axle
    for x in (290, 350):
        b.append(f'<line x1="{x}" y1="104" x2="{x}" y2="190" stroke="{AMB}" stroke-width="4"/>')
    b.append(f'<line x1="430" y1="60" x2="400" y2="160" stroke="{STEEL}" stroke-width="10" stroke-linecap="round"/>')  # shock
    b.append(f'<line x1="130" y1="170" x2="280" y2="170" stroke="{INK2}" stroke-width="6"/>')  # torque rod
    L = [
        (120, 80, 60, 26, 'Spring hanger', 'start'),
        (526, 94, 600, 26, 'Shackle', 'end'),
        (330, 126, 470, 214, 'Leaf spring: no missing, shifted, cracked or broken leaves', 'start'),
        (290, 130, 250, 226, 'U-bolts', 'end'),
        (320, 180, 330, 250, 'Axle', 'middle'),
        (416, 110, 470, 126, 'Shock absorber: secure, not leaking', 'start'),
        (180, 170, 70, 214, 'Torque rod', 'start'),
    ]
    for px, py, tx, ty, s, a in L:
        b.append(call(px, py, s=s))
    return svg(660, 210, 'Leaf-spring suspension: hanger, shackle, leaves, U-bolts, axle, shock absorber, torque rod', ''.join(b))


# ------------------------------------------------------------------ 6. brakes
def brakes():
    b = []
    b.append(f'<circle cx="220" cy="170" r="110" fill="#e6eaec" stroke="{INK}" stroke-width="2"/>')  # drum
    b.append(f'<circle cx="220" cy="170" r="100" fill="#fff" stroke="{STEEL}" stroke-width="1"/>')
    b.append(f'<path d="M140 125 A92 92 0 0 1 300 125" fill="none" stroke="{AMB}" stroke-width="12"/><path d="M140 215 A92 92 0 0 0 300 215" fill="none" stroke="{AMB}" stroke-width="12"/>')
    b.append(f'<path d="M126 128 A100 100 0 0 1 314 128" fill="none" stroke="{INK2}" stroke-width="6"/><path d="M126 212 A100 100 0 0 0 314 212" fill="none" stroke="{INK2}" stroke-width="6"/>')
    b.append(f'<path d="M205 150 C 250 150, 190 190, 235 190" fill="none" stroke="{INK}" stroke-width="10" stroke-linecap="round"/>')  # S-cam
    b.append(f'<line x1="220" y1="170" x2="420" y2="170" stroke="{STEEL}" stroke-width="10"/>')  # camshaft
    b.append(f'<rect x="410" y="110" width="22" height="80" rx="6" fill="{INK2}"/>')  # slack adjuster
    b.append(f'<line x1="421" y1="120" x2="530" y2="120" stroke="{INK}" stroke-width="5"/>')  # pushrod
    b.append(f'<rect x="530" y="80" width="80" height="80" rx="30" fill="{STEEL2}" stroke="{INK}" stroke-width="2"/>')  # chamber
    b.append(f'<path d="M610 120 C 640 120, 640 60, 620 40" fill="none" stroke="{INK}" stroke-width="4"/>')  # hose
    b.append(f'<line x1="480" y1="104" x2="506" y2="104" stroke="{RED}" stroke-width="2"/><path d="M480 98 v12 M506 98 v12" stroke="{RED}" stroke-width="2"/>')
    L = [
        (570, 100, 570, 22, 'Brake chamber: not leaking, cracked or dented; clamps on', 'end'),
        (620, 40, 650, 60, 'Hose', 'end'),
        (480, 120, 470, 236, 'Pushrod', 'middle'),
        (421, 160, 520, 260, 'Slack adjuster: nothing broken, loose or missing', 'start'),
        (220, 170, 90, 310, 'S-cam', 'start'),
        (300, 120, 330, 26, 'Brake lining: not dangerously thin', 'middle'),
        (130, 230, 40, 300, 'Drum: no cracks, dents, holes, oil or grease', 'start'),
    ]
    for px, py, tx, ty, s, a in L:
        b.append(call(px, py, s=s))
    b.append(t(492, 96, '≤ 1 in', 'middle', weight=700, fill=RED))
    return svg(660, 300, 'S-cam air brake at a wheel: chamber, pushrod, slack adjuster, S-cam, shoes and drum', ''.join(b))


# ------------------------------------------------------------------ 7. wheels and tires
def wheels():
    b = []
    import math
    cx, cy = 150, 150
    b.append(f'<circle cx="{cx}" cy="{cy}" r="120" fill="{TIRE}"/><circle cx="{cx}" cy="{cy}" r="80" fill="{STEEL2}" stroke="{STEEL}" stroke-width="2"/>')
    b.append(f'<circle cx="{cx}" cy="{cy}" r="34" fill="{STEEL}"/><circle cx="{cx}" cy="{cy}" r="16" fill="{BLUES}" stroke="{BLUE}" stroke-width="2"/>')
    for i in range(10):
        a = i * math.pi / 5
        x, y = cx + 52 * math.cos(a), cy + 52 * math.sin(a)
        b.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="6" fill="{INK2}"/>')
        if i == 1:
            b.append(f'<path d="M{x:.1f} {y + 6:.1f} q 3 16 -1 30" stroke="#a0522d" stroke-width="4" fill="none"/>')
    b.append(f'<rect x="{cx - 70}" y="{cy - 6}" width="18" height="8" fill="{INK}"/>')  # valve stem
    L = [
        (cx + 16, cy + 48, 330, 200, 'Rust trail = nut may be loose', 'start'),
        (cx, cy, 330, 150, 'Hub oil seal / sight glass: level OK, no leaks', 'start'),
        (cx - 62, cy - 2, 330, 250, 'Valve stem + cap present', 'start'),
        (cx + 70, cy - 60, 330, 100, 'Rim: not bent, no welding repairs', 'start'),
        (cx + 100, cy - 70, 330, 50, 'Tire: no cuts, bulges, even wear; check pressure with a gauge', 'start'),
    ]
    for px, py, tx, ty, s, a in L:
        b.append(call(px, py, s=s))
    # tread depth gauge
    b.append(t(510, 40, 'Tread depth (every major groove)', 'middle', weight=700))
    b.append(f'<rect x="400" y="52" width="220" height="40" fill="{TIRE}"/>')
    for x in (440, 510, 580):
        b.append(f'<rect x="{x - 8}" y="52" width="16" height="24" fill="#fff"/>')
    b.append(t(510, 112, 'Steer tires ≥ 4/32 in · others ≥ 2/32 in', 'middle', weight=700, fill=RED))
    # duals cross-section
    b.append(t(510, 170, 'Dual wheels (seen from behind)', 'middle', weight=700))
    b.append(f'<rect x="440" y="182" width="50" height="60" rx="6" fill="{TIRE}"/><rect x="530" y="182" width="50" height="60" rx="6" fill="{TIRE}"/>')
    b.append(f'<rect x="490" y="200" width="40" height="24" fill="{STEEL}"/>')
    b.append(t(510, 262, 'Evenly spaced, nothing stuck between', 'middle', size=11.5))
    return svg(660, 290, 'Wheel, rim, lug nuts, hub, valve stem, tread depth and dual spacing', ''.join(b))


# ------------------------------------------------------------------ 8. coupling close-up
def coupling():
    b = []
    # trailer body + upper plate (apron)
    b.append(f'<rect x="40" y="20" width="580" height="50" fill="#eef1ec" stroke="{INK2}"/>')
    b.append(f'<rect x="40" y="70" width="580" height="14" fill="{STEEL}" stroke="{INK}"/>')
    # fifth wheel top plate touching apron (no gap), cut in section around kingpin
    b.append(f'<path d="M110 84 L300 84 L300 104 L120 104 Z" fill="{STEEL2}" stroke="{INK}"/>')
    b.append(f'<path d="M360 84 L550 84 L540 104 L360 104 Z" fill="{STEEL2}" stroke="{INK}"/>')
    # pedestal / bracket and rails
    b.append(f'<path d="M230 104 L430 104 L410 170 L250 170 Z" fill="#dfe5e8" stroke="{INK}"/>')
    b.append(f'<rect x="140" y="170" width="380" height="12" fill="{STEEL}" stroke="{INK}"/>')
    for x in (170, 490):
        b.append(f'<circle cx="{x}" cy="176" r="5" fill="{AMB}" stroke="{INK}"/>')
    b.append(f'<rect x="60" y="182" width="540" height="16" fill="{STEEL}"/>')
    b.append(t(600, 214, 'tractor frame', 'end', fill=INK2, size=11))
    # kingpin: base, shank, head (large)
    b.append(f'<rect x="300" y="84" width="60" height="10" fill="{INK}"/>')
    b.append(f'<rect x="312" y="94" width="36" height="46" fill="{INK2}"/>')
    b.append(f'<rect x="302" y="140" width="56" height="16" rx="3" fill="{INK}"/>')
    # jaws closed around shank
    b.append(f'<rect x="286" y="104" width="26" height="30" fill="{ACC}"/><rect x="348" y="104" width="26" height="30" fill="{ACC}"/>')
    # release arm + safety latch
    b.append(f'<line x1="420" y1="140" x2="600" y2="150" stroke="{INK}" stroke-width="6" stroke-linecap="round"/>')
    b.append(f'<rect x="586" y="134" width="18" height="12" fill="{RED}"/>')
    b.append(call(470, 77, s='Apron (trailer underside): not bent, cracked or broken; lying flat — no gap'))
    b.append(call(345, 89, 420, 44, s='Kingpin base'))
    b.append(call(335, 117, 470, 124, s='Kingpin SHANK: the jaws close here; kingpin not bent or damaged'))
    b.append(call(340, 148, 470, 152, s='Kingpin head: the jaws must NOT be on the head'))
    b.append(call(296, 119, 190, 130, s='Locking jaws: fully closed around the shank'))
    b.append(call(180, 94, s='Fifth-wheel skid plate: greased, securely mounted, bolts and pins in place'))
    b.append(call(595, 140, s='Release arm engaged, safety latch in place'))
    b.append(call(490, 176, s='Sliding fifth wheel: locking pins present and fully engaged'))
    b.append(call(250, 176, s='Mounting brackets, clamps, bolts, nuts: none loose or missing; platform not cracked'))
    return svg(660, 222, 'Coupling close-up in section: apron, kingpin base, shank and head, jaws, skid plate, release arm, slider pins', ''.join(b))


def gauge(cx, cy, name, ang, sub=''):
    import math
    a = math.radians(ang)
    x, y = cx + 30 * math.cos(a), cy - 30 * math.sin(a)
    s = f'<circle cx="{cx}" cy="{cy}" r="40" fill="#fff" stroke="{INK}" stroke-width="3"/>'
    s += f'<line x1="{cx}" y1="{cy}" x2="{x:.1f}" y2="{y:.1f}" stroke="{RED}" stroke-width="3"/><circle cx="{cx}" cy="{cy}" r="4" fill="{INK}"/>'
    s += t(cx, cy + 58, name, 'middle', weight=700) + (t(cx, cy + 74, sub, 'middle', size=11, fill=INK2) if sub else '')
    return s


def dash():
    b = [f'<rect x="10" y="10" width="640" height="250" rx="16" fill="#e3e7e4"/>']
    b.append(gauge(75, 80, 'Oil pressure', 120, 'normal in seconds'))
    b.append(gauge(185, 80, 'Coolant temp', 160, 'climbs to normal'))
    b.append(gauge(295, 80, 'Air pressure', 20, 'build to cut-out'))
    b.append(gauge(405, 80, 'Voltmeter', 90, 'charging'))
    b.append(f'<path d="M503 58 l14 0 l10 10 v14 l-10 10 h-14 l-10 -10 v-14 z" fill="{RED}"/>')
    b.append(t(510, 128, 'Trailer air', 'middle', weight=700, size=11) + t(510, 142, 'supply (red)', 'middle', size=11, fill=INK2))
    b.append(f'<path d="M596 56 l18 18 l-18 18 l-18 -18 z" fill="{AMB}"/>')
    b.append(t(596, 128, 'Parking', 'middle', weight=700, size=11) + t(596, 142, 'brakes (yellow)', 'middle', size=11, fill=INK2))
    lamps = [('LOW AIR', RED), ('ABS', AMB), ('OIL', RED), ('CHARGE', RED), ('4-WAY', '#1d7a45'), ('HIGH BEAM', BLUE)]
    for i, (n, c) in enumerate(lamps):
        x = 30 + i * 102
        b.append(f'<rect x="{x}" y="186" width="90" height="30" rx="6" fill="{c}"/>' + t(x + 45, 206, n, 'middle', weight=700, fill='#fff', size=11))
    b.append(t(330, 244, 'Warning lights go out after start-up; the turn-signal, 4-way, high-beam and ABS indicators work.', 'middle', size=11))
    return svg(660, 266, 'Dashboard: oil, coolant, air, voltmeter gauges; red trailer air knob; yellow parking brake knob; warning lights', ''.join(b))


def airchart():
    b = []
    X0, Y0, H = 70, 24, 250
    y = lambda p: Y0 + H * (1 - p / 140)
    for p in (0, 20, 45, 55, 100, 120, 140):
        b.append(f'<line x1="{X0}" y1="{y(p):.1f}" x2="640" y2="{y(p):.1f}" stroke="{LINE}"/>' + t(X0 - 8, y(p) + 4, str(p), 'end', size=11))
    b.append(t(24, 14, 'psi', size=11, fill=INK2))
    b.append(f'<rect x="{X0}" y="{y(140):.1f}" width="570" height="{y(120) - y(140):.1f}" fill="{ACCS}" opacity=".7"/>')
    b.append(f'<rect x="{X0}" y="{y(45):.1f}" width="570" height="{y(20) - y(45):.1f}" fill="{REDS}" opacity=".8"/>')
    b.append(f'<line x1="{X0}" y1="{y(55):.1f}" x2="640" y2="{y(55):.1f}" stroke="{RED}" stroke-dasharray="6 4"/>')
    pts = [(X0, y(60)), (150, y(130)), (210, y(130)), (230, y(127)), (330, y(125)), (360, y(110)), (430, y(70)), (460, y(57)), (520, y(32)), (620, y(32))]
    b.append('<polyline points="' + ' '.join(f'{a:.0f},{c:.0f}' for a, c in pts) + f'" fill="none" stroke="{INK}" stroke-width="3"/>')
    b.append(t(630, y(128) + 4, 'governor cut-out 120–140', 'end', size=11, fill=ACC, weight=700))
    b.append(t(96, y(55) - 5, 'warning must come on above 55', 'start', size=11, fill=RED, weight=700))
    b.append(t(630, y(22), 'knobs pop out 20–45', 'end', size=11, fill=RED, weight=700))
    b.append(call(110, y(95), s='Build air to governor cut-out (120–140 psi, or maker\'s number). Say when it cut out.'))
    b.append(call(280, y(126), s='Applied leakage: engine off, parking brake(s) released, hold the foot brake 1 minute. Say the loss and your limit.'))
    b.append(call(395, y(90), s='Key on, engine off: fan the air down by pumping the foot brake.'))
    b.append(call(460, y(57), s='Low-air warning (buzzer/light) must come on BEFORE 55 psi. Say where it came on.'))
    b.append(call(520, y(32), s='Keep fanning: parking brake knob (and tractor protection valve) pop out, normally 20–45 psi. Say where.'))
    return svg(660, 290, 'Air brake check: pressure over time from cut-out, leak test, fanning down, low air warning, knobs popping', ''.join(b))


def rig(x, y, ang=0, ghost=False, scale=1.0, flip=False):
    """Tractor-semitrailer seen from above, cab pointing along +x rotated by ang degrees about (x,y) = rear of trailer."""
    op = ' opacity=".35"' if ghost else ''
    L = 150 * scale
    s = f'<g transform="translate({x} {y}) rotate({ang})"{op}>'
    s += f'<rect x="0" y="-11" width="{L:.0f}" height="22" rx="2" fill="#eef1ec" stroke="{INK2}" stroke-width="1.5"/>'
    s += f'<rect x="{L - 2:.0f}" y="-9" width="{36 * scale:.0f}" height="18" rx="4" fill="{ACC}"/>'
    s += f'<rect x="{12 * scale:.0f}" y="-13" width="{22 * scale:.0f}" height="4" fill="{TIRE}"/><rect x="{12 * scale:.0f}" y="9" width="{22 * scale:.0f}" height="4" fill="{TIRE}"/>'
    s += '</g>'
    return s


def cones_line(x1, y1, x2, y2, n):
    out = f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{INK2}" stroke-width="1.5"/>'
    for i in range(n):
        f = i / (n - 1)
        out += cone(x1 + (x2 - x1) * f, y1 + (y2 - y1) * f)
    return out


def arrow(d, color=BLUE, dash=True):
    da = ' stroke-dasharray="7 5"' if dash else ''
    return (f'<path d="{d}" fill="none" stroke="{color}" stroke-width="2.5"{da} marker-end="url(#ah)"/>')


DEFS = f'<defs><marker id="ah" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0 0 L10 5 L0 10 z" fill="{BLUE}"/></marker></defs>'


def m_straight():
    b = [DEFS, cones_line(40, 60, 620, 60, 12), cones_line(40, 130, 620, 130, 12)]
    b.append(rig(430, 95))
    b.append(rig(60, 95, ghost=True))
    b.append(arrow('M420 95 L240 95'))
    b.append(t(330, 30, 'Back straight down the lane — only 1 look allowed', 'middle', weight=700))
    b.append(t(330, 160, 'Start centered · watch both mirrors · small corrections early ("steer toward the drift")', 'middle', size=11, fill=INK2))
    return svg(660, 170, 'Straight line backing between two rows of cones', ''.join(b))


def m_offset(side):
    """side='right': start upper lane, back into lower (right-rear) lane. 'left': opposite."""
    up, lo = 70, 140
    start, end = (up, lo) if side == 'right' else (lo, up)
    b = [DEFS]
    for yy in (35, 105, 175):
        b.append(cones_line(40, yy, 260, yy, 4))
    b.append(f'<line x1="620" y1="20" x2="620" y2="190" stroke="{INK2}" stroke-width="2" stroke-dasharray="8 6"/>')
    b.append(t(612, 206, 'outer boundary', 'end', size=11, fill=INK2))
    b.append(rig(45, start, ghost=True))
    b.append(rig(420, start))
    b.append(arrow(f'M250 {start} L410 {start}', color=INK2))
    b.append(arrow(f'M420 {start} C 330 {start}, 330 {end}, 250 {end}'))
    b.append(rig(45, end))
    b.append(t(330, 14, f'Offset back / {side}: pull ahead to the outer line, then back into the lane to your {side} rear', 'middle', weight=700))
    b.append(t(150, 206, 'finish: front of rig past the first cones', 'middle', size=11, fill=ACC, weight=700))
    return svg(660, 214, f'Offset back {side}', ''.join(b))


def m_parallel(side):
    """Rig drives rightward (+x). Driver (left) side is up in the picture."""
    b = [DEFS]
    box_y = 20 if side == 'driver' else 70
    lane_y = 100 if side == 'driver' else 40
    b.append(f'<rect x="120" y="{box_y}" width="330" height="50" fill="none" stroke="{INK2}" stroke-width="1.5"/>')
    for x in range(120, 451, 55):
        b.append(cone(x, box_y + (50 if side == 'driver' else 0)))
    for yy in (box_y, box_y + 50):
        b.append(cone(120, yy) + cone(450, yy))
    b.append(rig(400, lane_y, ghost=True))
    b.append(rig(130, box_y + 25))
    b.append(arrow(f'M395 {lane_y} C 330 {lane_y}, 330 {box_y + 25}, 250 {box_y + 25}'))
    where = 'left' if side == 'driver' else 'right'
    b.append(t(330, 150, f'Space on your {where}: drive past it, then back in. The WHOLE rig must end inside the cones.', 'middle', weight=700, size=11.5))
    return svg(660, 160, f'Parallel park {side} side', ''.join(b))


def m_alley():
    b = [DEFS]
    b.append(cones_line(40, 30, 620, 30, 9))
    b.append(t(620, 22, 'outer boundary', 'end', size=11, fill=INK2))
    # alley below boundary between x=440..500, from y=110 to 300 (back at 300)
    b.append(cones_line(440, 150, 440, 300, 4) + cones_line(500, 150, 500, 300, 4) + cones_line(440, 300, 500, 300, 3))
    b.append(f'<line x1="440" y1="60" x2="440" y2="110" stroke="{INK2}" stroke-dasharray="6 5"/><line x1="500" y1="60" x2="500" y2="110" stroke="{INK2}" stroke-dasharray="6 5"/>')
    # rig heading left (cab at left), parallel to boundary, past the alley
    b.append(rig(260, 62, ang=180, ghost=True))  # rear at 260 -> body to x=110.. cab left
    b.append(arrow('M268 62 C 400 62, 470 90, 470 150'))
    b.append(rig(470, 290, ang=-90))
    b.append(t(40, 250, 'Finish: rear within 3 ft of the back line,', size=11.5, weight=700, fill=RED) + t(40, 266, 'and straight in the alley.', size=11.5, weight=700, fill=RED))
    b.append(f'<path d="M300 258 L425 296" stroke="{RED}" stroke-width="1.2"/>')
    b.append(t(40, 120, 'Sight-side (driver-side) back into the alley.', size=11.5, weight=700))
    b.append(t(40, 138, 'Set up parallel to the outer boundary, past the alley.', size=11.5))
    b.append(t(40, 156, 'Cab turned toward your driver side, so you watch the', size=11.5))
    b.append(t(40, 172, 'trailer out of your own window.', size=11.5))
    return svg(660, 320, 'Alley dock: sight-side backing into an alley at a right angle', ''.join(b))


def backing_wheel():
    b = [DEFS]
    b.append(f'<circle cx="130" cy="120" r="70" fill="none" stroke="{INK}" stroke-width="10"/><circle cx="130" cy="120" r="10" fill="{INK}"/>')
    b.append(f'<circle cx="130" cy="190" r="11" fill="{AMB}"/>')
    b.append(arrow('M130 214 Q 90 218 76 198', color=BLUE, dash=False))
    b.append(t(130, 24, 'Your hand at the BOTTOM of the wheel', 'middle', weight=700))
    b.append(t(130, 40, "(driver's view)", 'middle', size=11, fill=INK2))
    b.append(t(130, 244, 'Hand moves to your left', 'middle', weight=700))
    b.append(t(470, 40, 'Same moment, rig seen from above', 'middle', size=11, fill=INK2))
    b.append(rig(360, 150, ang=0, scale=.9))
    b.append(arrow('M358 150 Q 320 150 296 112', color=BLUE, dash=False))
    b.append(t(300, 96, "Trailer swings to your left", 'middle', weight=700, fill=BLUE))
    b.append(t(470, 200, "(driver's left = up in this picture)", 'middle', size=11, fill=INK2))
    return svg(660, 256, 'Backing steering: hand at the bottom of the wheel moves the trailer the same way', ''.join(b))


def three_points():
    b = []
    b.append(f'<rect x="200" y="20" width="200" height="220" rx="10" fill="{ACC}" opacity=".85"/>')
    b.append(f'<rect x="220" y="40" width="120" height="80" rx="4" fill="#cfe3ec"/>')
    b.append(f'<rect x="408" y="40" width="8" height="150" rx="4" fill="{STEEL}"/><rect x="186" y="60" width="8" height="120" rx="4" fill="{STEEL}"/>')
    for yy in (200, 250):
        b.append(f'<rect x="230" y="{yy}" width="90" height="10" fill="{STEEL}"/>')
    # person facing truck: head, body, arms to two handles, one foot on step
    b.append(f'<circle cx="300" cy="120" r="16" fill="{AMBS}" stroke="{INK}" stroke-width="2"/>')
    b.append(f'<line x1="300" y1="136" x2="300" y2="200" stroke="{INK}" stroke-width="5"/>')
    b.append(f'<line x1="300" y1="150" x2="190" y2="110" stroke="{INK}" stroke-width="4"/><line x1="300" y1="150" x2="412" y2="110" stroke="{INK}" stroke-width="4"/>')
    b.append(f'<line x1="300" y1="200" x2="275" y2="248" stroke="{INK}" stroke-width="4"/><line x1="300" y1="200" x2="330" y2="262" stroke="{INK}" stroke-width="4"/>')
    for x, y, n in [(190, 110, 1), (412, 110, 2), (275, 248, 3)]:
        b.append(f'<circle cx="{x}" cy="{y}" r="11" fill="{AMB}"/>' + t(x, y + 4, str(n), 'middle', weight=700))
    b.append(t(460, 90, 'Face the truck', weight=700))
    b.append(t(460, 110, '2 hands + 1 foot', weight=700))
    b.append(t(460, 130, '(or 2 feet + 1 hand)', size=11.5, fill=INK2))
    b.append(t(460, 150, 'touching at all times', weight=700))
    b.append(t(20, 90, 'Before you get out:', weight=700))
    b.append(t(20, 110, '1. Neutral'))
    b.append(t(20, 130, '2. Parking brake(s) set'))
    b.append(t(20, 150, '3. Climb down facing in'))
    return svg(660, 290, 'Getting out for a look: neutral, parking brake, face the truck, 3 points of contact', ''.join(b))


# ------------------------------------------------------------------ 12. road test scenes
def turns():
    b = [DEFS]
    # 4-lane road horizontal (2 each way) crossing vertical 4-lane road; view from above
    b.append(f'<rect x="0" y="120" width="660" height="140" fill="#e6eaec"/><rect x="260" y="0" width="140" height="380" fill="#e6eaec"/>')
    b.append(f'<line x1="0" y1="190" x2="260" y2="190" stroke="{AMB}" stroke-width="3"/><line x1="400" y1="190" x2="660" y2="190" stroke="{AMB}" stroke-width="3"/>')
    b.append(f'<line x1="330" y1="0" x2="330" y2="120" stroke="{AMB}" stroke-width="3"/><line x1="330" y1="260" x2="330" y2="380" stroke="{AMB}" stroke-width="3"/>')
    for yy in (155, 225):
        b.append(f'<line x1="0" y1="{yy}" x2="260" y2="{yy}" stroke="#fff" stroke-width="2" stroke-dasharray="12 10"/><line x1="400" y1="{yy}" x2="660" y2="{yy}" stroke="#fff" stroke-width="2" stroke-dasharray="12 10"/>')
    for xx in (295, 365):
        b.append(f'<line x1="{xx}" y1="0" x2="{xx}" y2="120" stroke="#fff" stroke-width="2" stroke-dasharray="12 10"/><line x1="{xx}" y1="260" x2="{xx}" y2="380" stroke="#fff" stroke-width="2" stroke-dasharray="12 10"/>')
    # truck coming north in right lane (x≈382) from bottom
    b.append(f'<rect x="372" y="300" width="20" height="60" fill="{ACC}"/>')
    b.append(arrow('M382 296 L382 226 Q 386 242 432 242 L 640 242', color=ACC, dash=False))  # right turn: finish in curb lane (y≈242)
    b.append(t(440, 290, 'Right turn → finish in the', size=11.5, weight=700, fill=ACC) + t(440, 306, 'right-most (curb) lane', size=11.5, weight=700, fill=ACC))
    b.append(arrow('M348 296 L348 230 Q 346 176 280 172 L 20 172', color=BLUE, dash=False))  # left turn: finish just right of center (y≈172)
    b.append(t(20, 94, 'Left turn → finish in the lane directly', size=11.5, weight=700, fill=BLUE))
    b.append(t(20, 110, 'right of the center line', size=11.5, weight=700, fill=BLUE))
    return svg(660, 380, 'Turns on a multi-lane road: right turn ends in curb lane, left turn ends right of the center line', ''.join(b))


def safegap():
    b = []
    b.append(f'<rect x="0" y="60" width="660" height="70" fill="#e6eaec"/>')
    b.append(f'<rect x="60" y="72" width="200" height="46" fill="#eef1ec" stroke="{INK2}"/><circle cx="205" cy="120" r="10" fill="{TIRE}"/><circle cx="235" cy="120" r="10" fill="{TIRE}"/><circle cx="85" cy="120" r="10" fill="{TIRE}"/>')
    b.append(f'<rect x="330" y="68" width="160" height="54" fill="{ACC}"/><rect x="330" y="68" width="40" height="54" fill="#0e7549"/>')
    b.append(t(330, 28, "Stopped behind another vehicle: stop where you can still SEE ITS REAR TIRES touching the road.", 'middle', weight=700, size=11.5))
    b.append(t(300, 150, 'safe gap', 'middle', weight=700, fill=RED))
    b.append(f'<line x1="262" y1="138" x2="328" y2="138" stroke="{RED}" stroke-width="2"/>')
    b.append(t(330, 176, 'Behind the stop line, no rolling, front wheels straight.', 'middle', size=11.5, fill=INK2))
    return svg(660, 186, 'Safe gap: stop where you can see the rear tires of the vehicle ahead', ''.join(b))


def rrcross():
    b = []
    b.append(f'<rect x="0" y="40" width="660" height="80" fill="#e6eaec"/>')
    for x in (560, 590):
        b.append(f'<line x1="{x}" y1="30" x2="{x}" y2="130" stroke="{INK2}" stroke-width="4"/>')
    for yy in range(34, 130, 12):
        b.append(f'<line x1="552" y1="{yy}" x2="598" y2="{yy}" stroke="{INK2}" stroke-width="2"/>')
    b.append(f'<rect x="260" y="48" width="290" height="64" fill="{ACCS}" stroke="{ACC}" stroke-dasharray="6 4"/>')
    b.append(t(405, 86, 'stop here: 15–50 ft from nearest rail', 'middle', weight=700, fill=ACC))
    b.append(f'<rect x="80" y="62" width="170" height="36" fill="{ACC}"/>' + t(165, 85, 'bus / placarded →', 'middle', fill='#fff', weight=700, size=11))
    b.append(t(10, 22, '4-way flashers ON approaching · stop · look and listen both ways · (bus: open window and door)', size=11.5, weight=700))
    b.append(t(10, 146, 'While ANY part is on the crossing: no stopping, no gear change, no passing, no lane change. Flashers OFF after.', size=11.5, weight=700, fill=RED))
    return svg(660, 156, 'Railroad crossing stop for buses and placarded vehicles: 15 to 50 feet from the nearest rail', ''.join(b))


def lanechange():
    b = [DEFS]
    b.append(f'<rect x="0" y="30" width="660" height="120" fill="#e6eaec"/>')
    b.append(f'<line x1="0" y1="90" x2="660" y2="90" stroke="#fff" stroke-width="2" stroke-dasharray="14 10"/>')
    b.append(f'<rect x="30" y="104" width="90" height="30" fill="{ACC}"/>')
    b.append(arrow('M124 119 C 200 119, 220 60, 300 60', color=BLUE, dash=False))
    b.append(arrow('M380 60 C 460 60, 480 119, 560 119', color=BLUE, dash=False))
    b.append(t(210, 22, '1. Change LEFT', 'middle', weight=700) + t(470, 22, '2. Then back RIGHT', 'middle', weight=700))
    b.append(t(330, 176, 'Each time: mirrors + traffic check → signal → wait for a safe gap → move → cancel signal', 'middle', weight=700, size=11.5))
    return svg(660, 186, 'Lane change test: left, then back right', ''.join(b))


def flow():
    b = []
    boxes = [('1', 'Vehicle inspection', '~40 min', 'Must pass first'), ('2', 'Basic control skills', '~30 min', 'Backing exercises'), ('3', 'Road test', '~45–60 min', '≤30 errors, 0 critical')]
    for i, (n, name, tm, note) in enumerate(boxes):
        x = 20 + i * 215
        b.append(f'<rect x="{x}" y="20" width="190" height="96" rx="10" fill="{ACCS}" stroke="{ACC}" stroke-width="2"/>')
        b.append(f'<circle cx="{x + 24}" cy="46" r="14" fill="{ACC}"/>' + t(x + 24, 51, n, 'middle', weight=700, fill='#fff'))
        b.append(t(x + 44, 51, name, weight=700))
        b.append(t(x + 16, 80, tm, fill=INK2) + t(x + 16, 102, note, weight=700, fill=ACC))
        if i < 2:
            b.append(f'<path d="M{x + 192} 68 l18 0 m-7 -7 l7 7 l-7 7" stroke="{INK}" stroke-width="2.5" fill="none"/>')
    b.append(t(330, 140, 'Fail one part → the rest is put off to another day. 3 tries in all for the three tests.', 'middle', weight=700, fill=RED, size=11.5))
    return svg(660, 150, 'The three skills tests in order', ''.join(b))


ALL = dict(anatomy=anatomy, route=route, engine=engine, steering=steering, suspension=suspension, brakes=brakes,
           wheels=wheels, coupling=coupling, dash=dash, airchart=airchart, straight=m_straight,
           offset_right=lambda: m_offset('right'), offset_left=lambda: m_offset('left'),
           pp_driver=lambda: m_parallel('driver'), pp_conv=lambda: m_parallel('conventional'), alley=m_alley,
           backing_wheel=backing_wheel, three_points=three_points, turns=turns, safegap=safegap, rr=rrcross,
           lanechange=lanechange, flow=flow)


def draw(name):
    """Return (svg markup, numbered legend list) for one diagram."""
    LEG.clear()
    out = ALL[name]()
    return out, list(LEG)
