# OpenAEC IFCX-referentiegebouw

Datum: 26 oktober 2026

Status: basisontwerp akkoord op 26 oktober 2026; viewer met model centraal en OpenAEC-huisstijl toegevoegd na bespreking van de schets. Uitvoeringsplan volgt.

Repository: `OpenAEC-Foundation/ifcx-rs`

## 1. Doel en gekozen richting

We ontwikkelen een omvangrijk, samenhangend gebouwvoorbeeld dat laat zien hoe OpenAEC IFCX voor zich ziet. Het gebouw vormt een inhoudelijke referentie voor applicaties, schema-ontwikkeling en gesprekken over IFC5. Het eindresultaat verbindt een compleet gebouwmodel met tekeningen, planning, begroting, constructieve berekeningen, een analytisch model en funderingsinformatie.

De gebruiker heeft gekozen voor één logisch gebouwproject met meerdere IFCX-deelbestanden, gezamenlijk overdraagbaar in een zipcontainer. De uitgepakte projectmap heeft dezelfde betekenis als het pakket. De onderdelen gebruiken dezelfde objectidentiteiten en expliciete verwijzingen. Eventuele domeinextensies zoals `.ifccad`, `.ifccalc` en `.ifcpp` zijn later herkenningsmiddelen voor inhoudelijke profielen, geen nieuwe onafhankelijke dataformaten.

Het openbare voorbeeld is geïnspireerd op het Beatrixgebouw. De oorspronkelijke stukken blijven lokaal buiten de repository. Gebouwgegevens mogen voor de uitwerking worden gebruikt; de gebruiker heeft geen opdracht gegeven om bronstukken of dit project te publiceren. Een afzonderlijke publicatiestap valt buiten deze ontwerpfase.

### Overwogen aanpakken

| Aanpak | Afweging |
| --- | --- |
| Eerst alle disciplines van het hele gebouw vullen en daarna afspraken consolideren | Snel veel inhoud; meer kans op verschillende identificatie-, meet- en verwijzingsafspraken en later herwerk. |
| Gebouwinhoud en gemeenschappelijke afspraken samen ontwikkelen | Concrete situaties toetsen elke schema-afspraak. Eerst een samenhangend deel, daarna uitbreiding tot de volledige afgesproken dekking. Aanbevolen werkvolgorde. |
| Eerst alle applicaties onderling koppelen | Direct aantoonbare uitwisseling, maar huidige applicatiebeperkingen zouden de inhoudelijke visie sterk beperken. |

De gekozen projectstructuur staat vast. De tweede werkvolgorde is onderdeel van het goedgekeurde basisontwerp: de volledige inhoudelijke scope wordt vooraf geïnventariseerd, een eerste deel verbindt alle disciplines, en daarna volgt uitbreiding naar het hele gebouw. Een klein demonstratiemodel vervangt het grote eindmodel niet.

## 2. Afbakening van IFC en OpenAEC

Er is geen eis om het complete IFC4-schema te implementeren. We nemen zo veel mogelijk relevante IFC4-betekenis mee die past bij de beschikbare IFCX-alpha, de OpenAEC-applicaties en hun ambities, en het referentiegebouw. Het bronmodel gebruikt IFC2x3; dat beperkt niet de nieuwe inhoudelijke scope.

Voor ieder onderwerp komt een dekkingsregister met:

- de betekenis die behouden of toegevoegd moet worden;
- bewijs uit een bron, een officiële alpha-uitwerking of een applicatiecontract;
- de gekozen IFCX-mapping, gebruikte schema's en versies;
- status: aangetoond in alpha, voorlopige mapping van IFC-inhoud, of OpenAEC-uitbreiding;
- status van de gebouwgegevens: brongegeven, afgeleid, of aanvullend voorbeeldgegeven;
- een klein gericht voorbeeld, een controle en een toepassing in het grote model;
- bekende beperkingen en concrete vragen voor vervolgonderzoek of buildingSMART.

Een onderwerp dat in IFC al bestaat wordt niet als nieuwe OpenAEC-semantiek gepresenteerd alleen omdat de alpha er nog geen uitgewerkt voorbeeld voor heeft. Evenmin is afwezigheid in de onderzochte alpha bewijs dat het onderwerp niet in IFC5 zal komen. Eigen mappings en uitbreidingen blijven expliciet voorlopige voorstellen.

## 3. Onderzochte basis en grenzen van het bewijs

### IFCX-alpha

Onderzocht checkpoint: `buildingSMART/IFC5-development` op `1a63082ada967c683cfacee2005f8f749c8e1b79`.

Het schema bevat de envelop `header`, `imports`, `schemas`, `data`, nodes met `path`, `children`, `inherits` en `attributes`, en onder meer Reference- en Blob-waardetypen. De aanwezigheid van een waardetype bewijst niet dat alle tooling het correct ondersteunt.

De onderzochte voorbeelden tonen onder meer bouwdelen, materiaal- en eigenschapsgegevens, classificaties, ruimtegrenzen, installatiesystemen, geotechniek en overlays. Compositie en imports zijn aanwezig. Importprioriteit en sommige verwijzings-, validatie- en verwijderingsgevallen zijn nog onderwerp van werk. We leggen daarom zowel de upstream-versie als elke gekozen afwijking en de bijbehorende test vast.

We gebruiken officiële voorbeelden om werking en compatibiliteit te onderzoeken. De openbare gebouwinhoud wordt uit de eigen casus en eigen aanvullingen gemaakt; upstream-voorbeeldmodellen worden niet als eigen gebouw herverpakt.

### Huidige Rust-repository

Onderzocht checkpoint: `ifcx-rs` op `35368d1a39cfff5bede70002376dbd529014180a`.

De kern heeft een documentmodel, JSON-IO, eenvoudige queries, samenvoegen van attributen en een beperkte diff. De bestaande 13 kerntests en één documentatievoorbeeld zijn in deze onderzoeksfase geslaagd. Schema-validatie, volledige compositie, geometrie-engine, editor en bindings zijn hiermee niet aangetoond; veel daarvan is nog een ontwerp of TODO. Ondersteuning voor null-verwijzingen, import-integriteit, behoud van onbekende gegevens en een volledige wijzigingssemantiek vraagt werk.

De bestaande architectuurbeschrijving is richtinggevend, maar beloftes over streaming, grote-bestandprestaties of een werkende OpenCascade-koppeling worden niet als gerealiseerd beschouwd.

### Beatrixmateriaal

De lokale verzameling bevat één IFC-model van ongeveer 77 MiB, 30 DWG's, 70 PDF's, zes GEF-bestanden en beeldmateriaal. Een eerste telling van STEP-records vond onder meer 470 wanden, 147 vloeren, 50 balken, acht kolommen, 58 materiaalopbouwsets en 63 systemen. Zulke recordtellingen zeggen nog niets over de inhoudelijke kwaliteit van ieder object.

Er zijn geen expliciete `IfcSpace`, `IfcElementQuantity`, `IfcTask`, `IfcCostItem` of `IfcStructuralAnalysisModel`-records gevonden. Dat betekent niet dat de benodigde informatie geheel ontbreekt: tekeningen, staten en rapporten kunnen aanvullende bronnen zijn. De constructieve bijlage bevat een funderingsberekening en een raamwerkberekening voor staal boven de entree. Daarmee zijn funderings- en FEM-voorbeelden inhoudelijk mogelijk; een directe machineleesbare modelimport is nog niet aangetoond.

## 4. Inhoudelijke scope

| Onderwerp | Beoogde inhoud | Aansluiting |
| --- | --- | --- |
| Project, locatie en ruimten | Projectstructuur, terrein, bouwlagen, ruimten, zones, begrenzingen, eenheden en coördinaten | Open BIM Studio; officiële alpha-voorbeelden voor gebouwstructuur, ruimtegrenzen en georeferentie |
| Bouwdelen, typen en relaties | Wanden, vloeren, daken, kozijnen, draagconstructie, typen, samenstellingen, openingen en fysieke aansluitingen | Bron-IFC, tekeningen en Open BIM Studio |
| Geometrie en ontwerpgegevens | Plaatsingen, herbruikbare geometrie, expliciete vormen, materiaalopbouwen en geselecteerde bewerkbare parameters | Alpha-geometrie en Open BIM Studio; geen belofte dat elke bronvorm parametrisch wordt gereconstrueerd |
| Eigenschappen en hoeveelheden | Classificatie, materiaaleigenschappen, prestaties, hoeveelheden, meetregels en herkomst | Alpha-eigenschappen, IFC-inhoud, model, staten en rapporten |
| Installaties | Componenten, systemen, verbindingen en bediende ruimten | Bronmodel, E/W-tekeningen, alpha-voorbeeld Domestic Hot Water |
| Tekeningen | IFCCAD-geometrie, lagen, blocks, Model/Paper, aanzichten, maatvoering en objectkoppelingen | Open CAD Studio en het gepinde IFCCAD-contract |
| Constructieve berekeningen | Uitgangspunten, invoer, formules, eenheden, bronversies, toetsingen en resultaten | Open Structural Calculations Studio en constructierapporten |
| Analytisch model | Knopen, staven, profielen, materialen, opleggingen/veren, belastingen, combinaties en resultaten | Open FEM Studio als beoogde toepassing; funderings- en raamwerkrapporten als bron |
| Fundering en ondergrond | Palen, belastinglocaties, sonderingen, funderingsadvies, draagvermogen, toewijzingen en varianten | Open Pile Plan Studio, GEF's, palenplan en rapporten |
| Planning | Werkpakketten, activiteiten, relaties, kalenders, middelen, voortgang en objecttoewijzingen | Open Planner Studio; voorbeeldplanning waar projectgegevens ontbreken |
| Begroting | Posten, hoeveelheden, middelen, tarieven, totalen en object-/werkpakketkoppelingen | Open Calc Studio; herleidbare voorbeeldprijzen en normen waar nodig |
| Dossier en wijzigingen | Ontwerpkeuzes, bronverwijzingen, rapporten, toestand, revisies en afhankelijkheden | Alle disciplines; geselecteerde aanvullende OpenAEC-schema's |

Onderwerpen krijgen inhoud zodra er een concreet geval, een betekenisvolle relatie en een controle is. Een nieuw domein wordt niet als voltooid gemeld op basis van alleen een lege schema-definitie. Overige ambities zoals energie, inspecties of beheer mogen later worden toegevoegd, maar zijn geen automatische eis voor de eerste complete gebouwuitwerking.

## 5. Eén project, meerdere documenten

### Projectmap en pakket

Voorgestelde eerste indeling:

```text
referentiegebouw/
  manifest.json
  project.ifcx
  modellen/bouwkundig.ifcx
  modellen/installaties.ifcx
  tekeningen/plattegronden.ifcx
  tekeningen/constructie.ifcx
  planning/uitvoering.ifcx
  begroting/gebouw.ifcx
  berekeningen/constructie.ifcx
  berekeningen/raamwerk.ifcx
  berekeningen/fundering.ifcx
  schemas/...
  assets/...
```

De zipvariant krijgt voorlopig `.ifcx.zip`. Dit is een OpenAEC-experiment en geen officiële IFCX-containerstandaard. `.ifcZIP` en `.ifcxb` krijgen geen nieuwe betekenis door dit voorstel. In de eerste versie blijven IFCX-payloads `.ifcx`; domeinextensies zijn optionele latere aliases voor geregistreerde profielen. Een bestaande `.ifcpp` of andere applicatie-extensie bewijst op zichzelf geen IFCX-compatibiliteit.

Het manifest is verpakkingsmetadata: pakketversie, projectidentiteit, entrypoint, inventaris van relatieve paden, document-ID's, profiel-/schemaversies, encoding en SHA-256-controlesommen. Gebouwinformatie blijft in IFCX. De definitie van het manifest zelf wordt vóór implementatie als openbaar contract uitgewerkt.

`project.ifcx` is het inhoudelijke startpunt en declareert de benodigde documenten via imports. Het manifest koppelt logische import-URI's aan de exacte pakketbestanden, ook voor gebundelde schema's. De loader moet dit expliciet ondersteunen; we nemen niet aan dat bestaande viewers ZIP of dit manifest al begrijpen.

### Resolutie en compositie

- Elke logische document-ID blijft stabiel bij verplaatsen of hernoemen van het fysieke bestand. De eerste versie gebruikt projectgebonden, versieerbare URI's; de exacte spelling is een implementatiekeuze die in het contract wordt vastgelegd.
- Voor het project wordt één globale ruimte van stabiele nodepaden afgesproken. Een object krijgt geen nieuwe identiteit doordat een ander deelbestand er gegevens aan toevoegt.
- Domeinrelaties gebruiken expliciete, getypeerde verwijzingen naar die paden. Bestandspaden, zichtbare namen en rijvolgorden zijn geen objectidentiteit.
- Imports beschrijven afhankelijkheden. Een laag die importeert wordt na zijn afhankelijkheden toegepast. De volgorde van imports bepaalt de volgorde van onafhankelijke lagen; elke logische laag wordt één keer toegepast. Dit is een OpenAEC-keuze die aansluit bij de upstream-discussie, niet een bewering dat de huidige alpha-loader al volledig zo werkt.
- Naast een volledige projectmodus is een deelmodus nodig: alleen een gekozen domeindocument met zijn transitieve afhankelijkheden laden. Relaties buiten de geladen selectie zijn aantoonbaar extern, geen verzonnen lege objecten. Volledige pakketvalidatie vereist resolutie van alle verplichte relaties.
- Een module declareert welke schema's nodig zijn om haar veilig te interpreteren. Onbekende optionele aanvullingen blijven behouden; een niet-ondersteund verplicht profiel levert een duidelijke beperking of afwijzing op.
- De complete zip moet offline oplosbaar zijn, inclusief gebruikte schemaversies en noodzakelijke assets. Een los domeinbestand mag afhankelijkheden hebben; zelfstandige overdracht levert het bestand met zijn afhankelijkheden.
- Twee domeinen mogen hetzelfde object aanvullen, maar hebben afgesproken eigenaarschap over de attributen. Afwijkende dubbele auteurswaarden geven een conflict. Expliciet als variant/overlay benoemde wijzigingen mogen op basis van de vastgelegde compositievolgorde overschrijven.

Het pakket verenigt dus inhoudelijk gekoppelde documenten. Het is geen verzameling onderling onafhankelijke exports en ook geen database of gelijktijdig bewerkprotocol. Een samengestelde enkelvoudige IFCX-export mag later als afgeleid hulpmiddel worden toegevoegd; de modulebestanden blijven de auteursbron.

## 6. Schema's, eenheden en afhankelijkheden

We hergebruiken aantoonbaar passende buildingSMART-, OpenUSD- en IFCCAD-schema's. Nieuwe afspraken worden in eigen OpenAEC-schema's geplaatst; officiële namespaces krijgen geen stilzwijgende alternatieve betekenis. Schema-ID's, exacte versies, onderbouwing en eventuele migratieregels staan in het project en de dekkingsdocumentatie.

De schema's beschrijven betekenis, typen, eenheden, verwijzingen en validatie. JSON-schema/IFCX-schema-validatie alleen is onvoldoende voor relaties, meetregels of rekenkundige samenhang; aanvullende inhoudelijke controles zijn nodig.

Coördinaten, plaatsingen en eenheden worden expliciet vastgelegd. De oude repositoryconventie 'mm voor alles' wordt niet blind toegepast op alle domeinen. De contracten onderscheiden lokale teken-/modelcoördinaten, geografische coördinaten, krachten, spanningen, tijd en geld. Er mag geen stilzwijgende factorconversie plaatsvinden.

Een fysiek bouwdeel, een analytische staaf, een begrotingspost en een activiteit zijn afzonderlijke objecten met expliciete relaties. Een balk kan bijvoorbeeld meerdere analytische staven of begrotingsregels hebben. Het geometrisch model wordt niet gelijkgesteld aan het analytisch model.

Afgeleide gegevens bewaren invoerverwijzingen, gebruikte document-/objectrevisies, methode of formule, eenheden en uitvoeringscontext. Een gewijzigde invoer markeert afhankelijke gegevens als verouderd. Het systeem belooft niet automatisch alle betrokken applicaties of rekenengines uit te voeren. Een resultaat krijgt pas de status actueel na een vastgelegde herberekening of expliciete beoordeling; alleen een nieuwe timestamp is onvoldoende.

## 7. Uitbreidbaar IFCCAD

Onderzocht checkpoint: `OpenAEC-Foundation/ifccad` op `2c7a4569804bff090b6cb1d894281bbe3f0b143c`.

Het bestaande contract accepteert vreemde IFCX-context naast CAD-inhoud en bewaart de bron apart. Onbekende CAD-attributen en onbekende velden in gesloten bestaande CAD-structuren worden afgewezen. De huidige documentencoder schrijft de CAD-projectie opnieuw; hij voegt bewerkingen niet automatisch terug in de volledige bron. Dit ontwerp belooft daarom geen bestaande verliesloze bewerkcyclus voor ons uitgebreide model.

De gebruiker geeft ruimte om IFCCAD verder uit te werken. Het voorstel is één rijk tekenmodel, met bestaande ondersteunde CAD-inhoud en optionele, eigen uitbreidingsattributen/nodes. Waar mogelijk worden verdere CAD-betekenissen later in overleg onderdeel van het actieve IFCCAD-contract; nieuwe veldnamen worden niet zonder aangepaste reader onder bestaande gesloten objecten gestopt.

Een reader moet het ondersteunde deel kunnen gebruiken en beperkingen rapporteren. Niet-ondersteunde informatie blijft behouden bij bewerken en opslaan, tenzij de gebruiker uitdrukkelijk een beperkte export kiest. Gegevens waarvan de betekenis afhangt van gewijzigde CAD-inhoud mogen daarbij niet stilzwijgend als geldig blijven staan. Hiervoor komt expliciet eigenaarschap, wijzigingsdetectie en een beleid voor afhankelijke aanvullingen; het bestaande onveranderde brondownloadpad alleen voldoet niet.

Voor nieuwe tekenobjecten kan een aanvullende grafische weergave in bestaande primitives worden gegenereerd. Die is herkenbaar als fallback, heeft een relatie naar het semantische object en wordt niet dubbel weergegeven met de primaire representatie. Als geen betekenisvolle fallback mogelijk is, meldt de reader wat ontbreekt.

Een beperkte compatibiliteitsuitvoer wordt zo nodig gegenereerd uit dezelfde bron, met een rapport van weggelaten betekenis. Er komen geen twee handmatig onderhouden gebouw- of tekenmodellen. Compatibiliteit wordt per gepinde readerversie vastgesteld; JSON/IFCX-behoud is geen automatische garantie voor CBOR, DXF of DWG.

## 8. Bronmateriaal, reproductie en publieke inhoud

De originele IFC-, DWG-, PDF-, GEF- en beeldbestanden blijven buiten Git. Ook volledige tekstextracties en lokale vertaalsleutels worden niet automatisch onderdeel van het openbare voorbeeld. Het referentiegebouw krijgt als voorstel een eigen projectidentiteit; gebruik van de naam Beatrix als openbare productnaam is nog niet besloten.

Een bronregister maakt onderscheid tussen rechtstreeks overgenomen feiten, afleidingen en voorbeeldwaarden. Ontbrekende planning, prijzen of modelobjecten mogen worden toegevoegd, maar worden niet als gegevens van het oorspronkelijke project gepresenteerd. Openbare herkomstinformatie mag documentsoort en methode noemen; persoonsgegevens, contactgegevens, bronbladen en commerciële stukken worden niet meegenomen enkel omdat ze lokaal beschikbaar zijn.

De publieke dataset, schema's, generator en controles moeten zonder toegang tot de oorspronkelijke stukken opnieuw hetzelfde pakket kunnen opleveren. Daarvoor wordt een geselecteerde, gecureerde invoerset met toegestane gebouwgegevens onderhouden. Lokale extractietooling voedt die set, maar is geen vereiste om het gepubliceerde voorbeeld te bouwen. Relevante identiteitstoewijzingen blijven stabiel bij hergeneratie; gegenereerde bestanden worden niet als tweede handmatige bron bewerkt.

De exacte datasetlicentie en eventuele afwijking van de repositorylicentie zijn publicatiebesluiten. Ze blokkeren lokaal ontwerp en uitwerking niet; een release mag geen onbesliste licentieclaim bevatten.

## 9. Module en ontwikkelgrenzen

De bestaande Rust-workspace is het vertrekpunt. Voorstel van de opsteller:

- `ifcx-core` verzorgt generieke IFCX-envelop, onbekende inhoud, resolutie/compositie en gedeelde controles, voor zover nodig voor het voorbeeld.
- Een aparte referentiemodule met een kleine CLI genereert, controleert en verpakt het gebouw. Gebouwspecifieke keuzes blijven buiten de generieke parser.
- Een lokale bronimport mag bestaande IFC-tooling gebruiken. Een complete nieuwe STEP-/IFC-parser of DWG-engine schrijven is geen doel van dit project. De implementer kiest onderhouden tooling na controle van beschikbaarheid en benodigde licenties.
- Schemacontracten, dekkingsregister, kleine voorbeelden en generatorinvoer worden samen met het grote model onderhouden.
- Bestaande bindings, webserver en OpenCascade-kernel worden alleen uitgebreid bij een concrete afhankelijkheid. De hieronder beschreven projectviewer is een expliciete oplevering. Het afronden van alle zeven oorspronkelijke crates is geen voorwaarde.
- Andere OpenAEC-repositories krijgen gerichte aansluitvoorbeelden en verwachte resultaten. Het ombouwen van alle applicaties of het wijzigen van IFCCAD-main is geen stilzwijgend onderdeel van de eerste implementatieopdracht.

Gebruik van IFCX voor het voorbeeld bewijst niet dat elk bestaand OpenAEC-bestandsformaat al dezelfde structuur gebruikt. Een eventuele JSON-LD-export van een applicatie wordt op inhoud en envelop gecontroleerd en zo nodig expliciet gemapt.

### Projectviewer: model centraal

Na vergelijking van twee interactieve indelingen kiest de gebruiker voor het gebouwmodel als centraal vertrekpunt. De viewer krijgt de OpenAEC-huisstijl. De schets gebruikte fictieve geometrie en gegevens en is geen uitgevoerde IFCX-reader of modelimport.

De eerste viewer is lezend en inspecterend. Het model staat centraal, met een project-/objectboom en eigenschappen van de selectie. Vanuit hetzelfde geselecteerde bouwdeel navigeert de gebruiker naar tekeningen, planning, begroting, berekeningen en funderingsinformatie. Omgekeerde navigatie vanuit een domein toont of markeert de betrokken gebouwobjecten. Objectidentiteit en selectie blijven behouden bij wisselen tussen weergaven.

De variant met samenhang centraal levert bruikbare ideeën voor een aanvullende relatieweergave bij de selectie, maar is niet de primaire applicatie-indeling. Niet alle informatie moet tegelijk zichtbaar zijn; het model en de actieve taak krijgen voorrang.

De viewer opent het IFCX-projectpakket lokaal en gebruikt dezelfde laad-, resolutie-, compositie- en validatiecontracten als de module. Voor de browser onderzoeken we een beperkte WASM-koppeling met de Rust-kern, zodat geen onafhankelijke tweede interpretatie van IFCX ontstaat. De precieze frontend-integratie wordt in het uitvoeringsplan vastgelegd.

De officiële IFC5-viewer is een inspiratie- en hergebruikbron voor Three.js-rendering, selectie, modelboom en lagen. De MIT-kennisgeving van overgenomen viewercode blijft behouden. De eigen viewer voegt de projectverpakking, domeinnavigatie en eerlijk gerapporteerde dekkingsbeperkingen toe. Het is geen belofte dat de bestaande alpha-viewer ons pakket of alle IFCCAD-uitbreidingen al opent.

De visuele basis is het OpenAEC-style-book en de daarbij horende ontwerpwaarden, componentafspraken en logo-assets. Onderzocht zijn Construction Amber als gericht accent, Deep Forge voor donkere vlakken/tekst en Space Grotesk, Inter en JetBrains Mono voor hun beschreven rollen. De groentinten en andere vrije stijldetails van de verkennende schets zijn geen vastgesteld productontwerp. De implementatie pint de gebruikte huisstijlversie en respecteert contrast, licenties en bronvermelding. Een omvangrijke ribbon of desktopverpakking is geen vereiste voor een eenvoudige webviewer; wel sluiten de relevante bediening, panelen en lichte/donkere weergave aan op de gedeelde OpenAEC-afspraken.

Een eerste bruikbare viewer toont het model, de objectselectie en een echte domeinkoppeling zodra het eerste samenhangende gebouwdeel beschikbaar is. Daarna breiden de domeinweergaven mee uit met de dataset. Voorbeeld- of nog niet berekende resultaten worden nooit als daadwerkelijke projecttoetsing gepresenteerd.

## 10. Werkvolgorde en opleveringen

1. **Contractbasis en broninventaris.** Het dekkingsregister uitwerken, bronnen inhoudelijk beoordelen, upstream/schema's pinnen en een minimale gevalideerde projectmap specificeren.
2. **Eerste complete samenhang.** Een constructief gedeelte van het gebouw verbinden met een tekening, berekening/analytisch model, hoeveelheden, begrotingsposten, planning en funderingsrelaties. De definitieve selectie volgt uit de bronnen; balk/fundering of het entree-raamwerk zijn concrete kandidaten.
3. **Volledig gebouw en domeinen.** Dezelfde afspraken toepassen op het complete gebouw, ruimten en ontbrekende inhoud reconstrueren, en de afgesproken domeinen inhoudelijk vullen. Onzekerheden en voorbeeldwaarden blijven herkenbaar.
4. **Uitwisseling en wijzigingsproef.** Volledige/partiële import, wijzigingen, onbekende inhoud, offline verpakking en compatibiliteitsuitvoer aantonen. Applicatiechecks worden afzonderlijk van schema- en modelchecks gerapporteerd.
5. **Referentie-uitgave voorbereiden.** Leesroute, kleine voorbeelden, dekkingsrapport, reproduceerbare bouwinstructies en onderbouwde ontwerpvragen opleveren. Publicatie of wijzigingen in andere repositories zijn afzonderlijke acties.

De kernopleveringen zijn het grote IFCX-project en pakket, de openbare schema's, gecureerde invoer/generator, de projectviewer, gerichte voorbeelden en controles, een beschrijving van de OpenAEC-keuzes en een eerlijk applicatiecompatibiliteitsoverzicht.

## 11. Acceptatie en verificatie

Een eerste volledige referentie-uitgave voldoet aan het volgende:

- Alle in het dekkingsregister voor die uitgave beloofde onderwerpen hebben ingevulde inhoud en gekoppelde voorbeelden; de grote casus is geen verzameling lege schema's.
- ZIP en uitgepakte map leveren dezelfde documenten, identiteiten, compositie en inhoudelijke resultaten op. Verplichte afhankelijkheden zijn offline beschikbaar; gewijzigde of ontbrekende pakketbestanden worden gedetecteerd.
- Alle verplichte relaties resolven in de volledige projectmodus. Herbenoemen/verplaatsen van pakketbestanden via een bijgewerkt manifest verandert de objectidentiteit niet.
- Kleine tests dekken onder meer importvolgorde, herhaalde imports, ontbrekende doelen, conflicten, null-markers, onbekende attributen, eenheden en behouden of gemelde informatie bij export.
- Voor een gekozen object is navigatie aantoonbaar van model naar tekening, rekenobject/resultaat, hoeveelheid, kostenpost en activiteit en terug, zonder een onjuiste één-op-éénrelatie af te dwingen.
- Een gewijzigde doorsnede of materiaalkeuze laat zien welke afgeleide gegevens verouderd zijn. Herberekening of beoordeling heeft een vastgelegde herkomst.
- Een bewerking van bekende CAD-inhoud behoudt onafhankelijke onbekende inhoud; gewijzigde afhankelijkheden worden herkend. Beperkte exports leveren een expliciet verliesrapport.
- Geometrische import wordt op geselecteerde onafhankelijke bronmaten, plaatsingen en objectaantallen gecontroleerd. Een succesvolle JSON-parse of een mooi beeld alleen bewijst geen correcte conversie.
- Generatie uit dezelfde gecureerde invoer en gepinde afhankelijkheden is reproduceerbaar. Veranderlijke tijdstempels worden vastgezet of expliciet buiten de bytevergelijking verantwoord.
- De publieke output bevat geen oorspronkelijke bronstukken. Een release-inventaris toont welke afgeleide inhoud en assets bewust zijn opgenomen.
- Het compatibiliteitsrapport onderscheidt contractvalidatie, eigen reader, gepinde externe reader en daadwerkelijk uitgevoerde applicatieproeven. Niet-uitgevoerde proeven blijven als zodanig gemarkeerd.
- De viewer opent dezelfde pakketinhoud als de module. Selectie in het model en navigatie naar en vanuit domeingegevens gebruiken dezelfde objectidentiteiten. De modelweergave staat centraal; bediening, typografie en kleurgebruik volgen de gepinde OpenAEC-huisstijl.

Er worden geen ongefundeerde prestatiedoelen, volledige IFC4-dekking, verliesloze alle-formaatconversie of werkende koppelingen met alle genoemde applicaties geclaimd. Grootte volgt uit het gebouw en zijn inhoud, niet uit kunstmatig vermenigvuldigen van objecten.

## 12. Keuzes die de implementer mag uitwerken

Binnen bovenstaande grenzen mag de implementer de namen van modules en bestanden, exacte profiel-/schema-URI's, lokale importtools, testorganisatie, fouttypen en deterministische serialisatie uitwerken. Daarbij worden keuzes in reguliere projectdocumentatie vastgelegd.

De eerste implementatieplanning moet concreet maken welke domeinen en externe applicatiechecks bij de eerste uitgave horen, welke nieuwe schemasemantiek daarvoor nodig is en hoe de publieke gecureerde invoer wordt opgebouwd. De complete scope hierboven is het einddoel; een tussenmijlpaal wordt niet als voltooid groot referentiegebouw gepresenteerd.

## 13. Bronnen

- [IFC5-development, gepind checkpoint](https://github.com/buildingSMART/IFC5-development/tree/1a63082ada967c683cfacee2005f8f749c8e1b79)
- [IFCX-envelop en waardetypen](https://github.com/buildingSMART/IFC5-development/blob/1a63082ada967c683cfacee2005f8f749c8e1b79/schema/ifcx.tsp)
- [Officiële voorbeelden en uitleg over meerdere bestanden](https://github.com/buildingSMART/IFC5-development/blob/1a63082ada967c683cfacee2005f8f749c8e1b79/Examples_FAQ.md)
- [Alpha-compositie](https://github.com/buildingSMART/IFC5-development/blob/1a63082ada967c683cfacee2005f8f749c8e1b79/src/ifcx-core/composition/compose.ts)
- [Discussie over importprioriteit, issue 144](https://github.com/buildingSMART/IFC5-development/issues/144)
- [IFCCAD, gepind checkpoint](https://github.com/OpenAEC-Foundation/ifccad/tree/2c7a4569804bff090b6cb1d894281bbe3f0b143c)
- [IFCCAD-contract](https://github.com/OpenAEC-Foundation/ifccad/blob/2c7a4569804bff090b6cb1d894281bbe3f0b143c/schemas/ifccad/experimental-contract-0.1.0.md)
- [IFCCAD-documentlevenscyclus en behoud van vreemde context](https://github.com/OpenAEC-Foundation/ifccad/blob/2c7a4569804bff090b6cb1d894281bbe3f0b143c/docs/experiments/ifccad-document-lifecycle.md)
- [OpenAEC IFCX-visie, als ambitie gelezen](https://open-aec.com/ifcx/)
- [Officiële IFC5-viewer](https://ifc5.technical.buildingsmart.org/viewer/)
- [IFC5-viewercode, gepind checkpoint](https://github.com/buildingSMART/IFC5-development/tree/1a63082ada967c683cfacee2005f8f749c8e1b79/src/viewer)
- [OpenAEC-style-book](https://github.com/OpenAEC-Foundation/OpenAEC-style-book)
- [OpenAEC Design System Reference](https://github.com/OpenAEC-Foundation/OpenAEC-style-book/blob/main/brandbook/DESIGN-SYSTEM.md)
- [Open BIM Studio](https://github.com/OpenAEC-Foundation/open-bim-studio)
- [Open Planner Studio](https://github.com/OpenAEC-Foundation/open-planner-studio)
- [Open Calc Studio](https://github.com/OpenAEC-Foundation/open-calc-studio)
- [Open Structural Calculation Studio](https://github.com/OpenAEC-Foundation/Open-Structural-Calculation-Studio)
- [Eerdere Open Calculations Studio-documentatie](https://github.com/OpenAEC-Foundation/Open-Calculations-Studio)
- [Open Pile Plan Studio](https://github.com/OpenAEC-Foundation/pile-plan-studio)
- Open FEM Studio: beoogde toepassing volgens gebruiker en OpenAEC-overzicht; de daar gelinkte repository was bij onderzoek niet openbaar bereikbaar. Het actuele bestandscontract is niet bevestigd.
- Lokale Beatrixbronnen: IFC-model, tekeningen, constructierapport met bijlage en sonderingen; oorspronkelijke stukken worden niet in deze repository opgenomen.

Applicatieverwijzingen zijn op 26 oktober 2026 onderzocht en zijn nog niet als uitvoerbare compatibiliteitscheck gepind. Dat gebeurt per werkelijk uitgevoerde integratieproef.
