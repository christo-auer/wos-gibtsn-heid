![Logo von Wos Gibstn Heid](resources/logo.png)

# mcp-wos-gibtsn-heid

Ein Claude MCP (Model Context Protocol) Tool, das einfachen Zugriff auf die Mensaspeisepläne des [Studierendenwerks Niederbayern/Ostbayern](https:://www.stwno.de) ermöglicht:

- unterstützt alle Mensen und Cafeterien des StWNOs
- Anzeigen von Speiseplänen für die aktuelle Woche, kommende Wochen oder eine beliebige Kalenderwoche
- ermöglicht das Filtern nach Inhaltsstoffen und Allergenen
- berücksichtigt bevorzugte Kennzeichnungen (vegetarisch, vegan, Rind, etc.)
- nutzbar von allen MCP clients (aktuell getestet mit Claude Desktop und OpenCode)

Der Name *wos gibts'n heid* ist bayerischer Dialekt für "Was gibt es heute?".


## Haftungsausschluss

**WICHTIG:**
- Dieses Projekt ist in keiner Weise mit dem Studierendenwerk Niederbayern/Ostbayern (StWNO) oder einem seiner Partner verbunden.
- LLMs und dieses Tool können Fehler machen. Benutzern wird dringend empfohlen, Allergene, Inhaltsstoffe und Kennzeichnungen selbst zu überprüfen und bei Bedarf das StWNO-Personal zu fragen.
- Die von diesem Tool bereitgestellten Informationen sollten nicht allein für Ernährungseinschränkungen oder Allergiebedenken herangezogen werden.

## Installation

### Claude Desktop/Claude Code

Um dieses Tool mit Claude zu verwenden:

1. Laden Sie die neueste Version der Binärdatei von der [GitHub-Releases-Seite](https://github.com/christo-auer/wos-gibtsn-heid/releases) herunter
2. Kopieren Sie die ausführbare Binärdatei an einen geeigneten Ort.
3. Konfigurieren Sie Claude Desktop/Claude Code zur Ausführung dieser Binärdatei als externes Tool:
   - Öffnen Sie die Claude Desktop-Einstellungen
   - Navigieren Sie zum MCP/Tools-Bereich
   - Fügen Sie ein neues externes Tool hinzu, das auf Ihre heruntergeladene Binärdatei verweist
   - stellen Sie die Kommandozeilenparameter entsprechend der Dokumentation (s. unten) ein
   - Speichern Sie die Konfiguration

Weitere Informationen über das Model Context Protocol finden Sie in der [Claude MCP-Dokumentation](https://docs.anthropic.com/claude/docs/model-context-protocol).

### MCPB Paket

Dieses Tool ist auch als MCPB (MCP Bundle) Paket verfügbar für die einfache Ein-Klick-Installation in unterstützten MCP-Clients wie Claude Desktop:

1. Laden Sie das neueste `.mcpb` Paket von der [GitHub-Releases-Seite](https://github.com/christo-auer/mcp-wos-gibtsn-heid/releases) herunter
2. Öffnen Sie die Datei mit Claude Desktop - es wird automatisch ein Installationsdialog angezeigt
3. Konfigurieren Sie Ihre Präferenzen (Standardort, zu vermeidende Allergene, bevorzugte Kennzeichnungen) über die benutzerfreundliche Oberfläche
4. Das Tool ist sofort einsatzbereit

Das MCPB-Paket enthält alle plattformspezifischen Binärdateien und eine benutzerfreundliche Konfigurationsoberfläche für alle verfügbaren Optionen.

### Aus dem Quellcode bauen

Wenn Sie aus dem Quellcode bauen möchten:

1. Stellen Sie sicher, dass Rust und Cargo installiert sind
2. Klonen Sie das Repository
3. Bauen Sie mit Cargo:
   ```
   cargo build --release
   ```
4. Die Binärdatei ist verfügbar unter `target/release/mcp-wos-gibtsn-heid`

## Verwendung

### CLI-Parameter

```
mcp-wos-gibtsn-heid [OPTIONEN]
```

#### Optionen:

| Option | Beschreibung | Mögliche Werte |
|--------|-------------|----------------|
| `--location <LOCATION>` | Standardort, für den der Speiseplan abgerufen werden soll | Siehe [Standorte](#standorte) |
| `--avoid-allergens <LIST-OF-ALLERGENES>` | Durch Kommas getrennte Liste von zu vermeidenden Allergenen | Beispiel: `AA,E,L` (Siehe [Allergene](#allergene)) |
| `--avoid-ingredients <LIST-OF-INGREDIENTS>` | Durch Kommas getrennte Liste von zu vermeidenden Inhaltsstoffen | Beispiel: `1,4,2` (Siehe [Inhaltsstoffe](#inhaltsstoffe)) |
| `--preferred-indicators <LIST-OF-INDICATORS>` | Durch Kommas getrennte Liste bevorzugter Kennzeichnungen | Beispiel: `V,VG` (Siehe [Kennzeichnungen](#kennzeichnungen)) |
| `--list-locations` | Verfügbare Standorte auflisten | |
| `--list-allergens` | Verfügbare Allergene auflisten | |
| `--list-ingredients` | Verfügbare Inhaltsstoffe auflisten | |
| `--list-indicators` | Verfügbare Kennzeichnungen auflisten | |
| `--help` | Hilfe anzeigen | |
| `--version` | Version anzeigen | |

#### Beispiel:

```bash
# Einstellungen für vegetarisches Essen an der OTH Regensburg (Mittagsgerichte), unter Vermeidung von Allergenen aus Eiern und Laktose
mcp-wos-gibtsn-heid --location HS-R-tag --preferred-indicators V,VG --avoid-allergens C,G

# Alle verfügbaren Standorte auflisten
mcp-wos-gibtsn-heid --list-locations
```

## Standorte

Die folgenden Standorte werden unterstützt:

| Standort-Code              | Beschreibung                                     |
|--------------              |-------------                                     |
| `HS-LA`                    | Hochschule Landshut Mensa                        |
| `HS-LA-Cafeteria`          | Hochschule Landshut Cafeteria                    |
| `UNI-R`                    | Universität Regensburg Mensa                     |
| `UNI-R-Gs`                 | Universität Regensburg Mensa - Gästesaal         |
| `Cafeteria-PT`             | Universität Regensburg Cafeteria PT              |
| `Cafeteria-Chemie`         | Universität Regensburg Cafeteria Chemie          |
| `Cafeteria-Sammelgebaeude` | Universität Regensburg Cafeteria Sammelgebäude   |
| `Cafeteria-Sport`          | Universität Regensburg Cafeteria Sport           |
| `HS-R-tag`                 | OTH Regensburg Mensa Seybothstraße (Mittags)     |
| `HS-R-abend`               | OTH Regensburg Mensa Seybothstraße (Abends)      |
| `Cafeteria-Pruefening`     | OTH Regensburg Mensa Prüfeningerstraße (Mittags) |
| `UNI-P`                    | Universität Passau Mensa                         |
| `Cafeteria-Nikolakloster`  | Universität Passau Cafeteria Nikolakloster       |
| `HS-DEG`                   | TH Deggendorf Mensa                              |
| `TH-DEG-Cham`              | TH Deggendorf-Cham                               |
| `HS-PAN`                   | European Campus Pfarrkirchen                     |
| `HS-SR`                    | TUM Campus Straubing                             |

## Allergene, Inhaltsstoffe und Kennzeichnungen

### Allergene

| Code | Beschreibung               |
|------|-------------               |
| `AA` | Weizengluten               |
| `AB` | Roggengluten               |
| `AC` | Gerstengluten              |
| `AD` | Hafergluten                |
| `AE` | Dinkelgluten               |
| `AF` | Kamutgluten                |
| `B`  | Krebstiere                 |
| `C`  | Eier                       |
| `D`  | Fisch                      |
| `E`  | Erdnüsse                   |
| `F`  | Soja                       |
| `G`  | Milch und Milchprodukte    |
| `HA` | Mandel                     |
| `HB` | Haselnuss                  |
| `HC` | Walnuss                    |
| `HD` | Cashew                     |
| `HE` | Pecannuss                  |
| `HF` | Paranuss                   |
| `HG` | Pistazie                   |
| `HH` | Macadamianuss              |
| `HI` | Queenslandnuss             |
| `I`  | Sellerie                   |
| `J`  | Senf                       |
| `K`  | Sesamsamen                 |
| `L`  | Schwefeldioxid und Sulfite |
| `M`  | Lupinen                    |
| `N`  | Weichtiere                 |
| `O`  | Nitrat                     |
| `P`  | Nitritpökelsalz            |

### Inhaltsstoffe

| Code | Beschreibung                                          |
|------|-------------                                          |
| `1`  | mit Farbstoff                                         |
| `2`  | mit Konservierungsstoff                               |
| `3`  | mit Antioxidationsmittel                              |
| `4`  | mit Geschmacksverstärker                              |
| `5`  | geschwefelt                                           |
| `6`  | geschwärzt                                            |
| `7`  | gewachst                                              |
| `8`  | mit Phosphat                                          |
| `9`  | mit Süssungsmittel Saccharin                          |
| `10` | mit Süssungsmittel Aspartam, enth. Phenylalaninquelle |
| `11` | mit Süssungsmittel Cyclamat                           |
| `12` | mit Süssungsmittel Acesulfam                          |
| `13` | chininhaltig                                          |
| `14` | coffeinhaltig                                         |
| `16` | enthält Sulfite                                       |
| `17` | enthält Phenylalanin                                  |

### Kennzeichnungen

| Code  | Beschreibung                        |
|------ |-------------                        |
| `V`   | vegetarisch                         |
| `VG`  | vegan                               |
| `S`   | Schwein                             |
| `R`   | Rind                                |
| `G`   | Geflügel                            |
| `F`   | Fisch                               |
| `A`   | Alkohol                             |
| `B`   | bio                                 |
| `PHD` | Klimateller (planetary health diet) |

## Lizenz

MIT-Lizenz

# 

A Claude MCP (Model Context Protocol) tool that provides easy access to the canteen menus of the Studierendenwerk Niederbayern/Ostbayern. This tool helps students and university employees check what's on the menu at various universities and colleges in Lower Bavaria and Upper Palatinate, including institutions in Regensburg, Passau, Deggendorf, Landshut, and other locations.

The name "gibts'n heid" is Bavarian dialect for "What's available today?" (Was gibt es heute?).

## Features

- Access menus from multiple university canteens and cafeterias
- Filter dishes by allergens and ingredients
- Find vegetarian, vegan, and other specially marked dishes
- View menus for current week or upcoming weeks
- Usable through Claude Desktop or as a standalone CLI tool

## Disclaimer

**IMPORTANT:**
- This project is in no way affiliated with Studierendenwerk Niederbayern/Ostbayern (StWNO) or any of its partners.
- LLMs and this tool can make errors. Users are strongly advised to check for allergens, ingredients, and indicators for themselves and ask the StWNO personnel if needed.
- The information provided by this tool should not be solely relied upon for dietary restrictions or allergy concerns.

## Installation

### Claude Desktop/Claude Code

To use this tool with Claude:

1. Download the latest binary from the [GitHub releases page](https://github.com/yourusername/mcp-wos-gibtsn-heid/releases)
2. Make the binary executable:
   ```
   chmod +x ./mcp-wos-gibtsn-heid
   ```
3. Configure Claude Desktop/Claude Code to execute this binary as an external tool:
   - Open Claude Desktop settings
   - Navigate to the MCP/Tools section
   - Add a new external tool pointing to your downloaded binary
   - Save the configuration

For more information about the Model Context Protocol, see the [Claude MCP documentation](https://docs.anthropic.com/claude/docs/model-context-protocol).

### MCPB Package

This tool is also available as an MCPB (MCP Bundle) package for easy one-click installation in supported MCP clients like Claude Desktop:

1. Download the latest `.mcpb` package from the [GitHub releases page](https://github.com/christo-auer/mcp-wos-gibtsn-heid/releases)
2. Open the file with Claude Desktop - it will automatically show an installation dialog
3. Configure your preferences (default location, allergens to avoid, preferred indicators) through the user-friendly interface
4. The tool is ready to use immediately

The MCPB package includes all platform-specific binaries and provides a user-friendly configuration interface for all available options.

### Standalone CLI

To install the standalone CLI version:

1. Download the latest release from the [GitHub releases page](https://github.com/christo-auer/mcp-wos-gibtsn-heid/releases)
2. Extract the archive
3. Make the binary executable:
   ```
   chmod +x ./mcp-wos-gibtsn-heid
   ```
4. Optionally, move the binary to a directory in your PATH:
   ```
   sudo mv ./mcp-wos-gibtsn-heid /usr/local/bin/
   ```

### Building from Source

If you prefer to build from source:

1. Ensure you have Rust and Cargo installed
2. Clone the repository
3. Build with Cargo:
   ```
   cargo build --release
   ```
4. The binary will be available in `target/release/mcp-wos-gibtsn-heid`

## Usage

### CLI Parameters

```
mcp-wos-gibtsn-heid [OPTIONS]
```

#### Options:

| Option | Description | Possible Values |
|--------|-------------|----------------|
| `--location <LOCATION>` | Default location for which to fetch the menu | See [Locations](#locations) |
| `--avoid-allergens <LIST-OF-ALLERGENES>` | Comma-separated list of allergens to avoid | Example: `AA,E,L` (See [Allergens](#allergens)) |
| `--avoid-ingredients <LIST-OF-INGREDIENTS>` | Comma-separated list of ingredients to avoid | Example: `1,4,2` (See [Ingredients](#ingredients)) |
| `--preferred-indicators <LIST-OF-INDICATORS>` | Comma-separated list of preferred indicators | Example: `V,VG` (See [Indicators](#indicators)) |
| `--list-locations` | List available locations | |
| `--list-allergens` | List available allergens | |
| `--list-ingredients` | List available ingredients | |
| `--list-indicators` | List available indicators | |
| `--help` | Print help | |
| `--version` | Print version | |

#### Example:

```bash
# Set preferences for vegetarian food, avoiding allergens from eggs and lactose
mcp-wos-gibtsn-heid --location HS-R-tag --preferred-indicators V,VG --avoid-allergens C,G

# List all available locations
mcp-wos-gibtsn-heid --list-locations
```

## Locations

The following locations are supported:

| Location Code | Description |
|--------------|-------------|
| `HS-LA` | Hochschule Landshut Mensa |
| `HS-LA-Cafeteria` | Hochschule Landshut Cafeteria |
| `UNI-R` | Universität Regensburg Mensa |
| `UNI-R-Gs` | Universität Regensburg Mensa - Gästesaal |
| `Cafeteria-PT` | Universität Regensburg Cafeteria PT |
| `Cafeteria-Chemie` | Universität Regensburg Cafeteria Chemie |
| `Cafeteria-Sammelgebaeude` | Universität Regensburg Cafeteria Sammelgebäude |
| `Cafeteria-Sport` | Universität Regensburg Cafeteria Sport |
| `HS-R-tag` | OTH Regensburg Mensa Seybothstraße (Mittags) |
| `HS-R-abend` | OTH Regensburg Mensa Seybothstraße (Abends) |
| `Cafeteria-Pruefening` | OTH Regensburg Mensa Prüfeningerstraße (Mittags) |
| `UNI-P` | Universität Passau Mensa |
| `Cafeteria-Nikolakloster` | Universität Passau Cafeteria Nikolakloster |
| `HS-DEG` | TH Deggendorf Mensa |
| `TH-DEG-Cham` | TH Deggendorf-Cham |
| `HS-PAN` | European Campus Pfarrkirchen |
| `HS-SR` | TUM Campus Straubing |

## Allergens, Ingredients, and Indicators

### Allergens

| Code | Description |
|------|-------------|
| `AA` | Weizengluten |
| `AB` | Roggengluten |
| `AC` | Gerstengluten |
| `AD` | Hafergluten |
| `AE` | Dinkelgluten |
| `AF` | Kamutgluten |
| `B` | Krebstiere |
| `C` | Eier |
| `D` | Fisch |
| `E` | Erdnüsse |
| `F` | Soja |
| `G` | Milch und Milchprodukte |
| `HA` | Mandel |
| `HB` | Haselnuss |
| `HC` | Walnuss |
| `HD` | Cashew |
| `HE` | Pecannuss |
| `HF` | Paranuss |
| `HG` | Pistazie |
| `HH` | Macadamianuss |
| `HI` | Queenslandnuss |
| `I` | Sellerie |
| `J` | Senf |
| `K` | Sesamsamen |
| `L` | Schwefeldioxid und Sulfite |
| `M` | Lupinen |
| `N` | Weichtiere |
| `O` | Nitrat |
| `P` | Nitritpökelsalz |

### Ingredients

| Code | Description |
|------|-------------|
| `1` | mit Farbstoff |
| `2` | mit Konservierungsstoff |
| `3` | mit Antioxidationsmittel |
| `4` | mit Geschmacksverstärker |
| `5` | geschwefelt |
| `6` | geschwärzt |
| `7` | gewachst |
| `8` | mit Phosphat |
| `9` | mit Süssungsmittel Saccharin |
| `10` | mit Süssungsmittel Aspartam, enth. Phenylalaninquelle |
| `11` | mit Süssungsmittel Cyclamat |
| `12` | mit Süssungsmittel Acesulfam |
| `13` | chininhaltig |
| `14` | coffeinhaltig |
| `16` | enthält Sulfite |
| `17` | enthält Phenylalanin |

### Indicators

| Code | Description |
|------|-------------|
| `V` | vegetarisch |
| `VG` | vegan |
| `S` | Schwein |
| `R` | Rind |
| `G` | Geflügel |
| `F` | Fisch |
| `A` | Alkohol |
| `B` | bio |
| `PHD` | Klimateller (planetary health diet) |

## License

MIT License

Copyright (c) 2025 

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
