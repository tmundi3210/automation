# Turlock Independent Food & Dining — Supply Chain & Operations Analysis

**Scope:** 40 independent Food & Dining businesses in Turlock, CA (Stanislaus County), drawn from
`analysis/food_dining/businesses.json`. Analysis is organized by **food format/category**, covering what
each produces, kitchen and equipment, staffing model, key inputs and likely suppliers, a labeled
cost/margin benchmark, seasonality, and the health-permit / ABC-license / commissary footprint.

**Compiled:** 2026-06-30 · Analyst role: Supply & Operations.

---

## Honesty & Methodology Guardrails

- **Per-business revenue is NOT public.** No dollar revenue is stated as fact for any named business.
  Where a figure appears, it is an **industry benchmark scaled by an observable size signal** (seats,
  staff, sqft, format) and is explicitly tagged **"estimate, not actual"** with a confidence level and
  what would confirm it.
- **Supplier attributions are likely/representative**, not confirmed contracts. Turlock independents do
  not publish their vendor lists. Named distributors below are the realistic supply universe (verified to
  exist and serve this geography); the link from a *specific named restaurant* to a *specific supplier* is
  inference unless a source says otherwise.
- **City/county/state figures** (permit fees, license costs) are cited public facts. Cost-structure
  percentages are cited published industry benchmarks.

### The regional supply universe (applies to most formats below)

| Node | Representative providers serving the Turlock/Modesto/Central Valley corridor | Source |
|---|---|---|
| Broadline distributor | **Sysco Central California**, 136 S. Mariposa Rd, **Modesto** (≈15 mi from Turlock) — meat, produce, frozen, dry, paper, chem, smallwares | [Sysco](https://www.sysco.com/contact/our-locations/central-california); [Modesto Chamber](https://business.modchamber.org/list/member/sysco-food-services-of-central-california-1770) |
| Broadline / cash-and-carry | **US Foods**, **Restaurant Depot** (membership cash-and-carry) — note Sysco is acquiring Jetro/Restaurant Depot (announced 2026), which may consolidate Valley pricing | [Forbes](https://www.forbes.com/sites/phillempert/2026/03/31/syscos-29-billion-power-grab-what-the-jetro-restaurant-depot-deal-means-for-main-street-menus/) |
| Tortilla/masa | **The Tortilla Plug** (Central Valley/Sacramento), **Romero's Food Products** (Fresno→SD), **Mi Rancho**, **Masienda** (heirloom masa) | [Tortilla Plug](https://www.thetortillaplug.com/); [Romero's](https://romerosfood.com/distribution/); [Masienda](https://masienda.com/collections/wholesale) |
| Mediterranean/Assyrian import | **Sadaf/Soofer**, **Zarrin/Blansh Intl** (San Jose), **Karabetian** (since 1987), **Macar Foods** | [Karabetian](https://karabetian.com/); [Zarrin](https://www.zarrinproducts.com/); [Macar](https://macarfoods.com/) |
| Seafood (mariscos) | **Pucci Foods** (SF, since 1918), **Santa Monica Seafood**, **Sierra Gold Seafood**, **Catalina Offshore** (Baja species) | [Pucci](https://puccifoods.com/); [Sierra Gold](https://sierragoldseafood.com/wholesale/); [Catalina](https://catalinaop.com/wholesale/) |
| Produce/dairy/meat (local) | Central Valley is a top US ag region; local carnicerias, dairies and produce houses supplement broadline (see catalog entries: La Rancherita, La Morenita, Tortilleria Tres Hermanos) | catalog `turlock_independent_business_catalog.json` |

Turlock sits in the heart of California's #1 dairy county group and a major produce/almond/poultry belt,
so **local produce, dairy, and meat are a genuine cost advantage** versus coastal metros — short freight,
direct-from-grower/carniceria options, and a dense Mexican/Assyrian/Portuguese grocery ecosystem
(documented in the catalog's `ethnic_community` lens) that doubles as specialty-input supply.

### Permit / license / commissary footprint (Stanislaus County + City of Turlock)

| Item | Public fact | Source |
|---|---|---|
| County food-facility health permit | Required for every fixed food facility; plan check + annual permit via Stanislaus County Environmental Resources Food Program | [Stan. County Food Program](https://www.stancounty.com/er/environmentalhealth/food-program.shtm) |
| Mobile food facility (truck/cart) county permit | 4 categories, **$69–$548/yr** depending on risk/type | [Modesto Bee via NewsBreak](https://www.newsbreak.com/the-modesto-bee-1592579/4171481250641-how-many-food-trucks-operate-in-stanislaus-county-what-it-takes-to-get-a-permit) |
| **Commissary requirement** | A mobile food vendor must operate "in conjunction with a commissary or other permanent food facility"; commissary verification form required | [Stan. County commissary form](https://www.stancounty.com/er/pdf/mff-commissary-application.pdf); [How to apply MFF](https://www.stancounty.com/er/pdf/how-to-apply-mff-permit.pdf) |
| City of Turlock mobile facility permit | **$900**; requires site plan, county health permit, commissary form, restroom form | [City of Turlock Mobile Food Vendor](https://www.cityofturlock.org/doingbusinessinturlock/permits/mobilefoodvendor.asp) |
| City of Turlock business license | **$114** | same |
| ABC Type 41 (beer & wine, bona fide eating place) | App fee **$905**; annual **$400–$900** (population-based) | [CA ABC fees](https://www.abc.ca.gov/licensing/license-fees/); [PermitPlace 2026](https://permitplace.com/which-california-restaurant-liquor-license-is-right-for-you/) |
| ABC Type 47 (full liquor + food) | App fee **$6,275–$16,560**; annual **$925–$1,450**; **secondary-market license can exceed $100k–$200k** in tight counties | same |

### Cost-structure benchmark backbone (used in every section below)

Published 2025–2026 foodservice benchmarks ([VantaInsights](https://vantainsights.com/insights/restaurant-food-cost-percentage),
[Toast](https://pos.toasttab.com/blog/on-the-line/restaurant-payroll-percentage), [Leverage Buying Group 2025](https://leveragebuyinggroup.com/wp-content/uploads/2025/05/2025-Restaurateur-Benchmark-Guide-Final-5-24-2025.pdf)):

- **Food cost:** QSR 28–32% · casual 30–34% · fine dining 32–35%.
- **Labor:** QSR 30–32% · full-service 36–40% (median FSR ~36.5%; profitable operators ~34%).
- **Prime cost (food+labor):** target **55–65%** of revenue.
- **Net margin:** full-service 3–8% · fast-casual 4–10% · QSR 5–12%. Only ~42% of US restaurants were profitable in 2024.
- **Coffee:** COGS 25–35%, labor 30–40%, rent ideally <12%; net 2.5–7% typical, 12–15% for tightly-run shops ([VantaInsights coffee](https://vantainsights.com/insights/coffee-shop-profit-margins), [Pool Six](https://blog.poolsixcoffeeroasters.com/setting-the-margins-on-your-coffee-menu/)).

---

## 1. Taqueria (sit-down/counter) — *La Taqueria, Tacos California, Taqueria La Primera*

**Produces/sells:** Tacos, burritos, quesadillas, loaded chips/fries, shrimp cocktail, aguas frescas
(La Taqueria, est. 2016). Counter-service, high-volume, value-priced.

**Kitchen & equipment:** Flat-top griddle/plancha, charbroiler, vertical al-pastor trompo (some), fryer,
steam table, walk-in, blender bank for aguas frescas, tortilla warmer. Modest footprint; many are
counter-order with a small dining room.

**Staffing:** 6–15 across shifts — owner-operator + cooks on the line, prep cook, cashier/front. Heavy
family-labor component is typical and structurally lowers reported labor %.

**Key inputs & likely suppliers:** Corn/flour tortillas and chips from a Valley tortilla supplier
(**The Tortilla Plug / Romero's / Mi Rancho**) or a local tortilleria (catalog: **Tortilleria Tres
Hermanos**); marinated meats (carne asada, al pastor, carnitas, lengua, tripa) from local **carnicerias**
(catalog: La Rancherita, La Morenita) and broadline (**Sysco Modesto / Restaurant Depot**); produce
(cilantro, onion, tomato, limes, avocado) from Central Valley produce — a regional cost advantage; dried
chiles/spices from Mexican grocers.

**Cost/margin pattern:** Fast-casual/QSR profile — **food 28–32%, labor 30–32%**; family labor often
pushes effective labor lower. *Benchmark, not actual.*

**Seasonality:** Summer lift (aguas frescas, patio); steady year-round demand.

**Permit footprint:** County food-facility permit; ABC Type 41 only if serving beer.

---

## 2. Taco Truck / Food Truck — *Taqueria Hidalgo (15 yrs, cash-only), Conchitas, Mel's*

**Produces/sells:** Street tacos (carne asada, al pastor, tripa, lengua), burritos, quesadillas. Hidalgo
is a 15-year fixture at one location.

**Kitchen & equipment:** On-board plancha, trompo, fryer, cold-hold, propane, generator, water/wastewater
tanks. **All trucks require a commissary** for overnight parking, water/waste servicing, and prep/storage.

**Staffing:** 2–5; owner-operator core, family labor.

**Key inputs & likely suppliers:** Same Mexican supply chain as taquerias — carniceria meats, Valley
tortilla supplier, local produce — but purchased in smaller lots, often via **Restaurant Depot** cash-and-carry.

**Cost/margin pattern:** Low fixed overhead (no dining-room rent) is the structural edge; **food ~30%**,
labor low (family). Net margin can exceed brick-and-mortar QSR but volume is capped by truck capacity. *Benchmark, not actual.*

**Seasonality:** Strong warm-season/evening and event-driven (fairs, soccer, downtown events); weather-exposed.

**Permit footprint (the binding constraint for this format):** Stanislaus County MFF permit
**$69–$548/yr**; **City of Turlock mobile facility permit $900** + **$114 business license**; **mandatory
commissary** verification; private-property/paved-surface operating rules. Sources: [City of Turlock](https://www.cityofturlock.org/doingbusinessinturlock/permits/mobilefoodvendor.asp);
[Stan. County commissary](https://www.stancounty.com/er/pdf/mff-commissary-application.pdf).

> **Size-scaled benchmark (illustrative):** A single taco truck running ~$30–$45 avg-ticket-equivalent
> volume of, say, 80–150 covers/day → published mobile-vendor benchmarks imply low-six-figure annual sales.
> **Estimate, not actual — low confidence.** Confirmed only by POS/sales records, which are private.

---

## 3. Panaderia (Mexican bakery) — *Panaderia Los Compadres, Panaderia San Miguel*

**Produces/sells:** Conchas, pan dulce, bolillos, tres leches, specialty breads (San Miguel:
jalapeño–cream-cheese bread). Open 7 days (Los Compadres). Retail-counter + wholesale to local taquerias/markets is common.

**Kitchen & equipment:** Revolving/rack ovens, dough mixers/sheeters, proofing cabinets, work tables,
display cases. Capital-intensive baking line; long pre-dawn production day.

**Staffing:** Bakers (panaderos) working overnight/early shift + counter staff; family-operated.

**Key inputs & likely suppliers:** Flour (bulk all-purpose/bread flour), sugar, lard/shortening, eggs,
yeast, piloncillo, food coloring — from broadline (**Sysco/Restaurant Depot**) and bakery-ingredient
distributors; eggs/dairy local. Pan dulce is **labor-intensive**, with each piece hand-shaped and
hand-decorated, which is the dominant cost driver and a known margin squeeze for community panaderias
([King Arthur](https://www.kingarthurbaking.com/blog/2021/05/18/conchas-pan-dulce); [pricing pressure](https://www.lemon8-app.com/@janvillarre/7552231105049002551?region=us)).

**Cost/margin pattern:** Bakery COGS relatively low (flour/sugar cheap) but **labor-heavy**; thin margins,
priced for community affordability. *Benchmark, not actual.*

**Seasonality:** Strong peaks — **Día de los Muertos (pan de muerto)** and **Día de Reyes (rosca de reyes, Jan 6)**;
Mother's Day/quinceañera/birthday cake orders year-round.

**Permit footprint:** County food-facility permit; retail bakery (no ABC).

---

## 4. Mariscos (Mexican seafood, sit-down) — *Alegre Marisquero, Mariscos Culiacan, Mariscos Guayabitos*

**Produces/sells:** Aguachile, ceviche, oysters, tostadas (tostada culichi), shrimp cocktails, whole
fish, caldos. Guayabitos draws on Nayarit/Michoacán/Sinaloa coastal styles.

**Kitchen & equipment:** Heavy **cold line** (raw-bar prep, oyster shucking, ceviche cure), ample
refrigeration/ice, blast capability, fryers, cooktop for caldos. Strong cold-chain integrity is critical
(raw seafood = highest food-safety risk class).

**Staffing:** 8–20; skilled raw-bar/cold-prep cooks plus hot line and servers (full table service).

**Key inputs & likely suppliers:** Fresh shrimp, fish, oysters, octopus — the cost-and-risk center.
Likely **Pucci Foods (SF)**, **Sierra Gold**, **Santa Monica Seafood**, **Catalina Offshore** (Baja
species fit the Sinaloa-style menu), supplemented by broadline frozen and Mexican specialty importers for
sauces/chiles/Clamato. Limes, cucumber, onion, cilantro, avocado — local Valley produce.

**Cost/margin pattern:** **Highest food cost in the set (often 35%+)** due to seafood volatility and
spoilage; full-service **labor 36–40%**. Margin sensitive to shrimp/fish market swings. *Benchmark, not actual.*

**Seasonality:** Lent (pescatarian demand spike), summer; seafood pricing seasonal and import-driven.

**Permit footprint:** County permit at a **high-risk category** (raw seafood); ABC Type 41/47 where alcohol served.

---

## 5. Sit-down ethnic — Mexican — *Las Casuelas, Mariachis, La Mo's Cafe*

**Produces/sells:** Full Mexican menus — molcajete (Mariachis), combinations, Latin-inspired plates
(La Mo's, which advertises **fresh ingredients from local farms**). Full table service.

**Kitchen & equipment:** Full hot line (range, char-grill, fryers, comal/plancha), steam tables, walk-in,
bar in some. Larger footprint than taquerias.

**Staffing:** 12–30; line cooks, prep, dishwashers, servers, host, bartender. Higher front-of-house ratio.

**Key inputs & likely suppliers:** Broadline (**Sysco Modesto**) for center-of-plate and dry goods;
carnicerias for specialty cuts; tortilla supplier; **La Mo's explicitly sources from local farms** (per
its catalog/Tripadvisor positioning) — a documented local-produce link. Dried chiles/spices from Mexican grocers.

**Cost/margin pattern:** Casual full-service — **food 30–34%, labor 36–40%, prime ~60–65%**, net 3–8%. *Benchmark, not actual.*

**Seasonality:** Cinco de Mayo, Mother's Day, holidays; steady otherwise.

**Permit footprint:** County permit; ABC Type 41 (beer/wine) or **Type 47** if full bar.

---

## 6. Cafe / Coffeehouse — *Alison's Cafe House, Cafe Rome, Grace and Gather, Rise N Brew, TaZa*

**Produces/sells:** Espresso/specialty drinks, signature lattes (TaZa: S'mores, Carmello Marshmello),
crepes (Cafe Rome), pastries (Rise N Brew bakes in-house; per catalog Rise N Brew may use a **house roast**).

**Kitchen & equipment:** Espresso machine + grinders (the capital anchor), brewers, blenders, refrigeration,
pastry case; light kitchen/bakery for food-forward shops. Small footprint, high seat-turn at peak.

**Staffing:** 3–10 baristas/shift; owner-operator + part-time, often students (Stanislaus State/CSU
Stanislaus is in Turlock — a labor and demand factor).

**Key inputs & likely suppliers:** Roasted coffee from a regional **roaster-wholesaler** (Valley roasters
exist; catalog notes Chatz Roasting in nearby Ceres and Rise N Brew's house roast); milk/alt-milks
(local dairy is a Valley strength); syrups, cups/lids/paper from broadline or **Restaurant Depot**;
pastries baked in-house or bought from local bakeries.

**Cost/margin pattern:** Beverage COGS low but **food/milk mix lifts COGS to 25–35%**; **labor 30–40%**;
rent ideally <12%. Net **2.5–7%** typical, **12–15%** for tightly run shops compressing labor to 35–38%
and COGS to 26–29%. Roasting in-house (green at $4–7/lb vs $8–14/lb wholesale roasted) is a real margin
lever ([Pool Six](https://blog.poolsixcoffeeroasters.com/setting-the-margins-on-your-coffee-menu/); [VantaInsights coffee](https://vantainsights.com/insights/coffee-shop-profit-margins)). *Benchmark, not actual.*

**Seasonality:** Iced/cold-brew summer lift; pumpkin/holiday Q4; tracks the **university academic calendar**.

**Permit footprint:** County permit (food-handling tier varies with food menu); no ABC unless serving alcohol.

---

## 7. Bakery-Cafe & European/Specialty Bakery — *Sunrise Bakery & Cafe, Olde Tyme Pastries, Mary Ann's Bakery & Mini Market*

**Produces/sells:** Fresh bread, pastries, cakes, deli items + espresso (Olde Tyme = bakery/deli/espresso
bar; Sunrise = long-standing baking tradition with coffee via Joe.coffee listing). Mary Ann's pairs bakery
with a mini-market (retail grocery).

**Kitchen & equipment:** Deck/rack ovens, mixers, sheeters, proofers, deli case, espresso bar, display
cases. Wholesale-capable (Sunrise/Mary Ann's appear in the catalog's wholesale/commercial-bakery lens).

**Staffing:** Overnight bakers + daytime counter/deli + barista; family/owner-operated.

**Key inputs & likely suppliers:** Bulk flour, butter, sugar, eggs, dairy (local), chocolate/fillings,
deli meats/cheese (broadline), coffee (regional roaster). Specialty European ingredients via specialty distributors.

**Cost/margin pattern:** Combined bakery (low COGS, high labor) + cafe (high-margin beverages) +
deli (mid). Blended **prime ~55–65%**; wholesale accounts add volume at thinner unit margin. *Benchmark, not actual.*

**Seasonality:** Major **holiday cake/pastry peaks** (Easter, Thanksgiving, Christmas), wedding/event cake season (spring–summer).

**Permit footprint:** County food-facility permit; retail/wholesale bakery; market component (Mary Ann's) adds retail-food handling.

---

## 8. European/Specialty — Italian sit-down & Pizzeria — *Villa Napoli, Vito's Ristorante (+pizza), Dean's Pizza, Rico's Pizza*

**Produces/sells:** Pizza (Vito's, Dean's, Rico's), pasta, Italian entrées, wine (Vito's has a wine list),
Italian sandwiches (Dean's).

**Kitchen & equipment:** Pizza deck/conveyor ovens, dough mixer/proofer, pasta cooker, range/sauté line,
walk-in, prep tables, sandwich station. Pizzerias are dough-and-cheese operations; sit-down Italian adds a full line.

**Staffing:** Pizzeria 5–12 (pizzaiolo, prep, counter/drivers); sit-down Italian 12–25 with servers and bar.

**Key inputs & likely suppliers:** Flour, **mozzarella/cheese (a major and volatile cost — Valley dairy is
a local advantage)**, canned tomatoes/sauce, cured meats, pasta, olive oil, wine. Broadline (**Sysco/US
Foods**) plus Italian/specialty distributors; pizza-cheese often a dedicated dairy contract.

**Cost/margin pattern:** Pizza is a classic **high-margin format** (low food cost on dough/sauce/cheese,
~28–32%) when volume is good; sit-down Italian runs casual full-service (food 30–34%, labor 36–40%).
Cheese-price swings are the main margin risk. *Benchmark, not actual.*

**Seasonality:** Pizza steady with sports/event spikes; sit-down Italian peaks on holidays/Valentine's/date nights.

**Permit footprint:** County permit; ABC Type 41 (Vito's wine) or 47.

---

## 9. Contemporary American / Fine Dining / Gastropub / Wine Bar — *Atrium, Bistro 234, Roth Social House, 10 East Kitchen & Tap House, Gray's Tavern, Camp 4 Wine*

**Produces/sells:** Chef-driven contemporary American (Bistro 234 — voted "Most Romantic" by Turlock
Journal readers), burgers + craft beer (10 East), tavern fare (Gray's), wine + lunch with monthly specials
(Camp 4 Wine). Downtown Turlock concentration.

**Kitchen & equipment:** Full scratch line (range, grill, sauté, fryer, sous-vide for finer spots),
walk-in, bar program with **draft system** (10 East — tap house), wine storage (Camp 4, Bistro). Higher
equipment and ambiance capex.

**Staffing:** 15–35; chef/sous, line cooks, prep, dish, full FOH (servers, bartenders, host). Highest
labor ratio in the set, especially fine dining.

**Key inputs & likely suppliers:** Broadline (**Sysco/US Foods Modesto**) for proteins/produce/dry;
**Central Valley local farms/ranches and seasonal produce** for chef-forward menus; craft beer via CA
beer distributors; wine via licensed wholesalers. Premium center-of-plate (steak, seafood) drives COGS.

**Cost/margin pattern:** **Fine dining food 32–35%, labor 35–40%** (premium ingredients + service);
gastropub/tap house improves margin via **high-margin beverage/alcohol mix** offsetting food. Net 3–8%
full-service. Alcohol is the margin engine for the gastropub/wine-bar subset. *Benchmark, not actual.*

**Seasonality:** Valentine's/holidays/Mother's Day peaks (date-night positioning); patio/summer for taproom; downtown event nights.

**Permit footprint:** County permit; **ABC Type 47** (full liquor — 10 East, Gray's, likely Bistro/Atrium/Roth)
or **Type 41/02** for wine bar (Camp 4). Type 47 app **$6,275–$16,560** + annual **$925–$1,450**, and a
**secondary-market license can run into six figures** — a material entry barrier for the bar-forward
concepts ([CA ABC](https://www.abc.ca.gov/licensing/license-fees/)).

---

## 10. Asian Fusion & American Comfort — *First & Main (pan-Asian), New Town Cafe*

**Produces/sells:** First & Main — pan-Asian (Indian, Korean, Japanese, Vietnamese influences), a
multi-cuisine line. New Town Cafe — American comfort with creative dishes.

**Kitchen & equipment:** First & Main needs **wok line + tandoor/curry station + multiple prep zones**
(equipment-diverse for a multi-cuisine menu); New Town is a standard cafe griddle/range line.

**Staffing:** 10–25; First & Main's breadth raises skilled-cook needs and prep labor; cafe is leaner.

**Key inputs & likely suppliers:** Broadline (**Sysco**) for proteins/produce + **Asian specialty
importers/distributors** (Bay Area Asian wholesalers) for rice, sauces, noodles, spices, specialty
produce. Multi-cuisine = more SKUs and more vendors than a single-cuisine kitchen.

**Cost/margin pattern:** Casual full-service (food 30–34%, labor 36–40%). Multi-cuisine inventory breadth
pressures food cost and waste; New Town cafe is a tighter QSR-leaning profile. *Benchmark, not actual.*

**Seasonality:** Steady; cafe skews breakfast/lunch daypart.

**Permit footprint:** County permit; ABC Type 41 if beer/wine.

---

## 11. Assyrian / Mediterranean — *House of Kabobs, Patogh Restaurant, Ishtar Mediterranean Market & Grill*

**Produces/sells:** Traditional Assyrian cuisine (House of Kabobs), Mediterranean + Assyrian (Patogh,
6+ yrs), chicken kebabs/lamb gyros **plus an Assyrian market** (Ishtar — spices, produce, breads). Turlock
has a large Assyrian community, making this a culturally anchored, locally-demanded format.

**Kitchen & equipment:** **Charbroiler/kebab grill** (the defining equipment), vertical gyro/shawarma
spit, rice cookers, tandoor/clay oven or flatbread oven, prep for marinades and mezze. Ishtar adds retail
grocery shelving + market refrigeration.

**Staffing:** 6–18; grill cooks, prep, FOH; family/community-operated. Ishtar's market side adds retail clerks.

**Key inputs & likely suppliers:** **Halal lamb and chicken** (specialty halal meat suppliers/local halal
markets — catalog lists Turlock Halal Market and halal grocers), rice (basmati), bulgur, spices, tahini,
olives, flatbreads/lavash — via **Mediterranean/Assyrian importers (Sadaf/Soofer, Zarrin/Blansh,
Karabetian, Macar)** and local Assyrian grocers (catalog: Nineveh Imports, International Bazar, Neesan
Market). Produce local. Ishtar itself is part supplier — a vertically-integrated market+grill.

**Cost/margin pattern:** Lamb/halal protein lifts food cost; **food 30–35%, labor 36–40%**. Ishtar's
**market arm diversifies revenue** and improves blended margin/cushions the restaurant. *Benchmark, not actual.*

**Seasonality:** Community/religious calendar (Assyrian New York/Kha b-Nisan in April, holidays), catering for community events.

**Permit footprint:** County food-facility permit; **dual permit for Ishtar** (food facility + retail
market); ABC only if alcohol served.

---

## 12. Portuguese / Azorean — *Corvelos Portuguese Bakery*

**Produces/sells:** Authentic Azorean/Portuguese rolls and breads (e.g., **massa sovada** sweet bread,
**Portuguese rolls**) "served at area Portuguese celebrations." Turlock/Central Valley has a deep Azorean
dairy-and-festa community, so this bakery is **event-and-tradition-driven** with strong wholesale/community demand.

**Kitchen & equipment:** Deck/rack ovens, large mixers/dough handling for high-volume bread/roll runs,
proofers. Production bakery oriented to bulk celebration orders.

**Staffing:** Bakers + counter; small family operation scaling up for festa season.

**Key inputs & likely suppliers:** Flour, sugar, eggs, butter/lard, yeast — bulk via broadline/bakery
distributors; **Portuguese specialty ingredients** (linguiça/chouriço adjacency, specific flours) via
Portuguese/specialty importers serving the Valley's Azorean community. Eggs/dairy local (Azorean-rooted dairies abound locally).

**Cost/margin pattern:** Bakery profile — low ingredient COGS, **labor-driven**, thin per-unit margin;
**festa/event bulk orders** are the volume and margin engine. *Benchmark, not actual.*

**Seasonality:** **Heavily seasonal** — peaks around **Holy Ghost/Espírito Santo festas (late spring–summer)**
and Portuguese community celebrations; Easter (sweet bread) and Christmas spikes. This is the most
seasonality-concentrated business in the set.

**Permit footprint:** County food-facility/retail bakery permit; no ABC.

---

## 13. Ice Cream / Dessert — *House of Random (est. 2019), Oak Barrel Ice & Creamery*

**Produces/sells:** Ice cream/desserts; **Oak Barrel makes homemade ice cream** (in-house production),
House of Random offers ice cream + desserts.

**Kitchen & equipment:** For house-made: **batch freezer, pasteurizer/mix prep, blast/hardening freezer,
dipping cabinets**; for both: display freezers, prep for toppings/desserts. Oak Barrel's in-house
production is more capital- and skill-intensive than a scoop-only shop.

**Staffing:** 3–10; scoopers/counter, plus a production person for the house-made operation; seasonal part-time.

**Key inputs & likely suppliers:** **Dairy cream/milk — a Central Valley local strength** (this is prime
dairy country, a genuine input advantage), sugar, flavorings, inclusions, cones, cups — via broadline and
dairy/ingredient suppliers; toppings via **Restaurant Depot**/broadline.

**Cost/margin pattern:** Ice cream is a **high-gross-margin** treat format (low ingredient cost per scoop)
but **highly seasonal and weather-dependent**; net margin pressured by short peak season and idle winter
fixed costs. *Benchmark, not actual.*

**Seasonality:** **Most weather-sensitive format** — strong summer (Turlock summers are hot), weak winter;
revenue concentrated in ~5–6 warm months.

**Permit footprint:** County food-facility permit; house-made ice cream raises the food-safety tier (dairy
processing) vs. a scoop-only shop.

---

## Cross-Cutting Operational Findings

1. **Local-input advantage is real and format-spanning.** Turlock's location in California's premier
   dairy/produce belt gives cafes, creameries, pizzerias, panaderias, and Portuguese/Mexican operators
   genuine cost and freshness edges on **dairy, eggs, produce, and meat** vs. coastal metros. La Mo's
   explicitly markets local-farm sourcing.

2. **The ethnic-grocery ecosystem doubles as the specialty supply chain.** The catalog's
   `ethnic_community` businesses (carnicerias, tortillerias, Assyrian importers, Portuguese bakery) are
   not just retail — they are *inputs* to the restaurant sector. Ishtar literally is both market and grill.

3. **Permitting is the format's true cost gate, not food cost.** Mobile vendors face a stacked
   **$900 city + $69–$548 county + $114 license + mandatory commissary** structure; bar-forward concepts
   face the **Type 47 secondary-market license** wall (potentially six figures). These fixed/entry costs
   shape who can operate, more than ingredient prices do.

4. **Labor — often family labor — is the swing variable.** Across taquerias, trucks, panaderias, mariscos,
   and ethnic sit-downs, owner/family labor structurally suppresses the 36–40% full-service labor benchmark,
   which is a primary reason these independents survive on thin published margins.

5. **Seasonality clusters by culture, not just weather.** Panaderias (Día de Muertos/Reyes), Portuguese
   bakery (festas), mariscos (Lent), and ice cream (summer) each have distinct, predictable demand peaks
   that drive staffing and cash-flow planning.

6. **Consolidation watch.** Sysco's announced acquisition of Jetro/Restaurant Depot could shift
   cash-and-carry pricing that many of these small operators rely on
   ([Forbes](https://www.forbes.com/sites/phillempert/2026/03/31/syscos-29-billion-power-grab-what-the-jetro-restaurant-depot-deal-means-for-main-street-menus/)).

---

## Sources

- Sysco Central California (Modesto): https://www.sysco.com/contact/our-locations/central-california ·
  https://business.modchamber.org/list/member/sysco-food-services-of-central-california-1770
- Sysco–Restaurant Depot acquisition: https://www.forbes.com/sites/phillempert/2026/03/31/syscos-29-billion-power-grab-what-the-jetro-restaurant-depot-deal-means-for-main-street-menus/
- Tortilla/masa: https://www.thetortillaplug.com/ · https://romerosfood.com/distribution/ · https://www.mirancho.com/wholesale/ · https://masienda.com/collections/wholesale
- Mediterranean/Assyrian importers: https://karabetian.com/ · https://www.zarrinproducts.com/ · https://macarfoods.com/ · https://www.foodcodirectory.com/2023/12/ca-mediterranean-middle-eastern.html
- Seafood: https://puccifoods.com/ · https://sierragoldseafood.com/wholesale/ · https://catalinaop.com/wholesale/ · https://santamonicaseafood.com/
- Pan dulce labor/cost: https://www.kingarthurbaking.com/blog/2021/05/18/conchas-pan-dulce · https://www.lemon8-app.com/@janvillarre/7552231105049002551?region=us
- Permits/commissary (City of Turlock): https://www.cityofturlock.org/doingbusinessinturlock/permits/mobilefoodvendor.asp
- Stanislaus County Food Program / MFF / commissary: https://www.stancounty.com/er/environmentalhealth/food-program.shtm · https://www.stancounty.com/er/pdf/mff-commissary-application.pdf · https://www.stancounty.com/er/pdf/how-to-apply-mff-permit.pdf
- Food-truck permit cost range: https://www.newsbreak.com/the-modesto-bee-1592579/4171481250641-how-many-food-trucks-operate-in-stanislaus-county-what-it-takes-to-get-a-permit
- ABC license fees: https://www.abc.ca.gov/licensing/license-fees/ · https://permitplace.com/which-california-restaurant-liquor-license-is-right-for-you/
- Restaurant cost benchmarks: https://vantainsights.com/insights/restaurant-food-cost-percentage · https://pos.toasttab.com/blog/on-the-line/restaurant-payroll-percentage · https://leveragebuyinggroup.com/wp-content/uploads/2025/05/2025-Restaurateur-Benchmark-Guide-Final-5-24-2025.pdf
- Coffee benchmarks: https://vantainsights.com/insights/coffee-shop-profit-margins · https://blog.poolsixcoffeeroasters.com/setting-the-margins-on-your-coffee-menu/
- Source datasets: `analysis/food_dining/businesses.json` · `catalog/turlock_independent_business_catalog.json`

*Every per-business revenue reference in this document is a labeled industry benchmark estimate scaled by an
observable size signal — "estimate, not actual" — never a stated fact. Supplier-to-named-business links are
representative inference unless a cited source states the relationship.*
