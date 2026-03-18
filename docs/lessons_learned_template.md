# Lessons Learned — [Projectnaam]

> Dit bestand documenteert lessons learned specifiek voor dit project.
> Lees ook: `../lessons_learned_global.md` voor cross-project patronen.
>
> Laatst bijgewerkt: [datum]

---

## Ontwerpbeslissingen

<!-- Architectuurkeuzes die achteraf goed of slecht uitpakten -->

| Beslissing | Uitkomst | Les |
|-----------|----------|-----|
| _Voorbeeld: Schema-first aanpak_ | _Goed: voorkomt API/frontend divergentie_ | _Altijd schema updaten vóór implementatie_ |

---

## Technische valkuilen

<!-- Bugs, workarounds, en onverwacht gedrag -->

| Probleem | Oorzaak | Oplossing | Voorkomen |
|----------|---------|-----------|-----------|
| _Voorbeeld: Transaction crash_ | _Dubbele transaction (MCP + eigen code)_ | _use_transaction=True in MCP_ | _Nooit eigen Transaction bij MCP tools_ |

---

## Procesfouten

<!-- Werkwijze-issues: volgorde, communicatie, aannames -->

- 

---

## Communicatie & Afstemming

<!-- Misverstanden met opdrachtgevers, architecten, aannemers -->

- 

---

## Wat ging goed

<!-- Bewust vastleggen wat werkte — herhaal dit in toekomstige projecten -->

- 

---

## Openstaande risico's

<!-- Bekende issues die nog niet opgelost zijn -->

| Risico | Impact | Mitigatie |
|--------|--------|-----------|
| | | |

---

*Voeg nieuwe items toe zodra ze zich voordoen. Review dit bestand aan het einde van elke significante sessie.*
