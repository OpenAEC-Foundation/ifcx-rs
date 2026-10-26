# Eerste IFCX-referentieslice Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Lever een reproduceerbaar IFCX-projectpakket voor het entree-raamwerk en een modelgerichte OpenAEC-viewer met werkende domeinkoppelingen.

**Architecture:** Eén Rust-projectloader controleert manifest, checksums, imports, compositie en verwijzingen en wordt ook via WASM gebruikt. Een Rust-generator maakt de gecureerde casus, de losse IFCX-bestanden en de zip. Een kleine TypeScript/Three.js-viewer leest het pakket via dezelfde loader en presenteert model, tekening, rekengegevens, begroting en planning.

**Tech Stack:** Bestaande Rust-workspace, serde_json, sha2, zip, wasm-bindgen/wasm-pack; TypeScript, Vite en Three.js; Node-tests en een browserproef.

**Spec:** `docs/superpowers/specs/2026-10-26-openaec-ifcx-reference-design.md`

## Global Constraints

- Eén logisch project, meerdere IFCX-deelbestanden, met een zipcontainer voor complete overdracht.
- De modelweergave staat centraal; bediening, typografie en kleurgebruik volgen de gepinde OpenAEC-huisstijl.
- De originele IFC-, DWG-, PDF-, GEF- en beeldbestanden blijven buiten Git.
- Onbekende inhoud blijft behouden; onondersteunde verplichte betekenis wordt gerapporteerd.
- Bron, afleiding en aanvullende voorbeeldgegevens zijn onderscheiden; geen volledige FEM-herberekening of volledig gebouw claimen.
- Eerste slice: brongegevens van het entree-raamwerk (vijf knopen, drie analytische staven; fysieke horizontale ligger heeft twee analytische segmenten). Planning en kosten zijn expliciete voorbeelden.
- Deze uitvoering is door de gebruiker gestart. Uitvoering gebeurt inline; er volgt geen extra goedkeuringsronde voor dit plan.

## Review Focus

- Gewijzigde ZIP-inhoud, ontbrekende imports of foute referenties mogen niet als geldig project verschijnen: Task 2 tests.
- Gedeelde afhankelijkheden mogen niet dubbel worden toegepast en conflicten mogen niet stil overschrijven: Task 2 tests.
- Bronformules/metadata en onbekende attributen mogen niet verloren gaan bij parser-roundtrip: Task 1 tests.
- Dezelfde fysieke ligger heeft twee analytische segmenten; navigatie moet beide bereiken en hoeveelheden niet dubbel tellen: Task 3 en Task 4 tests.
- Een fout pakket of niet-ondersteunde geometrie mag geen leeg of misleidend succes opleveren: Task 4 browserproef en diagnostiek.

## Bestandsindeling

- `crates/ifcx-core/src/document.rs`, `query.rs`: nullable relaties en onbekende envelop-/nodevelden behouden.
- `crates/ifcx-core/tests/roundtrip.rs`: verliesvrije transportproeven.
- `crates/ifcx-project/{Cargo.toml,src/lib.rs,src/package.rs,src/graph.rs,tests/project.rs}`: pakketcontract en gedeelde projectloader.
- `crates/ifcx-reference/{Cargo.toml,src/lib.rs,src/main.rs,tests/reference.rs}`: generator, CLI en onafhankelijke casuscontroles.
- `crates/ifcx-wasm/{Cargo.toml,src/lib.rs}`: beperkte browserbindings voor pakketladen.
- `examples/reference-entree/`: gegenereerde, leesbare IFCX-uitvoer en gecontroleerde schema-/bronmetadata.
- `viewer/{package.json,package-lock.json,index.html,tsconfig.json,src/*,scripts/*}`: browserinterface en voorbereiding van WASM/voorbeeldassets.
- `docs/reference/{package-contract.md,slice-coverage.md}` en `README.md`: werkelijke dekking, starten, testen en beperkingen.

### Task 1: Behoud van IFCX-inhoud

**Interfaces:** `IfcxDocument`, `IfcxDataEntry` behouden hun bestaande API; children/inherits krijgen optionele doelen. Niet-herkende velden worden via serde flatten als JSON bewaard.

- [ ] Schrijf `roundtrip_retains_null_relationships_and_foreign_fields`: lees null-doelen en onbekende velden op header/import/document/node en vergelijk de opnieuw geserialiseerde JSON met de invoer.
- [ ] Run `cargo test -p ifcx-core --test roundtrip`; verwacht falen op huidige null-/onbekende-veldondersteuning.
- [ ] Voeg verliesvrij veldbehoud toe en pas constructors en child-query aan; bestaande standaard namespacehelpers blijven bruikbaar.
- [ ] Run `cargo test -p ifcx-core`; verwacht alle oude en nieuwe tests groen.
- [ ] Leg deze afgebakende wijziging vast.

### Task 2: Projectpakket en gedeelde loader

**Interfaces:** `load_files(BTreeMap<String, Vec<u8>>) -> Result<LoadedProject, ProjectError>`, `load_zip(&[u8])`, `encode_zip(&BTreeMap<String, Vec<u8>>)`, `validate_directory(&Path)`; `LoadedProject` bevat manifest, geordende document-ID's, samengestelde nodes, omgekeerde links en diagnostiek.

- [ ] Schrijf tests voor offline importvolgorde, gedeelde import, veranderd bestand, ontbrekend doel, dubbele pad-/documentidentiteit, onveilige pakketpaden, onbekende inhoud en ZIP/map-equivalentie.
- [ ] Run `cargo test -p ifcx-project`; verwacht ontbreken van de nieuwe library/functionaliteit.
- [ ] Implementeer versie 0.1 van het manifest: entrypoint, project-ID, bestanden met document-ID, profiel, pad en SHA-256. Werk in geheugen; pak geen archiefpaden uit.
- [ ] Implementeer importresolutie met cyclus-/ambiguïteitsdetectie, eenmalige afhankelijkheden en expliciete overlayprioriteit. Leg beperkte ondersteuning van inheritance vast; onondersteunde actieve inheritance wordt geen stil succes.
- [ ] Valideer verplichte children- en getypeerde domeinverwijzingen. Bewaar vreemde inhoud en onderscheid beperkte/deelinterpretatie van een geldig volledig pakket.
- [ ] Run de volledige kern-/pakkettestset en leg de wijziging vast.

### Task 3: Gecureerde entreecasus en generator

**Interfaces:** `reference_files() -> Result<BTreeMap<String, Vec<u8>>, ProjectError>` levert deterministische input voor dezelfde loader; CLI `ifcx-reference generate <directory>`, `package <directory> <zip>`, `validate <directory-or-zip>`.

- [ ] Schrijf onafhankelijke tests: bronknopen `(5200,-2700)`, `(0,0)`, `(2700,0)`, `(4700,0)`, `(5200,0)` mm; staven 2700/4700/500 mm; fysieke ligger 5200 mm; profielmassa 45,3 kg/m geeft 235,56 kg; kolom 7,0 kg/m × 2,7 m geeft 18,9 kg.
- [ ] Controleer model → tekening, berekening, analytische segmenten, hoeveelheid, begroting en activiteit en omgekeerd. Controleer dat bronbestanden niet worden meegeleverd.
- [ ] Run de tests rood, implementeer de generator en maak de bestanden groen.
- [ ] Gebruik bronprofielen S235, L300X100X15 en buis Ø76×4. Maak de vormweergave expliciet afgeleid; bewaar bronprofieleigenschappen apart van geometrische benadering.
- [ ] Genereer model, analytische gegevens, berekenings-/hoeveelhedengegevens, begroting, planning en een eenvoudige IFCCAD-tekening met een onafhankelijke koppellaag. Gebruik één fysieke ligger met twee analytische staven.
- [ ] Publiceer eigen schemasemantiek als experimenteel, met gepinde upstream-verwijzingen. Bevestig IFCCAD-readbaarheid waar haalbaar; anders vermeld de onuitgevoerde proef zonder volledige compatibiliteit te claimen.
- [ ] Run `cargo test -p ifcx-reference` en CLI-generatie/validatie/map-zipvergelijking; leg code en leesbare voorbeeldbestanden vast.

### Task 4: Modelgerichte OpenAEC-viewer

**Interfaces:** WASM `load_project_zip(bytes: &[u8]) -> Result<String, JsValue>` retourneert dezelfde geserialiseerde `LoadedProject`; browserprojectie maakt geen eigen compositie-algoritme.

- [ ] Schrijf tests voor identificatie van fysieke objecten, omgekeerde domeinkoppelingen, twee analytische segmenten bij één ligger, kostentotalen en expliciete onondersteunde vormdiagnostiek.
- [ ] Run tests rood en implementeer de projectie plus minimale WASM-export.
- [ ] Bouw een viewer in de officiële OpenAEC-stijl: centrale 3D-weergave, projectboom, selectie-inspector, domeinweergaven en lokaal openen van projectpakketten. Begin met een echte gegenereerde casus, zonder hardcoded mockupgegevens.
- [ ] Voeg selectie, focus, isoleren, tekeningselectie en terugnavigatie toe; toon herkomst/voorbeeldstatus en begrijpelijke laadfouten. Ontbrekende domeinen zijn herkenbaar afwezig.
- [ ] Pin frontendafhankelijkheden; bundel assets voor lokaal/offline gebruik. Gebruik OpenAEC-logo/tokens met bron- en licentievermelding.
- [ ] Run Rust-tests, WASM-build, frontendtests en productiebouw. Voer een browserproef uit voor model-/domeinselectie, pakket openen, fout pakket en smal scherm; controleer schermafbeeldingen.
- [ ] Werk README, pakketcontract en dekkingsrapport bij op basis van werkelijk aangetoonde resultaten. Leg de slice vast en laat de volledige wijziging onafhankelijk beoordelen.

## Overdracht

De eerste slice is voltooid wanneer een gebruiker de gebouwfragmenten kan zien en de echte IFCX-relaties kan volgen, en dezelfde pakketinhoud door CLI en browser gecontroleerd wordt. De volgende stap is uitbreiding van de gecureerde gebouwinhoud; dat gebeurt na beoordeling van deze werkende basis. Het complete Beatrix-geïnspireerde gebouw blijft het einddoel.
